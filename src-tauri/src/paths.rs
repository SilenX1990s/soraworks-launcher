//! Filesystem layout. The launcher keeps everything under one data root,
//! using the familiar Minecraft layout (versions/, libraries/, assets/) so a
//! downloaded instance looks like a normal `.minecraft`.

use std::path::PathBuf;

/// Root folder for all launcher data. Can be overridden by the user's
/// "game folder" setting; this is the default when that is empty.
pub fn default_data_root() -> PathBuf {
    // Prefer a dedicated folder so we never clobber an existing vanilla
    // `.minecraft` the player may already have.
    let base = dirs::data_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("SoraWorksLauncher")
}

/// Resolve the effective data root from an optional user override.
pub fn data_root(override_dir: &str) -> PathBuf {
    if override_dir.trim().is_empty() {
        default_data_root()
    } else {
        PathBuf::from(override_dir)
    }
}

pub fn versions_dir(root: &PathBuf) -> PathBuf {
    root.join("versions")
}

pub fn version_dir(root: &PathBuf, id: &str) -> PathBuf {
    versions_dir(root).join(id)
}

pub fn libraries_dir(root: &PathBuf) -> PathBuf {
    root.join("libraries")
}

pub fn assets_dir(root: &PathBuf) -> PathBuf {
    root.join("assets")
}

pub fn natives_dir(root: &PathBuf, id: &str) -> PathBuf {
    version_dir(root, id).join("natives")
}

/// Where downloaded Java runtimes live, one folder per major version.
pub fn java_dir(root: &PathBuf) -> PathBuf {
    root.join("runtime")
}
