import { Hono } from "hono";
import { cors } from "hono/cors";
import type { Env } from "./types";
import admin from "./routes/admin";
import pub from "./routes/public";

const app = new Hono<{ Bindings: Env }>();

// Allow the dashboard (any origin) to call the API; admin routes still require
// the bearer token, so this is safe.
app.use("*", cors({ origin: "*", allowHeaders: ["Authorization", "Content-Type"] }));

app.get("/", (c) => c.json({ app: c.env.APP_NAME, ok: true }));
app.get("/health", (c) => c.json({ ok: true }));

app.route("/api/admin", admin);
app.route("/api", pub);

export default app;
