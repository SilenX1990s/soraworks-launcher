import { writable, derived } from "svelte/store";
import type { Account } from "../types";
import { auth } from "../tauri";

export const accounts = writable<Account[]>([]);

/** The currently active account, or null if none. */
export const activeAccount = derived(accounts, ($accounts) =>
  $accounts.find((a) => a.active) ?? $accounts[0] ?? null,
);

/** Pull the account list from the backend (reads the OS keychain). */
export async function refreshAccounts(): Promise<void> {
  const list = await auth.list();
  accounts.set(list);
}

export async function removeAccount(id: string): Promise<void> {
  await auth.remove(id);
  await refreshAccounts();
}

export async function setActiveAccount(id: string): Promise<void> {
  await auth.setActive(id);
  await refreshAccounts();
}
