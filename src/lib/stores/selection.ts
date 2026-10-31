import { writable } from "svelte/store";

const STORAGE_KEY = "soraworks.selection";

export interface Selection {
  versionId: string;
}

function load(): Selection {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw) as Selection;
  } catch {
    /* ignore */
  }
  return { versionId: "" };
}

export const selection = writable<Selection>(load());

selection.subscribe((value) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
  } catch {
    /* ignore */
  }
});

export function selectVersion(versionId: string) {
  selection.update((s) => ({ ...s, versionId }));
}
