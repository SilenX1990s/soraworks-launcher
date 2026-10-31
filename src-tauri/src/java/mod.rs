//! Automatic Java runtime management via the Adoptium (Eclipse Temurin) API.
//!
//! Each Minecraft version declares the Java major version it needs
//! (`javaVersion.majorVersion`); older versions that don't declare one get a
//! sensible default. We download the matching Temurin JRE once and reuse it.

use crate::error::{Error, Result};
use crate::http;
use crate::paths;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JavaInfo {
    pub major: u32,
    pub path: String,
}

fn adoptium_os() -> &'static str {
    match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "mac",
        _ => "linux",
    }
}

fn adoptium_arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        "x86" => "x86",
        other => other,
    }
}

fn java_exe_name() -> &'static str {
    if cfg!(windows) {
        "java.exe"
    } else {
        "java"
    }
}

/// Ensure a JRE for `major` is installed under the data root, downloading it
/// from Adoptium if necessary. Returns the path to the `java` executable.
pub async fn ensure(root: &PathBuf, major: u32) -> Result<JavaInfo> {
    let target = paths::java_dir(root).join(major.to_string());

    // Already installed?
    if let Some(exe) = find_java_exe(&target) {
        return Ok(JavaInfo {
            major,
            path: exe.to_string_lossy().into_owned(),
        });
    }

    tokio::fs::create_dir_all(&target).await?;

    let url = format!(
        "https://api.adoptium.net/v3/binary/latest/{major}/ga/{os}/{arch}/jre/hotspot/normal/eclipse",
        major = major,
        os = adoptium_os(),
        arch = adoptium_arch(),
    );

    let is_zip = cfg!(windows);
    let archive = target.join(if is_zip { "download.zip" } else { "download.tar.gz" });
    http::download_to(&url, &archive, None).await?;

    // Extraction is blocking/CPU-bound, so run it off the async runtime.
    let target_clone = target.clone();
    let archive_clone = archive.clone();
    tokio::task::spawn_blocking(move || {
        if is_zip {
            extract_zip(&archive_clone, &target_clone)
        } else {
            extract_targz(&archive_clone, &target_clone)
        }
    })
    .await
    .map_err(|e| Error::other(format!("extract join: {e}")))??;

    let _ = tokio::fs::remove_file(&archive).await;

    let exe = find_java_exe(&target)
        .ok_or_else(|| Error::other("java executable not found after extraction"))?;

    Ok(JavaInfo {
        major,
        path: exe.to_string_lossy().into_owned(),
    })
}

/// Walk `dir` looking for a `bin/java[.exe]` executable.
fn find_java_exe(dir: &Path) -> Option<PathBuf> {
    let want = java_exe_name();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(cur) = stack.pop() {
        let entries = std::fs::read_dir(&cur).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some(want) {
                // Must live inside a `bin` directory.
                if path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str())
                    == Some("bin")
                {
                    return Some(path);
                }
            }
        }
    }
    None
}

fn extract_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let out = match entry.enclosed_name() {
            Some(p) => dest.join(p),
            None => continue,
        };
        if entry.is_dir() {
            std::fs::create_dir_all(&out)?;
        } else {
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut writer = std::fs::File::create(&out)?;
            std::io::copy(&mut entry, &mut writer)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() {
                    let _ = std::fs::set_permissions(&out, std::fs::Permissions::from_mode(mode));
                }
            }
        }
    }
    Ok(())
}

fn extract_targz(archive: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(archive)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut tar = tar::Archive::new(decoder);
    tar.set_preserve_permissions(true);
    tar.unpack(dest)?;
    Ok(())
}

/// Default Java major for versions that don't declare one (very old releases).
pub fn default_major_for(version_id: &str) -> u32 {
    // 1.17+ needs 16/17, 1.18+ needs 17, 1.20.5+ needs 21. Versions without a
    // declared javaVersion are pre-1.17 and run fine on 8.
    let _ = version_id;
    8
}
