# SoraWorks Launcher

A lightweight Minecraft: Java Edition launcher for the players of the SoraWorks
modpack and server community. Built with **Tauri v2 (Rust)** and **Svelte 5**
for Windows, macOS and Linux.

> Online Microsoft login only — there is **no offline or cracked mode**.

## Status — Phase 1 (foundation)

Implemented in this phase:

- **Project skeleton** — Tauri v2 + Svelte 5 + Vite + TypeScript, three-OS build.
- **Three languages** — Thai / 日本語 / English, switchable instantly (no restart),
  from per-language JSON files in `src/lib/i18n/`.
- **Theme system** — presets plus live editing of accent color, light/dark mode,
  corner radius, surface translucency and font size; themes export/import as a
  shareable JSON file.
- **Microsoft login** — OAuth 2.0 Authorization Code + **PKCE** (loopback
  redirect) with **Device Code** as a fallback, through the full
  Xbox Live → XSTS → Minecraft Services chain. Tokens are stored encrypted in
  the **OS keychain**; multiple accounts are supported and refreshed silently.
  Xbox/XSTS errors are translated into all three languages.
- **Version support** — the full release/snapshot list is pulled live from the
  Mojang version manifest.
- **Vanilla launch** — downloads the client, libraries, natives and assets (with
  SHA-1 verification), builds the classpath and arguments (modern and legacy),
  and starts the game.
- **Automatic Java** — the matching Temurin (Adoptium) JRE is downloaded per
  game version (e.g. Java 8 for old versions, 17/21 for new ones).

See [`docs/ROADMAP.md`](docs/ROADMAP.md) for the later phases (modpack sync,
optimizer, dashboard, RCON, crash analysis, and the mod-protection layers).

## Development

Requirements: Node 20+, Rust (stable), and the
[Tauri v2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for
your OS.

```bash
npm install
npm run tauri dev
```

Build a production bundle:

```bash
npm run tauri build
```

See [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md) for the architecture.

## Privacy

Login tokens are stored encrypted in the OS keychain and are never sent to any
third-party server. Crash reports (a later phase) will strip tokens and personal
file paths and are only ever sent with the player's explicit consent.

## License

Proprietary — © SoraWorks. The launcher is distributed to the SoraWorks
community; mods by other authors are downloaded from their official sources
(Modrinth / CurseForge) under their own licenses.
