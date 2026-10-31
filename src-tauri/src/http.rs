//! Shared HTTP client and download helpers.

use crate::error::{Error, Result};
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use std::path::Path;
use std::sync::OnceLock;
use tokio::io::AsyncWriteExt;

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent(concat!("SoraWorksLauncher/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client")
    })
}

pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
    let resp = client().get(url).send().await?.error_for_status()?;
    Ok(resp.json::<T>().await?)
}

#[allow(dead_code)] // used by later phases (modpack manifest fetch)
pub async fn get_bytes(url: &str) -> Result<Vec<u8>> {
    let resp = client().get(url).send().await?.error_for_status()?;
    Ok(resp.bytes().await?.to_vec())
}

/// Download `url` to `dest`, creating parent dirs. If `expected_sha1` is given
/// and an existing file already matches it, the download is skipped.
pub async fn download_to(url: &str, dest: &Path, expected_sha1: Option<&str>) -> Result<()> {
    if let Some(want) = expected_sha1 {
        if let Ok(existing) = tokio::fs::read(dest).await {
            if sha1_hex(&existing) == want.to_lowercase() {
                return Ok(());
            }
        }
    } else if tokio::fs::try_exists(dest).await.unwrap_or(false) {
        return Ok(());
    }

    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let resp = client().get(url).send().await?.error_for_status()?;
    let mut stream = resp.bytes_stream();

    // Download to a temp file first, then rename, so a crash never leaves a
    // half-written file that would later pass as "already downloaded".
    let tmp = dest.with_extension("part");
    let mut file = tokio::fs::File::create(&tmp).await?;
    let mut hasher = Sha1::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        hasher.update(&chunk);
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    drop(file);

    if let Some(want) = expected_sha1 {
        let got = hex::encode(hasher.finalize());
        if got != want.to_lowercase() {
            let _ = tokio::fs::remove_file(&tmp).await;
            return Err(Error::other(format!(
                "checksum mismatch for {url}: expected {want}, got {got}"
            )));
        }
    }

    tokio::fs::rename(&tmp, dest).await?;
    Ok(())
}

pub fn sha1_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
