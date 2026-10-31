//! Vanilla Minecraft launch: resolve a version, download everything it needs,
//! build the command line, and start the JVM. Progress and process output are
//! reported to the frontend via Tauri events.

use crate::auth;
use crate::error::{Error, Result};
use crate::http;
use crate::java;
use crate::mojang::{self, ArgValue, ArgValueInner, VersionDetail};
use crate::paths;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;

/// Shared launcher state: the currently running game process (if any).
#[derive(Default)]
pub struct LaunchState {
    pub child: Arc<Mutex<Option<tokio::process::Child>>>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Progress {
    stage: String,
    done: u64,
    total: u64,
}

fn emit_progress(app: &AppHandle, stage: &str, done: u64, total: u64) {
    let _ = app.emit(
        "launch://progress",
        Progress {
            stage: stage.to_string(),
            done,
            total,
        },
    );
}

#[derive(Serialize, Clone)]
struct ExitPayload {
    code: Option<i32>,
}

/// Options passed from the frontend.
pub struct LaunchOptions {
    pub version_id: String,
    pub account_id: String,
    pub max_memory_mb: u32,
    pub extra_jvm_args: Vec<String>,
}

// ---- OS / rules helpers ----

fn os_rule_name() -> &'static str {
    match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "osx",
        _ => "linux",
    }
}

fn classpath_separator() -> &'static str {
    if cfg!(windows) {
        ";"
    } else {
        ":"
    }
}

/// Evaluate a library/argument rule list for the current OS.
fn rules_allow(rules: &[mojang::Rule]) -> bool {
    if rules.is_empty() {
        return true;
    }
    // Default deny when rules exist; an allow rule that matches turns it on.
    let mut allowed = false;
    for rule in rules {
        let matches = match &rule.os {
            None => true,
            Some(os) => {
                let name_ok = os.name.as_deref().map(|n| n == os_rule_name()).unwrap_or(true);
                let arch_ok = os
                    .arch
                    .as_deref()
                    .map(|a| a == std::env::consts::ARCH || (a == "x86" && cfg!(target_arch = "x86")))
                    .unwrap_or(true);
                name_ok && arch_ok
            }
        };
        if matches {
            allowed = rule.action == "allow";
        }
    }
    allowed
}

/// Derive a maven-style relative path from a library name like
/// `group:artifact:version[:classifier]`.
fn maven_path(name: &str) -> Option<String> {
    let parts: Vec<&str> = name.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    let group = parts[0].replace('.', "/");
    let artifact = parts[1];
    let version = parts[2];
    let classifier = parts.get(3).map(|c| format!("-{c}")).unwrap_or_default();
    Some(format!(
        "{group}/{artifact}/{version}/{artifact}-{version}{classifier}.jar"
    ))
}

// ---- Main entry ----

pub async fn launch_vanilla(
    app: AppHandle,
    state: Arc<Mutex<Option<tokio::process::Child>>>,
    opts: LaunchOptions,
    game_dir_override: String,
) -> Result<()> {
    let root = paths::data_root(&game_dir_override);
    emit_progress(&app, "preparing", 0, 0);

    // 1. Resolve version metadata.
    let url = mojang::version_url(&opts.version_id).await?;
    let detail: VersionDetail = mojang::fetch_version_detail(&url).await?;

    // 2. Ensure a valid account + token.
    let account = auth::valid_token(&opts.account_id).await?;

    // 3. Ensure Java.
    emit_progress(&app, "java", 0, 0);
    let major = detail
        .java_version
        .as_ref()
        .map(|j| j.major_version)
        .unwrap_or_else(|| java::default_major_for(&detail.id));
    let java_info = java::ensure(&root, major).await?;

    // 4. Client jar.
    let version_dir = paths::version_dir(&root, &detail.id);
    let client_jar = version_dir.join(format!("{}.jar", detail.id));
    http::download_to(&detail.downloads.client.url, &client_jar, Some(&detail.downloads.client.sha1))
        .await?;

    // 5. Libraries + natives.
    let (classpath, natives) = resolve_libraries(&app, &root, &detail).await?;
    let natives_dir = paths::natives_dir(&root, &detail.id);
    extract_natives(&app, &natives, &natives_dir).await?;

    // 6. Assets.
    download_assets(&app, &root, &detail).await?;

    // 7. Build the command line.
    emit_progress(&app, "launching", 0, 0);
    let mut full_cp = classpath;
    full_cp.push(client_jar.clone());
    let cp_string = full_cp
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(classpath_separator());

    let mem = if opts.max_memory_mb == 0 { 2048 } else { opts.max_memory_mb };

    let mut vars: HashMap<&str, String> = HashMap::new();
    vars.insert("auth_player_name", account.name.clone());
    vars.insert("version_name", detail.id.clone());
    vars.insert("game_directory", root.to_string_lossy().into_owned());
    vars.insert("assets_root", paths::assets_dir(&root).to_string_lossy().into_owned());
    vars.insert("assets_index_name", detail.assets.clone());
    vars.insert("auth_uuid", account.id.clone());
    vars.insert("auth_access_token", account.mc_access_token.clone());
    vars.insert("auth_session", format!("token:{}", account.mc_access_token));
    vars.insert("clientid", String::new());
    vars.insert("auth_xuid", String::new());
    vars.insert("user_type", "msa".into());
    vars.insert("version_type", detail.vtype.clone());
    vars.insert("user_properties", "{}".into());
    vars.insert("natives_directory", natives_dir.to_string_lossy().into_owned());
    vars.insert("launcher_name", "SoraWorksLauncher".into());
    vars.insert("launcher_version", env!("CARGO_PKG_VERSION").into());
    vars.insert("classpath", cp_string.clone());
    vars.insert("library_directory", paths::libraries_dir(&root).to_string_lossy().into_owned());
    vars.insert("classpath_separator", classpath_separator().into());
    vars.insert(
        "game_assets",
        paths::assets_dir(&root).join("virtual").join(&detail.assets).to_string_lossy().into_owned(),
    );

    let mut jvm_args: Vec<String> = Vec::new();
    jvm_args.push(format!("-Xmx{mem}M"));
    jvm_args.push(format!("-Xms{}M", (mem / 2).max(512)));
    jvm_args.extend(opts.extra_jvm_args.clone());

    let mut game_args: Vec<String> = Vec::new();

    match &detail.arguments {
        Some(args) => {
            // Modern (1.13+) structured arguments.
            jvm_args.extend(collect_args(&args.jvm, &vars));
            game_args.extend(collect_args(&args.game, &vars));
        }
        None => {
            // Legacy: a flat argument string + implicit JVM args.
            jvm_args.push(format!("-Djava.library.path={}", natives_dir.to_string_lossy()));
            jvm_args.push("-cp".into());
            jvm_args.push(cp_string.clone());
            if let Some(legacy) = &detail.minecraft_arguments {
                for tok in legacy.split_whitespace() {
                    game_args.push(substitute(tok, &vars));
                }
            }
        }
    }

    // For modern versions the classpath/main class come via -cp placeholder in
    // the jvm args; if the template didn't include it, add it defensively.
    if detail.arguments.is_some() && !jvm_args.iter().any(|a| a == "-cp" || a == "-classpath") {
        jvm_args.push("-cp".into());
        jvm_args.push(cp_string.clone());
    }

    // 8. Spawn.
    let mut command = tokio::process::Command::new(&java_info.path);
    command.current_dir(&root);
    command.args(&jvm_args);
    command.arg(&detail.main_class);
    command.args(&game_args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|e| Error::other(format!("spawn: {e}")))?;

    // Stream stdout/stderr to the frontend as log lines.
    if let Some(stdout) = child.stdout.take() {
        spawn_log_reader(app.clone(), stdout);
    }
    if let Some(stderr) = child.stderr.take() {
        spawn_log_reader(app.clone(), stderr);
    }

    emit_progress(&app, "running", 0, 0);

    // Keep the child so `stop` can kill it, and watch for exit.
    {
        let mut guard = state.lock().await;
        *guard = Some(child);
    }
    watch_exit(app, state);

    Ok(())
}

fn spawn_log_reader<R>(app: AppHandle, reader: R)
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let _ = app.emit("launch://log", line);
        }
    });
}

fn watch_exit(app: AppHandle, state: Arc<Mutex<Option<tokio::process::Child>>>) {
    tokio::spawn(async move {
        // Poll for exit so the child stays in the shared state (and thus
        // killable via `stop`) for as long as the game is running.
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let mut guard = state.lock().await;
            let exited = match guard.as_mut() {
                Some(child) => match child.try_wait() {
                    Ok(Some(status)) => Some(status.code()),
                    Ok(None) => None,         // still running
                    Err(_) => Some(None),     // treat as exited on error
                },
                None => return, // cleared elsewhere (e.g. a new launch)
            };
            if let Some(code) = exited {
                *guard = None;
                drop(guard);
                let _ = app.emit("launch://exit", ExitPayload { code });
                return;
            }
        }
    });
}

// ---- Argument expansion ----

fn substitute(template: &str, vars: &HashMap<&str, String>) -> String {
    let mut out = template.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("${{{k}}}"), v);
    }
    out
}

fn collect_args(args: &[ArgValue], vars: &HashMap<&str, String>) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        match arg {
            ArgValue::Plain(s) => out.push(substitute(s, vars)),
            ArgValue::Conditional { rules, value } => {
                if rules_allow(rules) {
                    match value {
                        ArgValueInner::One(s) => out.push(substitute(s, vars)),
                        ArgValueInner::Many(list) => {
                            for s in list {
                                out.push(substitute(s, vars));
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

// ---- Libraries / natives / assets ----

/// Download every applicable library and return (classpath entries, native jars).
async fn resolve_libraries(
    app: &AppHandle,
    root: &PathBuf,
    detail: &VersionDetail,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let lib_dir = paths::libraries_dir(root);
    let mut classpath = Vec::new();
    let mut natives = Vec::new();

    let total = detail.libraries.len() as u64;
    let mut done = 0u64;

    for lib in &detail.libraries {
        done += 1;
        if let Some(rules) = &lib.rules {
            if !rules_allow(rules) {
                continue;
            }
        }

        if let Some(downloads) = &lib.downloads {
            // Regular jar on the classpath.
            if let Some(artifact) = &downloads.artifact {
                let rel = artifact
                    .path
                    .clone()
                    .or_else(|| maven_path(&lib.name))
                    .ok_or_else(|| Error::other(format!("no path for {}", lib.name)))?;
                let dest = lib_dir.join(&rel);
                http::download_to(&artifact.url, &dest, Some(&artifact.sha1)).await?;
                classpath.push(dest);
            }

            // Native classifier for this OS.
            if let Some(natives_map) = &lib.natives {
                if let Some(classifier) = natives_map.get(os_rule_name()) {
                    let classifier = classifier.replace("${arch}", "64");
                    if let Some(classifiers) = &downloads.classifiers {
                        if let Some(artifact) = classifiers.get(&classifier) {
                            let rel = artifact
                                .path
                                .clone()
                                .or_else(|| maven_path(&lib.name))
                                .ok_or_else(|| Error::other("no native path".to_string()))?;
                            let dest = lib_dir.join(&rel);
                            http::download_to(&artifact.url, &dest, Some(&artifact.sha1)).await?;
                            natives.push(dest);
                        }
                    }
                }
            }
        }

        emit_progress(app, "libraries", done, total);
    }

    Ok((classpath, natives))
}

/// Unzip native jars into the per-version natives directory.
async fn extract_natives(app: &AppHandle, natives: &[PathBuf], natives_dir: &Path) -> Result<()> {
    emit_progress(app, "natives", 0, natives.len() as u64);
    tokio::fs::create_dir_all(natives_dir).await?;

    let natives = natives.to_vec();
    let out = natives_dir.to_path_buf();
    tokio::task::spawn_blocking(move || -> Result<()> {
        for jar in &natives {
            let file = std::fs::File::open(jar)?;
            let mut zip = zip::ZipArchive::new(file)?;
            for i in 0..zip.len() {
                let mut entry = zip.by_index(i)?;
                let name = entry.name().to_string();
                // Skip signatures / metadata.
                if name.starts_with("META-INF") || name.ends_with('/') {
                    continue;
                }
                let dest = out.join(Path::new(&name).file_name().unwrap_or_default());
                let mut writer = std::fs::File::create(&dest)?;
                std::io::copy(&mut entry, &mut writer)?;
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| Error::other(format!("natives join: {e}")))??;

    Ok(())
}

/// Download the asset index and all referenced objects.
async fn download_assets(app: &AppHandle, root: &PathBuf, detail: &VersionDetail) -> Result<()> {
    let assets_dir = paths::assets_dir(root);
    let index_path = assets_dir.join("indexes").join(format!("{}.json", detail.asset_index.id));
    http::download_to(&detail.asset_index.url, &index_path, Some(&detail.asset_index.sha1)).await?;

    let index = mojang::fetch_asset_index(&detail.asset_index.url).await?;
    let objects_dir = assets_dir.join("objects");

    let total = index.objects.len() as u64;
    let mut done = 0u64;
    emit_progress(app, "assets", 0, total);

    // A simple bounded-concurrency download loop.
    let mut tasks = futures_util::stream::FuturesUnordered::new();
    use futures_util::StreamExt;

    for (_name, obj) in index.objects.iter() {
        let hash = obj.hash.clone();
        let prefix = &hash[0..2];
        let url = format!("https://resources.download.minecraft.net/{prefix}/{hash}");
        let dest = objects_dir.join(prefix).join(&hash);
        tasks.push(async move { http::download_to(&url, &dest, Some(&hash)).await });

        if tasks.len() >= 16 {
            if let Some(res) = tasks.next().await {
                res?;
                done += 1;
                if done % 20 == 0 {
                    emit_progress(app, "assets", done, total);
                }
            }
        }
    }
    while let Some(res) = tasks.next().await {
        res?;
    }
    let _ = done;
    emit_progress(app, "assets", total, total);

    Ok(())
}

pub async fn stop(state: Arc<Mutex<Option<tokio::process::Child>>>) -> Result<()> {
    let mut guard = state.lock().await;
    if let Some(child) = guard.as_mut() {
        let _ = child.start_kill();
    }
    Ok(())
}

pub async fn is_running(state: Arc<Mutex<Option<tokio::process::Child>>>) -> bool {
    let guard = state.lock().await;
    guard.is_some()
}
