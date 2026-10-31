import { writable, get } from "svelte/store";
import {
  type Theme,
  type ThemeFile,
  CURRENT_THEME_VERSION,
  DEFAULT_THEME_ID,
  cloneTheme,
  getPreset,
} from "../theme/presets";

const STORAGE_KEY = "soraworks.theme";

/** Turn "#rrggbb" into "r, g, b" so we can build rgba() in CSS. */
function hexToRgb(hex: string): string {
  const clean = hex.replace("#", "");
  const full =
    clean.length === 3
      ? clean
          .split("")
          .map((c) => c + c)
          .join("")
      : clean;
  const num = parseInt(full, 16);
  if (Number.isNaN(num)) return "0, 0, 0";
  const r = (num >> 16) & 255;
  const g = (num >> 8) & 255;
  const b = num & 255;
  return `${r}, ${g}, ${b}`;
}

/**
 * Write a theme to the document as CSS custom properties.
 * Every component styles itself with these variables, so a change here
 * re-themes the whole UI instantly with no reload.
 */
export function applyTheme(theme: Theme) {
  if (typeof document === "undefined") return;
  const r = document.documentElement.style;
  const c = theme.colors;

  r.setProperty("--accent", c.accent);
  r.setProperty("--accent-rgb", hexToRgb(c.accent));
  r.setProperty("--accent2", c.accent2);
  r.setProperty("--bg", c.bg);
  r.setProperty("--text", c.text);
  r.setProperty("--text-muted", c.textMuted);
  r.setProperty("--border", c.border);

  // Surfaces honor the translucency slider.
  const op = Math.max(0, Math.min(0.4, theme.surfaceOpacity));
  const alpha = (1 - op).toFixed(3);
  r.setProperty("--surface", `rgba(${hexToRgb(c.surface)}, ${alpha})`);
  r.setProperty("--surface-2", `rgba(${hexToRgb(c.surface2)}, ${alpha})`);
  r.setProperty("--surface-solid", c.surface);

  r.setProperty("--radius", `${theme.radius}px`);
  r.setProperty("--radius-sm", `${Math.max(2, theme.radius - 4)}px`);
  r.setProperty("--radius-lg", `${theme.radius + 6}px`);
  r.setProperty("--font-scale", String(theme.fontScale));

  document.documentElement.dataset.mode = theme.mode;
  r.setProperty("color-scheme", theme.mode);
}

function loadInitial(): Theme {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as Theme;
      if (parsed && parsed.colors) return parsed;
    }
  } catch {
    /* ignore corrupt storage */
  }
  return cloneTheme(getPreset(DEFAULT_THEME_ID)!);
}

const initial = loadInitial();

/** The theme currently applied to the UI. */
export const theme = writable<Theme>(initial);

theme.subscribe((value) => {
  applyTheme(value);
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
  } catch {
    /* ignore */
  }
});

/** Replace the whole theme (e.g. when a preset is chosen). */
export function setTheme(next: Theme) {
  theme.set(cloneTheme(next));
}

/** Patch one or more fields of the active theme (live editing). */
export function updateTheme(patch: Partial<Theme>) {
  theme.update((cur) => ({ ...cur, ...patch, colors: { ...cur.colors, ...patch.colors } }));
}

/** Patch a single color. */
export function setColor(key: keyof Theme["colors"], value: string) {
  theme.update((cur) => ({ ...cur, colors: { ...cur.colors, [key]: value } }));
}

/** Serialize the current theme into a shareable file payload. */
export function exportTheme(): string {
  const file: ThemeFile = {
    format: "soraworks-theme",
    version: CURRENT_THEME_VERSION,
    theme: get(theme),
  };
  return JSON.stringify(file, null, 2);
}

/** Load a theme from a previously exported file payload. Throws on bad input. */
export function importTheme(json: string): Theme {
  const parsed = JSON.parse(json) as Partial<ThemeFile>;
  if (parsed.format !== "soraworks-theme" || !parsed.theme || !parsed.theme.colors) {
    throw new Error("Not a valid SoraWorks theme file");
  }
  const next = cloneTheme(parsed.theme);
  setTheme(next);
  return next;
}
