import { writable, derived, get } from "svelte/store";
import { BUNDLED_INSTANCES, type Instance } from "../instances";
import { updateTheme } from "./theme";

const ACTIVE_KEY = "soraworks.activeInstance";
const JAVA_OVERRIDE_KEY = "soraworks.javaOverrides";

/** All instances available to the player (bundled now; synced from R2 later). */
export const instances = writable<Instance[]>(BUNDLED_INSTANCES);

function loadActiveId(): string {
  try {
    const saved = localStorage.getItem(ACTIVE_KEY);
    if (saved && BUNDLED_INSTANCES.some((i) => i.id === saved)) return saved;
  } catch {
    /* ignore */
  }
  return BUNDLED_INSTANCES[0]?.id ?? "";
}

export const activeInstanceId = writable<string>(loadActiveId());

activeInstanceId.subscribe((id) => {
  try {
    localStorage.setItem(ACTIVE_KEY, id);
  } catch {
    /* ignore */
  }
});

export const activeInstance = derived(
  [instances, activeInstanceId],
  ([$instances, $id]) => $instances.find((i) => i.id === $id) ?? $instances[0] ?? null,
);

// --- Per-instance Java override (the one thing a player may change) ---

function loadOverrides(): Record<string, number> {
  try {
    const raw = localStorage.getItem(JAVA_OVERRIDE_KEY);
    if (raw) return JSON.parse(raw) as Record<string, number>;
  } catch {
    /* ignore */
  }
  return {};
}

export const javaOverrides = writable<Record<string, number>>(loadOverrides());

javaOverrides.subscribe((map) => {
  try {
    localStorage.setItem(JAVA_OVERRIDE_KEY, JSON.stringify(map));
  } catch {
    /* ignore */
  }
});

/** The Java major to launch an instance with: the player's override, else the
 *  instance's recommended default. */
export function effectiveJavaMajor(inst: Instance): number {
  const map = get(javaOverrides);
  return map[inst.id] ?? inst.javaMajor;
}

export function setJavaOverride(instanceId: string, major: number) {
  javaOverrides.update((m) => ({ ...m, [instanceId]: major }));
}

export function clearJavaOverride(instanceId: string) {
  javaOverrides.update((m) => {
    const next = { ...m };
    delete next[instanceId];
    return next;
  });
}

export function selectInstance(id: string) {
  activeInstanceId.set(id);
}

/** Apply an instance's branding (accent colors) to the live theme. */
export function applyInstanceBranding(inst: Instance) {
  const patch: Record<string, string> = {};
  if (inst.accentColor) patch.accent = inst.accentColor;
  if (inst.accentColor2) patch.accent2 = inst.accentColor2;
  if (Object.keys(patch).length) {
    updateTheme({ colors: patch as never });
  }
}
