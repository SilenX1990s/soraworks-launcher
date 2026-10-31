import { writable } from "svelte/store";

export interface Settings {
  /** Turn off video backgrounds and heavy effects. */
  lowSpec: boolean;
  /** Play UI sound effects. */
  uiSounds: boolean;
  /** Max heap for the game, in MB (0 = auto based on system RAM). */
  maxMemoryMb: number;
  /** Game data folder; empty string means "use the platform default". */
  gameDir: string;
  /** Show snapshots in the version picker. */
  showSnapshots: boolean;
}

const STORAGE_KEY = "soraworks.settings";

const DEFAULTS: Settings = {
  lowSpec: false,
  uiSounds: true,
  maxMemoryMb: 0,
  gameDir: "",
  showSnapshots: false,
};

function load(): Settings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return { ...DEFAULTS, ...(JSON.parse(raw) as Partial<Settings>) };
  } catch {
    /* ignore */
  }
  return { ...DEFAULTS };
}

export const settings = writable<Settings>(load());

settings.subscribe((value) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
  } catch {
    /* ignore */
  }
});

export function patchSettings(patch: Partial<Settings>) {
  settings.update((cur) => ({ ...cur, ...patch }));
}
