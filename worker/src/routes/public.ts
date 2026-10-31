import { Hono } from "hono";
import type { Env, InstanceRow } from "../types";
import { mapInstance, now, randomToken } from "../lib";

const pub = new Hono<{ Bindings: Env }>();

// Published instances for the launcher.
pub.get("/instances", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM instances WHERE status = 'published' ORDER BY sort_order, created_at",
  ).all<InstanceRow>();
  return c.json((results ?? []).map(mapInstance));
});

// File manifest for an instance (for hash-diff modpack sync).
pub.get("/instances/:id/manifest", async (c) => {
  const id = c.req.param("id");
  const { results } = await c.env.DB.prepare(
    "SELECT path, sha1, size, owned FROM instance_files WHERE instance_id = ?",
  )
    .bind(id)
    .all();
  const inst = await c.env.DB.prepare("SELECT content_version FROM instances WHERE id = ?")
    .bind(id)
    .first<{ content_version: string }>();
  return c.json({
    instanceId: id,
    contentVersion: inst?.content_version ?? "1.0.0",
    files: results ?? [],
  });
});

/**
 * Issue a short-lived download token for a player.
 *
 * SECURITY NOTE: in production the player's UUID must be proven with a valid
 * Minecraft/Xbox token (verified server-side), not merely sent by the client.
 * This phase records the intent and enforces the whitelist; token-binding to a
 * verified identity is wired up with the launcher handshake in a later phase.
 */
pub.post("/download-token", async (c) => {
  const { instanceId, uuid, name } = (await c.req.json()) as {
    instanceId: string;
    uuid: string;
    name?: string;
  };
  if (!instanceId || !uuid) return c.json({ error: "instanceId and uuid required" }, 400);

  const inst = await c.env.DB.prepare("SELECT access FROM instances WHERE id = ?")
    .bind(instanceId)
    .first<{ access: string }>();
  if (!inst) return c.json({ error: "not_found" }, 404);

  if (inst.access === "whitelist") {
    const wl = await c.env.DB.prepare(
      "SELECT status FROM whitelist WHERE instance_id = ? AND uuid = ?",
    )
      .bind(instanceId, uuid)
      .first<{ status: string }>();
    if (!wl || wl.status !== "approved") {
      return c.json({ error: "not_whitelisted" }, 403);
    }
  }

  const token = randomToken();
  const expires = now() + 300; // 5 minutes
  await c.env.DB.prepare(
    "INSERT INTO download_tokens (token, instance_id, uuid, expires_at) VALUES (?,?,?,?)",
  )
    .bind(token, instanceId, uuid, expires)
    .run();
  if (name) {
    await c.env.DB.prepare(
      "INSERT OR REPLACE INTO members (uuid, name, role, created_at) VALUES (?,?, COALESCE((SELECT role FROM members WHERE uuid = ?), 'player'), COALESCE((SELECT created_at FROM members WHERE uuid = ?), ?))",
    )
      .bind(uuid, name, uuid, uuid, now())
      .run();
  }
  return c.json({ token, expiresAt: expires });
});

// Download a file via a valid token (Worker proxies the private R2 object).
pub.get("/download/:id/:path{.+}", async (c) => {
  const id = c.req.param("id");
  const path = c.req.param("path");
  const token = c.req.query("token") ?? "";

  const tok = await c.env.DB.prepare(
    "SELECT * FROM download_tokens WHERE token = ? AND instance_id = ?",
  )
    .bind(token, id)
    .first<{ expires_at: number; uses: number; max_uses: number }>();
  if (!tok || tok.expires_at < now() || tok.uses >= tok.max_uses) {
    return c.json({ error: "invalid_token" }, 403);
  }

  const file = await c.env.DB.prepare(
    "SELECT r2_key FROM instance_files WHERE instance_id = ? AND path = ?",
  )
    .bind(id, path)
    .first<{ r2_key: string }>();
  if (!file) return c.json({ error: "not_found" }, 404);

  const object = await c.env.BUCKET.get(file.r2_key);
  if (!object) return c.json({ error: "missing_object" }, 404);

  await c.env.DB.prepare("UPDATE download_tokens SET uses = uses + 1 WHERE token = ?")
    .bind(token)
    .run();

  return new Response(object.body, {
    headers: {
      "Content-Type": "application/octet-stream",
      "Content-Length": String(object.size),
    },
  });
});

// News for the launcher.
pub.get("/news", async (c) => {
  const { results } = await c.env.DB.prepare(
    "SELECT * FROM news WHERE instance_id IS NULL OR instance_id = ? ORDER BY pinned DESC, created_at DESC LIMIT 30",
  )
    .bind(c.req.query("instance") ?? "")
    .all();
  return c.json(results ?? []);
});

// Crash report upload (player consented).
pub.post("/crash", async (c) => {
  const instanceId = c.req.query("instance") ?? null;
  const uuid = c.req.query("uuid") ?? null;
  const text = await c.req.text();
  const key = `crashes/${Date.now()}-${randomToken(6)}.txt`;
  await c.env.BUCKET.put(key, text);
  await c.env.DB.prepare(
    "INSERT INTO crash_reports (instance_id, uuid, r2_key, summary, created_at) VALUES (?,?,?,?,?)",
  )
    .bind(instanceId, uuid, key, text.slice(0, 200), now())
    .run();

  if (c.env.DISCORD_WEBHOOK) {
    await fetch(c.env.DISCORD_WEBHOOK, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        content: `🧩 New crash report for \`${instanceId ?? "unknown"}\``,
      }),
    }).catch(() => {});
  }
  return c.json({ ok: true });
});

export default pub;
