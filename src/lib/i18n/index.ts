import { derived, writable, get } from "svelte/store";
import en from "./en.json";
import th from "./th.json";
import ja from "./ja.json";

export type Locale = "en" | "th" | "ja";

export const LOCALES: { id: Locale; label: string; flag: string }[] = [
  { id: "th", label: "ไทย", flag: "🇹🇭" },
  { id: "ja", label: "日本語", flag: "🇯🇵" },
  { id: "en", label: "English", flag: "🇬🇧" },
];

type Dict = Record<string, unknown>;

const DICTS: Record<Locale, Dict> = { en, th, ja };

const STORAGE_KEY = "soraworks.locale";

function detectInitial(): Locale {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "en" || saved === "th" || saved === "ja") return saved;
  } catch {
    /* localStorage may be unavailable */
  }
  const nav = typeof navigator !== "undefined" ? navigator.language : "en";
  if (nav.startsWith("th")) return "th";
  if (nav.startsWith("ja")) return "ja";
  return "en";
}

/** The currently active locale. Changing it re-renders every `$t(...)` instantly. */
export const locale = writable<Locale>(detectInitial());

locale.subscribe((value) => {
  try {
    localStorage.setItem(STORAGE_KEY, value);
  } catch {
    /* ignore */
  }
  if (typeof document !== "undefined") {
    document.documentElement.lang = value;
  }
});

export function setLocale(next: Locale) {
  locale.set(next);
}

/** Resolve a dotted key path ("account.signIn") against a dictionary. */
function resolve(dict: Dict, key: string): string | undefined {
  const parts = key.split(".");
  let node: unknown = dict;
  for (const part of parts) {
    if (node && typeof node === "object" && part in (node as Dict)) {
      node = (node as Dict)[part];
    } else {
      return undefined;
    }
  }
  return typeof node === "string" ? node : undefined;
}

/** Replace {name} style placeholders. */
function interpolate(template: string, vars?: Record<string, string | number>): string {
  if (!vars) return template;
  return template.replace(/\{(\w+)\}/g, (match, name) =>
    name in vars ? String(vars[name]) : match,
  );
}

/**
 * Reactive translation function.
 * Usage in a component: `{$t("account.signIn")}` or `{$t("sync.progress", { pct: 40 })}`.
 * Falls back to English, then to the raw key, so a missing string is visible but never crashes.
 */
export const t = derived(locale, ($locale) => {
  return (key: string, vars?: Record<string, string | number>): string => {
    const value = resolve(DICTS[$locale], key) ?? resolve(DICTS.en, key) ?? key;
    return interpolate(value, vars);
  };
});

/** Non-reactive one-off lookup, for use outside components. */
export function translate(key: string, vars?: Record<string, string | number>): string {
  const $locale = get(locale);
  const value = resolve(DICTS[$locale], key) ?? resolve(DICTS.en, key) ?? key;
  return interpolate(value, vars);
}
