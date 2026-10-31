import { get } from "svelte/store";
import { apiBase, adminToken } from "./store";

async function req<T>(method: string, path: string, body?: unknown): Promise<T> {
  const base = get(apiBase).replace(/\/$/, "");
  const token = get(adminToken);
  const res = await fetch(`${base}${path}`, {
    method,
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(body !== undefined ? { "Content-Type": "application/json" } : {}),
    },
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  const data = text ? JSON.parse(text) : null;
  if (!res.ok) {
    throw new Error((data && data.error) || `HTTP ${res.status}`);
  }
  return data as T;
}

export interface ApiInstance {
  id: string;
  name: string;
  description?: string;
  gameVersion: string;
  loader: { type: string; version?: string };
  memoryMb: number;
  jvmArgs: string[];
  javaMajor: number;
  allowJavaOverride: boolean;
  serverAddress?: string;
  accentColor?: string;
  accentColor2?: string;
  cover?: string;
  iconImage?: string;
  links?: Record<string, string>;
  access: string;
  status: string;
  contentVersion: string;
  playersOnline: number;
}

export const api = {
  health: () => req<{ ok: boolean; app?: string }>("GET", "/health"),
  overview: () =>
    req<{ instances: number; members: number; whitelisted: number; pendingCrashes: number }>(
      "GET",
      "/api/admin/overview",
    ),

  listInstances: () => req<ApiInstance[]>("GET", "/api/admin/instances"),
  getInstance: (id: string) => req<ApiInstance>("GET", `/api/admin/instances/${id}`),
  createInstance: (data: Partial<ApiInstance>) =>
    req<ApiInstance>("POST", "/api/admin/instances", data),
  updateInstance: (id: string, data: Partial<ApiInstance>) =>
    req<ApiInstance>("PATCH", `/api/admin/instances/${id}`, data),
  deleteInstance: (id: string) => req<{ ok: boolean }>("DELETE", `/api/admin/instances/${id}`),

  listWhitelist: (id: string) =>
    req<{ uuid: string; name: string; status: string }[]>(
      "GET",
      `/api/admin/instances/${id}/whitelist`,
    ),
  addWhitelist: (id: string, uuid: string, name: string) =>
    req("POST", `/api/admin/instances/${id}/whitelist`, { uuid, name }),
  removeWhitelist: (id: string, uuid: string) =>
    req("DELETE", `/api/admin/instances/${id}/whitelist/${uuid}`),

  listMembers: () =>
    req<{ uuid: string; name: string; role: string }[]>("GET", "/api/admin/members"),
  addMember: (uuid: string, name: string, role: string) =>
    req("POST", "/api/admin/members", { uuid, name, role }),
  removeMember: (uuid: string) => req("DELETE", `/api/admin/members/${uuid}`),

  getSettings: () => req<Record<string, string>>("GET", "/api/admin/settings"),
  putSettings: (data: Record<string, string>) => req("PUT", "/api/admin/settings", data),
};
