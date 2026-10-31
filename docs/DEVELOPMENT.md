# Development & architecture

## Stack

- **Frontend:** Svelte 5 (runes) + Vite + TypeScript, in `src/`.
- **Backend:** Rust (Tauri v2), in `src-tauri/`.
- The two talk over Tauri's command/event bridge. Every backend command has a
  typed wrapper in [`src/lib/tauri.ts`](../src/lib/tauri.ts); payload shapes are
  mirrored in [`src/lib/types.ts`](../src/lib/types.ts).

## Frontend layout

```
src/
  main.ts                 app entry; applies the saved theme before first paint
  App.svelte              shell: sidebar + topbar + view switch
  app.css                 global styles driven entirely by CSS variables
  lib/
    i18n/                 locale store + th/ja/en JSON (instant switching)
    theme/presets.ts      built-in themes (serializable)
    stores/               theme, settings, accounts, selection (localStorage-backed)
    components/           Sidebar, LanguagePicker
    views/                Home, Versions, Accounts, Settings, About
    tauri.ts              typed command/event wrappers
    types.ts              shared DTOs
```

Theming works by writing CSS custom properties (`--accent`, `--surface`, …) to
`:root` from the theme store. Because every component styles itself from those
variables, changing a theme (or editing a color) restyles the whole UI with no
reload.

## Backend layout

```
src-tauri/src/
  main.rs / lib.rs        Tauri builder, plugin + command registration
  commands.rs             #[tauri::command] handlers (the only public surface)
  error.rs                Error enum; Display strings are stable codes the UI maps
  http.rs                 shared reqwest client + checksummed downloader
  paths.rs                data-root layout (versions/, libraries/, assets/, runtime/)
  mojang/                 version manifest + per-version detail + asset index
  java/                   Adoptium (Temurin) download + archive extraction
  auth/
    microsoft.rs          OAuth PKCE + device code + refresh (consumers tenant)
    xbox.rs               Xbox Live -> XSTS -> Minecraft Services + profile
    store.rs              OS-keychain account storage (multi-account + active)
    mod.rs                public auth API, produces the frontend Account
  launch/                 vanilla launch: download, classpath, args, spawn, events
```

### Error codes

`error.rs` variants serialize to stable strings (`auth_no_xbox`, `auth_child`,
`network`, …). The frontend matches on these and shows the matching
`error.*` translation, so **don't change a code without updating the locale
files** in `src/lib/i18n/`.

### Events

The launcher emits:

- `launch://progress` — `{ stage, done, total }`
- `launch://log` — a stdout/stderr line from the game
- `launch://exit` — `{ code }`

## Microsoft OAuth

Public client, `consumers` tenant (personal Microsoft accounts only). The Azure
client id is embedded (it is not a secret). Two flows:

1. **PKCE / browser** — a loopback HTTP server on a random port catches the
   redirect; the code is exchanged with the PKCE verifier.
2. **Device code** — shows a short code + URL; polled until the user completes it.

Both continue through the same Xbox chain and store tokens in the keychain.

## Conventions

- Run `npm run check` (svelte-check) and `cargo check` before committing.
- Keep `src/lib/types.ts` and the Rust `serde` structs in sync (all camelCase
  over the bridge).
