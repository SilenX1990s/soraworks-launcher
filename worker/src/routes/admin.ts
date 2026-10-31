import { Hono } from "hono";
import type { Env } from "../types";
import type { InstanceRow } from "../types";
import { mapInstance, now, requireAdmin } from "../lib";

const admin = new Hono<{ Bindings: Env }>();

admin.use("*", requireAdmin);

// ---- Overview ----

admin.get("/overview", async (c) => {
  const db = c.env.DB;
  const [instances, members, pendingCrash] = await Promise.all([
    db.prepare("SELECT COUNT(*) AS n FROM instances").first<{ n: number }>(),
    db.prepare("SELECT COUNT(*) AS n FROM members").first<{ n: number }>(),
    db.prepare("SELECT COUNT(*) AS n FROM crash_reports WHERE status = 'new'").first<{ n: number }>(),
  ]);
  const wl = await db.prepare("SELECT COUNT(*) AS n FROM whitelist").first<{ n: number }>();
  return c.json({
    instances: instances?.n ?? 0,
    members: members?.n ?? 0,
    whitelisted: wl?.n ?? 0,
    pendingCrashes: pendingCrash?.n ?? 0,
  });
});

// ---- Instances CRUD ----

admin.get("/instances", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM instances ORDER BY sort_order, created_at",
  ).all<InstanceRow>();
  return c.json((results ?? []).map(mapInstance));
});

admin.get("/instances/:id", async (c) => {
  const row = await c.env.DB.prepare("SELECT * FROM instances WHERE id = ?")
    .bind(c.req.param("id"))
    .first<InstanceRow>();
  if (!row) return c.json({ error: "not_found" }, 404);
  return c.json(mapInstance(row));
});

interface InstanceInput {
  id?: string;
  name?: string;
  description?: string;
  gameVersion?: string;
  loader?: { type?: string; version?: string };
  memoryMb?: number;
  jvmArgs?: string[];
  javaMajor?: number;
  allowJavaOverride?: boolean;
  serverAddress?: string;
  accentColor?: string;
  accentColor2?: string;
  cover?: string;
  iconImage?: string;
  background?: unknown;
  links?: Record<string, string>;
  access?: string;
  status?: string;
  contentVersion?: string;
  sortOrder?: number;
}

function slugify(name: string): string {
  return (
    name
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/(^-|-$)/g, "") || "instance"
  );
}

admin.post("/instances", async (c) => {
  const body = (await c.req.json()) as InstanceInput;
  if (!body.name || !body.gameVersion) {
    return c.json({ error: "name and gameVersion are required" }, 400);
  }
  const id = body.id || `${slugify(body.name)}-${Math.random().toString(36).slice(2, 6)}`;
  const t = now();
  await c.env.DB.prepare(
    `INSERT INTO instances
      (id, name, description, game_version, loader_type, loader_version, memory_mb,
       jvm_args, java_major, allow_java_override, server_address, accent, accent2,
       cover, icon_image, background, links, access, status, content_version,
       sort_order, created_at, updated_at)
     VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)`,
  )
    .bind(
      id,
      body.name,
      body.description ?? null,
      body.gameVersion,
      body.loader?.type ?? "vanilla",
      body.loader?.version ?? null,
      body.memoryMb ?? 0,
      JSON.stringify(body.jvmArgs ?? []),
      body.javaMajor ?? 17,
      body.allowJavaOverride === false ? 0 : 1,
      body.serverAddress ?? null,
      body.accentColor ?? null,
      body.accentColor2 ?? null,
      body.cover ?? null,
      body.iconImage ?? null,
      body.background ? JSON.stringify(body.background) : null,
      body.links ? JSON.stringify(body.links) : null,
      body.access ?? "whitelist",
      body.status ?? "draft",
      body.contentVersion ?? "1.0.0",
      body.sortOrder ?? 0,
      t,
      t,
    )
    .run();
  const row = await c.env.DB.prepare("SELECT * FROM instances WHERE id = ?")
    .bind(id)
    .first<InstanceRow>();
  return c.json(mapInstance(row!), 201);
});

admin.patch("/instances/:id", async (c) => {
  const id = c.req.param("id");
  const existing = await c.env.DB.prepare("SELECT * FROM instances WHERE id = ?")
    .bind(id)
    .first<InstanceRow>();
  if (!existing) return c.json({ error: "not_found" }, 404);
  const b = (await c.req.json()) as InstanceInput;

  // Map camelCase input onto columns; only set what was provided.
  const cols: Record<string, unknown> = {
    name: b.name,
    description: b.description,
    game_version: b.gameVersion,
    loader_type: b.loader?.type,
    loader_version: b.loader?.version,
    memory_mb: b.memoryMb,
    jvm_args: b.jvmArgs ? JSON.stringify(b.jvmArgs) : undefined,
    java_major: b.javaMajor,
    allow_java_override:
      b.allowJavaOverride === undefined ? undefined : b.allowJavaOverride ? 1 : 0,
    server_address: b.serverAddress,
    accent: b.accentColor,
    accent2: b.accentColor2,
    cover: b.cover,
    icon_image: b.iconImage,
    background: b.background ? JSON.stringify(b.background) : undefined,
    links: b.links ? JSON.stringify(b.links) : undefined,
    access: b.access,
    status: b.status,
    content_version: b.contentVersion,
    sort_order: b.sortOrder,
  };
  const entries = Object.entries(cols).filter(([, v]) => v !== undefined);
  if (entries.length) {
    const setSql = entries.map(([k]) => `${k} = ?`).join(", ");
    const values = entries.map(([, v]) => v as string | number);
    await c.env.DB.prepare(
      `UPDATE instances SET ${setSql}, updated_at = ? WHERE id = ?`,
    )
      .bind(...values, now(), id)
      .run();
  }
  const row = await c.env.DB.prepare("SELECT * FROM instances WHERE id = ?")
    .bind(id)
    .first<InstanceRow>();
  return c.json(mapInstance(row!));
});

admin.delete("/instances/:id", async (c) => {
  await c.env.DB.prepare("DELETE FROM instances WHERE id = ?").bind(c.req.param("id")).run();
  return c.json({ ok: true });
});

// ---- Whitelist ----

admin.get("/instances/:id/whitelist", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM whitelist WHERE instance_id = ? ORDER BY created_at DESC",
  )
    .bind(c.req.param("id"))
    .all();
  return c.json(results ?? []);
});

admin.post("/instances/:id/whitelist", async (c) => {
  const id = c.req.param("id");
  const { uuid, name, status } = (await c.req.json()) as {
    uuid: string;
    name: string;
    status?: string;
  };
  if (!uuid || !name) return c.json({ error: "uuid and name required" }, 400);
  await c.env.DB.prepare(
    `INSERT OR REPLACE INTO whitelist (instance_id, uuid, name, status, created_at)
     VALUES (?,?,?,?,?)`,
  )
    .bind(id, uuid, name, status ?? "approved", now())
    .run();
  // NOTE: the RCON `whitelist add <name>` sync happens from the server agent
  // (later phase); here we only record intent.
  return c.json({ ok: true });
});

admin.delete("/instances/:id/whitelist/:uuid", async (c) => {
  await c.env.DB.prepare("DELETE FROM whitelist WHERE instance_id = ? AND uuid = ?")
    .bind(c.req.param("id"), c.req.param("uuid"))
    .run();
  return c.json({ ok: true });
});

// ---- Members ----

admin.get("/members", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM members ORDER BY created_at DESC",
  ).all();
  return c.json(results ?? []);
});

admin.post("/members", async (c) => {
  const { uuid, name, role } = (await c.req.json()) as {
    uuid: string;
    name: string;
    role?: string;
  };
  if (!uuid || !name) return c.json({ error: "uuid and name required" }, 400);
  await c.env.DB.prepare(
    "INSERT OR REPLACE INTO members (uuid, name, role, created_at) VALUES (?,?,?,?)",
  )
    .bind(uuid, name, role ?? "player", now())
    .run();
  return c.json({ ok: true });
});

admin.delete("/members/:uuid", async (c) => {
  await c.env.DB.prepare("DELETE FROM members WHERE uuid = ?").bind(c.req.param("uuid")).run();
  return c.json({ ok: true });
});

// ---- News ----

admin.get("/news", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM news ORDER BY pinned DESC, created_at DESC",
  ).all();
  return c.json(results ?? []);
});

admin.post("/news", async (c) => {
  const { instanceId, title, body, pinned } = (await c.req.json()) as {
    instanceId?: string;
    title: string;
    body: string;
    pinned?: boolean;
  };
  if (!title || !body) return c.json({ error: "title and body required" }, 400);
  await c.env.DB.prepare(
    "INSERT INTO news (instance_id, title, body, pinned, created_at) VALUES (?,?,?,?,?)",
  )
    .bind(instanceId ?? null, title, body, pinned ? 1 : 0, now())
    .run();
  return c.json({ ok: true });
});

admin.delete("/news/:id", async (c) => {
  await c.env.DB.prepare("DELETE FROM news WHERE id = ?").bind(c.req.param("id")).run();
  return c.json({ ok: true });
});

// ---- Settings (key/value: R2 config, discord webhook, …) ----

admin.get("/settings", async (c) => {
  const { results } = await c.env.DB.prepare("SELECT key, value FROM settings").all<{
    key: string;
    value: string;
  }>();
  const out: Record<string, string> = {};
  for (const row of results ?? []) out[row.key] = row.value;
  return c.json(out);
});

admin.put("/settings", async (c) => {
  const body = (await c.req.json()) as Record<string, string>;
  const stmts = Object.entries(body).map(([k, v]) =>
    c.env.DB.prepare("INSERT OR REPLACE INTO settings (key, value) VALUES (?,?)").bind(k, String(v)),
  );
  if (stmts.length) await c.env.DB.batch(stmts);
  return c.json({ ok: true });
});

export default admin;
