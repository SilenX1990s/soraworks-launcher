//! Xbox Live -> XSTS -> Minecraft Services token chain, plus profile lookup.
//!
//! Given a Microsoft access token, this produces a Minecraft access token and
//! the player's profile (UUID + name). Ownership is verified implicitly: an
//! account that does not own Minecraft: Java Edition has no profile.

use crate::error::{Error, Result};
use crate::http;
use serde::Deserialize;

pub struct McAuth {
    pub access_token: String,
    pub expires_in: u64,
    pub uuid: String,
    pub name: String,
}

#[derive(Deserialize)]
struct XboxResponse {
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: DisplayClaims,
}

#[derive(Deserialize)]
struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Deserialize)]
struct Xui {
    uhs: String,
}

#[derive(Deserialize)]
struct XstsError {
    #[serde(rename = "XErr", default)]
    xerr: Option<u64>,
}

#[derive(Deserialize)]
struct McLoginResponse {
    access_token: String,
    #[serde(default)]
    expires_in: u64,
}

#[derive(Deserialize)]
struct Profile {
    id: String,
    name: String,
}

/// Full chain from a Microsoft access token to a Minecraft session.
pub async fn authenticate(ms_access_token: &str) -> Result<McAuth> {
    // 1. Xbox Live user token.
    let xbl = http::client()
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&serde_json::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={ms_access_token}")
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .error_for_status()?
        .json::<XboxResponse>()
        .await?;

    let xbl_token = xbl.token;

    // 2. XSTS token for Minecraft services.
    let xsts_resp = http::client()
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&serde_json::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?;

    if xsts_resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        let err: XstsError = xsts_resp.json().await.unwrap_or(XstsError { xerr: None });
        return Err(match err.xerr {
            Some(2148916233) => Error::AuthNoXbox,
            Some(2148916235) => Error::AuthRegion,
            Some(2148916238) => Error::AuthChild,
            _ => Error::Auth("xsts".into()),
        });
    }

    let xsts = xsts_resp.error_for_status()?.json::<XboxResponse>().await?;
    let xsts_token = xsts.token;
    let uhs = xsts
        .display_claims
        .xui
        .first()
        .map(|x| x.uhs.clone())
        .ok_or_else(|| Error::Auth("no user hash".into()))?;

    // 3. Minecraft Services login.
    let mc = http::client()
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&serde_json::json!({
            "identityToken": format!("XBL3.0 x={uhs};{xsts_token}")
        }))
        .send()
        .await?
        .error_for_status()?
        .json::<McLoginResponse>()
        .await?;

    // 4. Profile (and implicit ownership check).
    let profile_resp = http::client()
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(&mc.access_token)
        .send()
        .await?;

    if profile_resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::AuthNoGame);
    }
    let profile = profile_resp.error_for_status()?.json::<Profile>().await?;

    Ok(McAuth {
        access_token: mc.access_token,
        expires_in: if mc.expires_in == 0 { 86_400 } else { mc.expires_in },
        uuid: profile.id,
        name: profile.name,
    })
}
