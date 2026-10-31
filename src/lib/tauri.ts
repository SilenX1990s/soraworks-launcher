// Typed wrappers around the Tauri command + event bridge.
// Each function maps 1:1 to a #[tauri::command] in the Rust backend.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  Account,
  DeviceCodeInfo,
  JavaInfo,
  LaunchExit,
  LaunchOptions,
  LaunchProgress,
  VersionManifest,
} from "./types";

// ---- Accounts / auth ----

export const auth = {
  /** Full interactive browser (PKCE) login. Resolves with the new account. */
  loginBrowser: () => invoke<Account>("auth_login_browser"),

  /** Start the device-code flow; show the returned code to the user. */
  startDeviceCode: () => invoke<DeviceCodeInfo>("auth_start_device_code"),

  /** Poll until the user finishes the device-code login (or it fails). */
  completeDeviceCode: (deviceCode: string) =>
    invoke<Account>("auth_complete_device_code", { deviceCode }),

  list: () => invoke<Account[]>("accounts_list"),
  remove: (id: string) => invoke<void>("accounts_remove", { id }),
  setActive: (id: string) => invoke<void>("accounts_set_active", { id }),

  /** Refresh (and silently re-auth) an account's Minecraft token. */
  refresh: (id: string) => invoke<Account>("auth_refresh", { id }),
};

// ---- Mojang versions ----

export const mojang = {
  versions: (forceRefresh = false) =>
    invoke<VersionManifest>("mojang_versions", { forceRefresh }),
};

// ---- Java ----

export const java = {
  /** Ensure a runtime for the given major version exists, downloading if needed. */
  ensure: (major: number) => invoke<JavaInfo>("java_ensure", { major }),
};

// ---- Launch ----

export const launch = {
  vanilla: (options: LaunchOptions, gameDir = "") =>
    invoke<void>("launch_vanilla", { options, gameDir }),
  stop: () => invoke<void>("launch_stop"),
  isRunning: () => invoke<boolean>("launch_is_running"),

  onProgress: (cb: (p: LaunchProgress) => void): Promise<UnlistenFn> =>
    listen<LaunchProgress>("launch://progress", (e) => cb(e.payload)),

  onExit: (cb: (e: LaunchExit) => void): Promise<UnlistenFn> =>
    listen<LaunchExit>("launch://exit", (e) => cb(e.payload)),

  onLog: (cb: (line: string) => void): Promise<UnlistenFn> =>
    listen<string>("launch://log", (e) => cb(e.payload)),
};

// ---- Paths / misc ----

export const paths = {
  defaultGameDir: () => invoke<string>("paths_default_game_dir"),
};
