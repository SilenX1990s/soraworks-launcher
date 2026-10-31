//! Tauri command handlers — the bridge between the Svelte frontend and the
//! launcher backend. Each command maps 1:1 to a wrapper in src/lib/tauri.ts.

use crate::auth::{self, Account, DeviceCodeInfo};
use crate::error::Result;
use crate::java::{self, JavaInfo};
use crate::launch::{self, LaunchOptions, LaunchState};
use crate::mojang::{self, VersionManifest};
use crate::paths;
use serde::Deserialize;
use tauri::State;

// ---- Auth / accounts ----

#[tauri::command]
pub async fn auth_login_browser() -> Result<Account> {
    auth::login_browser().await
}

#[tauri::command]
pub async fn auth_start_device_code() -> Result<DeviceCodeInfo> {
    auth::start_device_code().await
}

#[tauri::command]
pub async fn auth_complete_device_code(device_code: String) -> Result<Account> {
    auth::complete_device_code(&device_code).await
}

#[tauri::command]
pub async fn accounts_list() -> Result<Vec<Account>> {
    Ok(auth::list())
}

#[tauri::command]
pub async fn accounts_remove(id: String) -> Result<()> {
    auth::remove(&id)
}

#[tauri::command]
pub async fn accounts_set_active(id: String) -> Result<()> {
    auth::set_active(&id)
}

#[tauri::command]
pub async fn auth_refresh(id: String) -> Result<Account> {
    auth::refresh(&id).await
}

// ---- Mojang ----

#[tauri::command]
pub async fn mojang_versions(force_refresh: bool) -> Result<VersionManifest> {
    mojang::fetch_manifest(force_refresh).await
}

// ---- Java ----

#[tauri::command]
pub async fn java_ensure(major: u32) -> Result<JavaInfo> {
    let root = paths::default_data_root();
    java::ensure(&root, major).await
}

// ---- Launch ----

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchOptionsDto {
    pub version_id: String,
    pub account_id: String,
    #[serde(default)]
    pub max_memory_mb: u32,
    #[serde(default)]
    pub extra_jvm_args: Vec<String>,
}

#[tauri::command]
pub async fn launch_vanilla(
    app: tauri::AppHandle,
    state: State<'_, LaunchState>,
    options: LaunchOptionsDto,
    game_dir: String,
) -> Result<()> {
    let opts = LaunchOptions {
        version_id: options.version_id,
        account_id: options.account_id,
        max_memory_mb: options.max_memory_mb,
        extra_jvm_args: options.extra_jvm_args,
    };
    launch::launch_vanilla(app, state.child.clone(), opts, game_dir).await
}

#[tauri::command]
pub async fn launch_stop(state: State<'_, LaunchState>) -> Result<()> {
    launch::stop(state.child.clone()).await
}

#[tauri::command]
pub async fn launch_is_running(state: State<'_, LaunchState>) -> Result<bool> {
    Ok(launch::is_running(state.child.clone()).await)
}

// ---- Paths ----

#[tauri::command]
pub async fn paths_default_game_dir() -> Result<String> {
    Ok(paths::default_data_root().to_string_lossy().into_owned())
}
