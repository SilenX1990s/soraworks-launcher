import type { Context, Next } from "hono";
import type { Env, InstanceRow } from "./types";

/** Admin auth: require `Authorization: Bearer <ADMIN_TOKEN>`.
 *  If ADMIN_TOKEN is not configured, allow (local dev only). */
export async function requireAdmin(c: Context<{ Bindings: Env }>, next: Next) {
  const token = c.env.ADMIN_TOKEN;
  if (!token) {
    // Dev mode — no token set. Fine for `wrangler dev`, never for production.
    return next();
  }
  const auth = c.req.header("Authorization") ?? "";
  const provided = auth.startsWith("Bearer ") ? auth.slice(7) : "";
  if (provided !== token) {
    return c.json({ error: "unauthorized" }, 401);
  }
  return next();
}

function parseJson<T>(value: string | null, fallback: T): T {
  if (!value) return fallback;
  try {
    return JSON.parse(value) as T;
  } catch {
    return fallback;
  }
}

/** Map a DB row to the public instance object the launcher/dashboard consume. */
export function mapInstance(row: InstanceRow) {
  return {
    id: row.id,
    name: row.name,
    description: row.description ?? undefined,
    gameVersion: row.game_version,
    loader: {
      type: row.loader_type,
      version: row.loader_version ?? undefined,
    },
    memoryMb: row.memory_mb,
    jvmArgs: parseJson<string[]>(row.jvm_args, []),
    javaMajor: row.java_major,
    allowJavaOverride: row.allow_java_override === 1,
    serverAddress: row.server_address ?? undefined,
    accentColor: row.accent ?? undefined,
    accentColor2: row.accent2 ?? undefined,
    cover: row.cover ?? undefined,
    iconImage: row.icon_image ?? undefined,
    background: parseJson<unknown>(row.background, undefined),
    links: parseJson<Record<string, string>>(row.links, {}),
    access: row.access,
    status: row.status,
    contentVersion: row.content_version,
    playersOnline: 0,
  };
}

/** Generate a random opaque token. */
export function randomToken(bytes = 24): string {
  const arr = new Uint8Array(bytes);
  crypto.getRandomValues(arr);
  return [...arr].map((b) => b.toString(16).padStart(2, "0")).join("");
}

export function now(): number {
  return Math.floor(Date.now() / 1000);
}
