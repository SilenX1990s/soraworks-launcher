//! Mojang (piston-meta) metadata: the version manifest and per-version detail.

use crate::error::Result;
use crate::http;
use crate::paths;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

// ---- Version manifest ----

#[derive(Debug, Deserialize)]
struct RawManifest {
    latest: RawLatest,
    versions: Vec<RawVersion>,
}

#[derive(Debug, Deserialize)]
struct RawLatest {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)] // `sha1`/`url` kept to mirror the manifest schema
pub struct RawVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub vtype: String,
    pub url: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    #[serde(default)]
    pub sha1: Option<String>,
}

/// Shape sent to the frontend (camelCase, trimmed down).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    pub latest_release: String,
    pub latest_snapshot: String,
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub vtype: String,
    pub release_time: String,
}

/// Fetch the manifest, with a small on-disk cache. On network failure we fall
/// back to the cached copy if one exists.
pub async fn fetch_manifest(force_refresh: bool) -> Result<VersionManifest> {
    let cache = paths::default_data_root().join("cache").join("version_manifest_v2.json");

    let raw: RawManifest = if force_refresh {
        let text = fetch_and_cache(&cache).await?;
        serde_json::from_str(&text)?
    } else {
        match http::client().get(MANIFEST_URL).send().await {
            Ok(resp) => {
                let text = resp.error_for_status()?.text().await?;
                // Refresh the cache opportunistically; ignore write errors.
                if let Some(parent) = cache.parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                let _ = tokio::fs::write(&cache, &text).await;
                serde_json::from_str(&text)?
            }
            // Offline: fall back to the cached copy if we have one.
            Err(_) => {
                let text = tokio::fs::read_to_string(&cache).await?;
                serde_json::from_str(&text)?
            }
        }
    };

    Ok(VersionManifest {
        latest_release: raw.latest.release,
        latest_snapshot: raw.latest.snapshot,
        versions: raw
            .versions
            .into_iter()
            .map(|v| VersionEntry {
                id: v.id,
                vtype: v.vtype,
                release_time: v.release_time,
            })
            .collect(),
    })
}

async fn fetch_and_cache(cache: &std::path::Path) -> Result<String> {
    let text = http::client()
        .get(MANIFEST_URL)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    if let Some(parent) = cache.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let _ = tokio::fs::write(cache, &text).await;
    Ok(text)
}

/// Resolve a single version's metadata URL from the manifest.
pub async fn version_url(id: &str) -> Result<String> {
    let raw: RawManifest = http::get_json(MANIFEST_URL).await?;
    raw.versions
        .into_iter()
        .find(|v| v.id == id)
        .map(|v| v.url)
        .ok_or_else(|| crate::error::Error::NotFound(format!("version {id}")))
}

// ---- Per-version detail (used by the launcher) ----

#[derive(Debug, Deserialize)]
pub struct VersionDetail {
    pub id: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndexRef,
    pub assets: String,
    pub downloads: ClientDownloads,
    pub libraries: Vec<Library>,
    #[serde(default)]
    pub arguments: Option<Arguments>,
    #[serde(default, rename = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
    #[serde(default, rename = "javaVersion")]
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "type")]
    pub vtype: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // `total_size` kept to mirror the schema
pub struct AssetIndexRef {
    pub id: String,
    pub url: String,
    pub sha1: String,
    #[serde(rename = "totalSize", default)]
    pub total_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ClientDownloads {
    pub client: Artifact,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)] // `size` kept to mirror the schema
pub struct Artifact {
    #[serde(default)]
    pub path: Option<String>,
    pub url: String,
    pub sha1: String,
    #[serde(default)]
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub name: String,
    #[serde(default)]
    pub downloads: Option<LibraryDownloads>,
    #[serde(default)]
    pub rules: Option<Vec<Rule>>,
    #[serde(default)]
    pub natives: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownloads {
    #[serde(default)]
    pub artifact: Option<Artifact>,
    #[serde(default)]
    pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // `features` kept to mirror the schema (used by later phases)
pub struct Rule {
    pub action: String,
    #[serde(default)]
    pub os: Option<OsRule>,
    #[serde(default)]
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Deserialize)]
pub struct OsRule {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JavaVersion {
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

/// Minecraft's launch arguments can be either the modern structured form
/// (`arguments`) or the legacy flat string (`minecraftArguments`).
#[derive(Debug, Deserialize)]
pub struct Arguments {
    #[serde(default)]
    pub game: Vec<ArgValue>,
    #[serde(default)]
    pub jvm: Vec<ArgValue>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgValue {
    Plain(String),
    Conditional { rules: Vec<Rule>, value: ArgValueInner },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgValueInner {
    One(String),
    Many(Vec<String>),
}

// ---- Asset index ----

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // `size` kept to mirror the schema
pub struct AssetObject {
    pub hash: String,
    #[serde(default)]
    pub size: u64,
}

pub async fn fetch_version_detail(url: &str) -> Result<VersionDetail> {
    http::get_json(url).await
}

pub async fn fetch_asset_index(url: &str) -> Result<AssetIndex> {
    http::get_json(url).await
}
