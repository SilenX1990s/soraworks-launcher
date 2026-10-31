//! Account / authentication API used by the Tauri commands.

mod microsoft;
mod store;
mod xbox;

pub use store::StoredAccount;

use crate::error::{Error, Result};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// Account shape sent to the frontend.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub expires_at: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCodeInfo {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn to_account(acc: &StoredAccount, active_id: Option<&str>) -> Account {
    Account {
        id: acc.id.clone(),
        name: acc.name.clone(),
        active: active_id == Some(acc.id.as_str()),
        expires_at: acc.mc_expires_at,
    }
}

/// Turn a Microsoft token set into a stored account + Minecraft session.
async fn finalize(ms: microsoft::MsToken) -> Result<Account> {
    let mc = xbox::authenticate(&ms.access_token).await?;
    let stored = StoredAccount {
        id: mc.uuid,
        name: mc.name,
        mc_access_token: mc.access_token,
        mc_expires_at: now() + mc.expires_in,
        ms_refresh_token: ms.refresh_token,
    };
    store::save_account(&stored)?;
    Ok(to_account(&stored, store::active_id().as_deref()))
}

pub async fn login_browser() -> Result<Account> {
    let ms = microsoft::login_pkce().await?;
    finalize(ms).await
}

pub async fn start_device_code() -> Result<DeviceCodeInfo> {
    let start = microsoft::start_device_code().await?;
    Ok(DeviceCodeInfo {
        device_code: start.device_code,
        user_code: start.user_code,
        verification_uri: start.verification_uri,
        expires_in: start.expires_in,
        interval: start.interval,
    })
}

pub async fn complete_device_code(device_code: &str) -> Result<Account> {
    // Poll with a conservative default interval; `slow_down` bumps it.
    let ms = microsoft::poll_device_code(device_code, 5).await?;
    finalize(ms).await
}

pub fn list() -> Vec<Account> {
    let active = store::active_id();
    store::list()
        .iter()
        .map(|a| to_account(a, active.as_deref()))
        .collect()
}

pub fn remove(id: &str) -> Result<()> {
    store::delete_account(id)
}

pub fn set_active(id: &str) -> Result<()> {
    store::set_active(id)
}

/// Refresh an account's Minecraft token using its stored Microsoft refresh
/// token, re-running the Xbox chain.
pub async fn refresh(id: &str) -> Result<Account> {
    let stored = store::load_account(id)?;
    if stored.ms_refresh_token.is_empty() {
        return Err(Error::Auth("no refresh token".into()));
    }
    let ms = microsoft::refresh(&stored.ms_refresh_token).await?;
    let mc = xbox::authenticate(&ms.access_token).await?;
    let updated = StoredAccount {
        id: stored.id,
        name: mc.name,
        mc_access_token: mc.access_token,
        mc_expires_at: now() + mc.expires_in,
        // Microsoft may or may not rotate the refresh token.
        ms_refresh_token: if ms.refresh_token.is_empty() {
            stored.ms_refresh_token
        } else {
            ms.refresh_token
        },
    };
    store::save_account(&updated)?;
    Ok(to_account(&updated, store::active_id().as_deref()))
}

/// Return a valid Minecraft access token for `id`, refreshing if it is near
/// expiry. Used by the launcher just before starting the game.
pub async fn valid_token(id: &str) -> Result<StoredAccount> {
    let stored = store::load_account(id)?;
    // Refresh if it expires within the next 2 minutes.
    if stored.mc_expires_at <= now() + 120 {
        refresh(id).await?;
        return store::load_account(id);
    }
    Ok(stored)
}
