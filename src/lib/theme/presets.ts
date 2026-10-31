/**
 * Theme presets. A theme is a plain, serializable object so it can be
 * exported to / imported from a file and shared between users.
 */

export type ThemeMode = "light" | "dark";

export interface ThemeColors {
  /** Primary brand / action color. */
  accent: string;
  /** Secondary accent, used for gradients and highlights. */
  accent2: string;
  /** Window background. */
  bg: string;
  /** Panels and cards drawn on top of the background. */
  surface: string;
  /** A slightly raised surface (inputs, hovered rows). */
  surface2: string;
  /** Primary text. */
  text: string;
  /** Muted / secondary text. */
  textMuted: string;
  /** Borders and separators. */
  border: string;
}

export interface Theme {
  /** Stable id, also used as the storage key for a custom theme. */
  id: string;
  /** Display name (not translated — chosen by whoever made the theme). */
  name: string;
  mode: ThemeMode;
  colors: ThemeColors;
  /** Base corner radius in px (buttons/cards scale from this). */
  radius: number;
  /** Surface translucency, 0 (opaque) … 0.4 (very see-through). */
  surfaceOpacity: number;
  /** Multiplier applied to the base font size, 0.85 … 1.3. */
  fontScale: number;
}

export const CURRENT_THEME_VERSION = 1;

export interface ThemeFile {
  format: "soraworks-theme";
  version: number;
  theme: Theme;
}

const sora: Theme = {
  id: "sora-dark",
  name: "SoraWorks (Dark)",
  mode: "dark",
  colors: {
    accent: "#6c8cff",
    accent2: "#9d6cff",
    bg: "#0e1016",
    surface: "#171a22",
    surface2: "#202532",
    text: "#eef1f8",
    textMuted: "#9aa3b7",
    border: "#2a3040",
  },
  radius: 12,
  surfaceOpacity: 0,
  fontScale: 1,
};

const soraLight: Theme = {
  id: "sora-light",
  name: "SoraWorks (Light)",
  mode: "light",
  colors: {
    accent: "#3b5bdb",
    accent2: "#7048e8",
    bg: "#f4f6fb",
    surface: "#ffffff",
    surface2: "#eef1f8",
    text: "#141722",
    textMuted: "#5a6478",
    border: "#d9deea",
  },
  radius: 12,
  surfaceOpacity: 0,
  fontScale: 1,
};

const emerald: Theme = {
  id: "emerald",
  name: "Emerald",
  mode: "dark",
  colors: {
    accent: "#2fcf7f",
    accent2: "#17b0c0",
    bg: "#0c1411",
    surface: "#122019",
    surface2: "#193024",
    text: "#eafaf1",
    textMuted: "#8fb6a4",
    border: "#1f3a2c",
  },
  radius: 10,
  surfaceOpacity: 0,
  fontScale: 1,
};

const sakura: Theme = {
  id: "sakura",
  name: "Sakura",
  mode: "light",
  colors: {
    accent: "#e8609b",
    accent2: "#f0883e",
    bg: "#fff6f9",
    surface: "#ffffff",
    surface2: "#ffeaf1",
    text: "#2a1c22",
    textMuted: "#8a6b74",
    border: "#f3d2de",
  },
  radius: 16,
  surfaceOpacity: 0,
  fontScale: 1,
};

const midnight: Theme = {
  id: "midnight",
  name: "Midnight Glass",
  mode: "dark",
  colors: {
    accent: "#58a6ff",
    accent2: "#bc8cff",
    bg: "#05070d",
    surface: "#0d1220",
    surface2: "#141b2e",
    text: "#e6edf7",
    textMuted: "#8793ab",
    border: "#1b2336",
  },
  radius: 14,
  surfaceOpacity: 0.12,
  fontScale: 1,
};

export const PRESETS: Theme[] = [sora, soraLight, emerald, sakura, midnight];

export const DEFAULT_THEME_ID = sora.id;

export function getPreset(id: string): Theme | undefined {
  return PRESETS.find((p) => p.id === id);
}

export function cloneTheme(theme: Theme): Theme {
  return { ...theme, colors: { ...theme.colors } };
}
