import { writable } from "svelte/store";

const API_KEY = "soraworks.dash.api";
const TOKEN_KEY = "soraworks.dash.token";

function load(key: string, fallback: string): string {
  try {
    return localStorage.getItem(key) ?? fallback;
  } catch {
    return fallback;
  }
}

/** Base URL of the Worker API (e.g. http://127.0.0.1:8787 in dev). */
export const apiBase = writable<string>(load(API_KEY, "http://127.0.0.1:8787"));
/** Admin bearer token (matches the Worker's ADMIN_TOKEN secret). */
export const adminToken = writable<string>(load(TOKEN_KEY, ""));
/** Whether the user has entered connection details. */
export const connected = writable<boolean>(false);

apiBase.subscribe((v) => {
  try {
    localStorage.setItem(API_KEY, v);
  } catch {
    /* ignore */
  }
});
adminToken.subscribe((v) => {
  try {
    localStorage.setItem(TOKEN_KEY, v);
  } catch {
    /* ignore */
  }
});

// --- Toasts ---
export interface Toast {
  id: number;
  message: string;
  kind: "ok" | "error";
}
export const toasts = writable<Toast[]>([]);
let toastId = 0;
export function toast(message: string, kind: "ok" | "error" = "ok") {
  const id = ++toastId;
  toasts.update((t) => [...t, { id, message, kind }]);
  setTimeout(() => toasts.update((t) => t.filter((x) => x.id !== id)), 3500);
}
