//! Encrypted, OS-keychain-backed storage of account tokens.
//!
//! Each account's tokens live in a separate keychain entry keyed by the
//! Minecraft UUID. A small index entry tracks which accounts exist and which
//! one is active. Nothing is written in plaintext to disk.

use crate::error::{Error, Result};
use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE: &str = "com.soraworks.launcher";
const INDEX_KEY: &str = "__index__";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAccount {
    /// Minecraft profile UUID (no dashes).
    pub id: String,
    /// Minecraft username.
    pub name: String,
    /// Minecraft access token (bearer for the game session).
    pub mc_access_token: String,
    /// Unix seconds when the Minecraft token expires.
    pub mc_expires_at: u64,
    /// Microsoft refresh token, used to silently re-auth.
    pub ms_refresh_token: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Index {
    pub ids: Vec<String>,
    #[serde(default)]
    pub active: Option<String>,
}

fn entry(key: &str) -> Result<Entry> {
    Entry::new(SERVICE, key).map_err(Error::from)
}

pub fn load_index() -> Index {
    match entry(INDEX_KEY).and_then(|e| e.get_password().map_err(Error::from)) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => Index::default(),
    }
}

fn save_index(index: &Index) -> Result<()> {
    let json = serde_json::to_string(index)?;
    entry(INDEX_KEY)?.set_password(&json)?;
    Ok(())
}

pub fn load_account(id: &str) -> Result<StoredAccount> {
    let json = entry(id)?.get_password()?;
    Ok(serde_json::from_str(&json)?)
}

/// Save (or overwrite) an account and make sure it is in the index. The first
/// account added becomes the active one.
pub fn save_account(acc: &StoredAccount) -> Result<()> {
    let json = serde_json::to_string(acc)?;
    entry(&acc.id)?.set_password(&json)?;

    let mut index = load_index();
    if !index.ids.contains(&acc.id) {
        index.ids.push(acc.id.clone());
    }
    if index.active.is_none() {
        index.active = Some(acc.id.clone());
    }
    save_index(&index)
}

pub fn delete_account(id: &str) -> Result<()> {
    if let Ok(e) = entry(id) {
        let _ = e.delete_credential();
    }
    let mut index = load_index();
    index.ids.retain(|x| x != id);
    if index.active.as_deref() == Some(id) {
        index.active = index.ids.first().cloned();
    }
    save_index(&index)
}

pub fn set_active(id: &str) -> Result<()> {
    let mut index = load_index();
    if !index.ids.contains(&id.to_string()) {
        return Err(Error::NotFound(format!("account {id}")));
    }
    index.active = Some(id.to_string());
    save_index(&index)
}

pub fn list() -> Vec<StoredAccount> {
    let index = load_index();
    index
        .ids
        .iter()
        .filter_map(|id| load_account(id).ok())
        .collect()
}

pub fn active_id() -> Option<String> {
    load_index().active
}
