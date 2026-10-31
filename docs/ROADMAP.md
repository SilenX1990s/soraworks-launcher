# Roadmap

The launcher is being built in phases. Phase 1 is the foundation in this repo;
later phases build on it.

## Phase 1 — Foundation ✅ (current)
Project skeleton, 3-language UI, theme system, Microsoft login, vanilla launch
for any version, automatic Java download.

## Phase 2 — Loaders & modpack sync
- Forge **1.20.1** (default preset), NeoForge, Fabric, Quilt, with per-version
  loader selection.
- Modpack Sync: one-click update from a manifest on Cloudflare R2, hashing every
  file and downloading only what changed.
- Baseline Cloudflare backend: Workers (API), D1 (database), R2 (storage).
- Other authors' mods are fetched from Modrinth / CurseForge, never re-hosted.

## Phase 3 — Optimizer & extras
- **Optimize** button (fully reversible): RAM sizing, JVM flags, Java
  auto-download, high priority while playing, temporary power plan, forcing the
  discrete GPU, an FPS mod set, and `options.txt` tuning. **Low-spec mode.**
- News/announcements, live server status + player count, play-time stats,
  screenshot gallery, skin management, world backup, Discord Rich Presence,
  live log view, modpack update notifications.

## Phase 4 — Crash analysis, dashboard & RCON
- Local rule-based crash analysis (missing dependency, version mismatch, Mixin,
  out-of-memory, driver, wrong Java) explained in all three languages; tokens
  and personal paths stripped; sent only with consent.
- Backend: store crash files in R2, AI (Claude API) root-cause analysis, dedupe
  repeated crashes, Discord webhook alerts.
- Web dashboard: manage modpacks, news, allowed players, anti-leak settings,
  view crashes.
- **RCON** from the dashboard: two-way whitelist sync with the launcher, plus
  common commands (list, say/broadcast, kick/ban, restart, op), multi-server,
  encrypted passwords, a command audit log, and a retry queue when a server is
  offline.

## Phase 5 — Mod-protection layers (own mods only)
Configurable per instance and per mod, with presets (off / basic / strict /
maximum). Order of implementation: **1, 4, 5, 6 first, then 2, 3, 7**.

1. Distribution control — download only for whitelisted, logged-in UUIDs via
   short-lived per-user signed URLs; per-user revocation and device limits.
2. On-disk encryption — AES-256-GCM jars, per-user/device keys from the Worker.
3. In-memory loading — a Forge 1.20.1 custom `IModLocator` that decrypts in RAM
   (**research + prototype first**).
4. Per-user watermark — a unique id embedded in multiple places, plus a
   dashboard tool to identify the source of a leaked file.
5. Online license — the mod requests a short-lived Ed25519-signed token bound to
   the UUID at startup; no token, no run.
6. Server handshake — the server-side mod verifies the client came from a valid
   launcher with a valid license before letting it join.
7. Build-time obfuscation — obfuscate own mods (careful with Mixin / reflection /
   Epic Fight API).

Each layer will ship with **honest documentation of what it does and does not
protect** — no layer is 100%. **No malware techniques** (no kernel drivers, no
process hiding, no covert data collection).

## Phase 6 — Self-update
The launcher updates itself from R2 using signed update files; three-OS builds
via GitHub Actions.
