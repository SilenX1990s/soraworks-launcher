//! Microsoft identity platform OAuth 2.0 for the "consumers" tenant
//! (personal Microsoft accounts only), using the public-client flows the
//! launcher is registered for: Authorization Code + PKCE and Device Code.

use crate::error::{Error, Result};
use crate::http;
use base64::Engine;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::time::Duration;

/// Azure application (client) id. This is a public client id — not a secret —
/// and is meant to ship inside the app.
pub const CLIENT_ID: &str = "3178009e-6f05-4d28-afd4-dbe26974f5f4";

const AUTHORIZE_URL: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const DEVICE_CODE_URL: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const SCOPE: &str = "XboxLive.signin offline_access";

/// A Microsoft OAuth token set.
pub struct MsToken {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

impl TokenResponse {
    fn into_token(self) -> Result<MsToken> {
        if let Some(err) = self.error {
            if err == "authorization_declined" || err == "access_denied" {
                return Err(Error::AuthCancelled);
            }
            return Err(Error::Auth(self.error_description.unwrap_or(err)));
        }
        Ok(MsToken {
            access_token: self
                .access_token
                .ok_or_else(|| Error::Auth("no access_token".into()))?,
            refresh_token: self.refresh_token.unwrap_or_default(),
        })
    }
}

// ---- PKCE helpers ----

fn random_string(len: usize) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let mut rng = rand::thread_rng();
    (0..len).map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char).collect()
}

fn pkce_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

// ---- Authorization Code + PKCE (interactive browser login) ----

/// Run the full interactive login: start a loopback server, open the browser,
/// wait for the redirect, and exchange the code for tokens.
pub async fn login_pkce() -> Result<MsToken> {
    let verifier = random_string(64);
    let challenge = pkce_challenge(&verifier);
    let state = random_string(24);

    // Bind a loopback server on a random port for the redirect.
    let server = tiny_http::Server::http("127.0.0.1:0")
        .map_err(|e| Error::other(format!("loopback server: {e}")))?;
    let port = match server.server_addr() {
        tiny_http::ListenAddr::IP(addr) => addr.port(),
        #[allow(unreachable_patterns)]
        _ => return Err(Error::other("unexpected listen addr")),
    };
    let redirect_uri = format!("http://localhost:{port}");

    let auth_url = url::Url::parse_with_params(
        AUTHORIZE_URL,
        &[
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("redirect_uri", &redirect_uri),
            ("response_mode", "query"),
            ("scope", SCOPE),
            ("state", &state),
            ("code_challenge", &challenge),
            ("code_challenge_method", "S256"),
            ("prompt", "select_account"),
        ],
    )
    .map_err(|e| Error::other(format!("auth url: {e}")))?;

    // Open the system browser for the user to sign in.
    let _ = open::that(auth_url.as_str());

    // Wait (up to 5 minutes) for the browser to redirect back with a code.
    let expected_state = state.clone();
    let code = tokio::task::spawn_blocking(move || wait_for_code(server, &expected_state))
        .await
        .map_err(|e| Error::other(format!("redirect join: {e}")))??;

    // Exchange the authorization code for tokens.
    let params = [
        ("client_id", CLIENT_ID),
        ("grant_type", "authorization_code"),
        ("code", code.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("code_verifier", verifier.as_str()),
        ("scope", SCOPE),
    ];
    let resp: TokenResponse = http::client()
        .post(TOKEN_URL)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    resp.into_token()
}

/// Block on the loopback server until the OAuth redirect arrives.
fn wait_for_code(server: tiny_http::Server, expected_state: &str) -> Result<String> {
    let deadline = std::time::Instant::now() + Duration::from_secs(300);
    loop {
        if std::time::Instant::now() >= deadline {
            return Err(Error::AuthCancelled);
        }
        let request = match server.recv_timeout(Duration::from_secs(1)) {
            Ok(Some(req)) => req,
            Ok(None) => continue,
            Err(_) => return Err(Error::AuthCancelled),
        };

        let url = format!("http://localhost{}", request.url());
        let parsed = url::Url::parse(&url).ok();
        let mut code: Option<String> = None;
        let mut state_ok = false;
        let mut err: Option<String> = None;
        if let Some(p) = parsed {
            for (k, v) in p.query_pairs() {
                match k.as_ref() {
                    "code" => code = Some(v.into_owned()),
                    "state" => state_ok = v == expected_state,
                    "error" => err = Some(v.into_owned()),
                    _ => {}
                }
            }
        }

        let (body, result) = if let Some(e) = err {
            (
                "Sign-in failed. You can close this tab.",
                Err(if e == "access_denied" {
                    Error::AuthCancelled
                } else {
                    Error::Auth(e)
                }),
            )
        } else if let (Some(c), true) = (code.clone(), state_ok) {
            ("Signed in! You can close this tab and return to SoraWorks Launcher.", Ok(c))
        } else {
            ("Unexpected response. You can close this tab.", Err(Error::Auth("bad redirect".into())))
        };

        let html = format!(
            "<!doctype html><html><head><meta charset=utf-8><title>SoraWorks</title>\
             <style>body{{font-family:system-ui;background:#0e1016;color:#eef1f8;\
             display:grid;place-items:center;height:100vh;margin:0}}</style></head>\
             <body><p>{body}</p></body></html>"
        );
        let response = tiny_http::Response::from_string(html).with_header(
            tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                .unwrap(),
        );
        let _ = request.respond(response);

        return result;
    }
}

// ---- Device Code flow ----

#[derive(Deserialize)]
pub struct DeviceCodeStart {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

pub async fn start_device_code() -> Result<DeviceCodeStart> {
    let params = [("client_id", CLIENT_ID), ("scope", SCOPE)];
    let resp = http::client()
        .post(DEVICE_CODE_URL)
        .form(&params)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

/// Poll the token endpoint until the user completes (or abandons) the login.
pub async fn poll_device_code(device_code: &str, interval: u64) -> Result<MsToken> {
    let mut wait = interval.max(1);
    loop {
        tokio::time::sleep(Duration::from_secs(wait)).await;

        let params = [
            ("client_id", CLIENT_ID),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("device_code", device_code),
        ];
        let resp: TokenResponse = http::client()
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        if let Some(err) = &resp.error {
            match err.as_str() {
                "authorization_pending" => continue,
                "slow_down" => {
                    wait += 5;
                    continue;
                }
                "expired_token" | "code_expired" => return Err(Error::AuthCancelled),
                "authorization_declined" | "access_denied" => return Err(Error::AuthCancelled),
                other => return Err(Error::Auth(other.to_string())),
            }
        }
        return resp.into_token();
    }
}

// ---- Refresh ----

pub async fn refresh(refresh_token: &str) -> Result<MsToken> {
    let params = [
        ("client_id", CLIENT_ID),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("scope", SCOPE),
    ];
    let resp: TokenResponse = http::client()
        .post(TOKEN_URL)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    resp.into_token()
}
