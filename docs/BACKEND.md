# Backend (Cloudflare) + Dashboard

Two projects alongside the launcher:

- `worker/` — the API (Cloudflare Worker) with **D1** (database) and **R2**
  (file storage). Framework: [Hono](https://hono.dev).
- `dashboard/` — the owner web console (Vite + Svelte SPA) that talks to the API.

## Run it locally (no Cloudflare account needed)

```bash
# 1. API — uses a local, persisted D1/R2 via miniflare
cd worker
npm install
npm run db:init:local      # create tables in the local D1
npm run db:seed:local      # optional sample data
npm run dev                # http://127.0.0.1:8787
```

```bash
# 2. Dashboard
cd dashboard
npm install
npm run dev                # http://localhost:5173
```

Open the dashboard, enter the API base `http://127.0.0.1:8787`, leave the admin
token empty (dev mode), and connect.

## Deploy (your Cloudflare account)

```bash
cd worker
npx wrangler login
npx wrangler d1 create soraworks           # paste the id into wrangler.toml
npx wrangler r2 bucket create soraworks-files
npm run db:init:remote
npx wrangler secret put ADMIN_TOKEN        # owner auth for the dashboard
npm run deploy
```

Build and host the dashboard (e.g. Cloudflare Pages) from `dashboard/` and point
it at your Worker URL.

## API surface

Public (launcher):
- `GET /api/instances` — published instances
- `GET /api/instances/:id/manifest` — modpack file list (hash-diff sync)
- `POST /api/download-token` — issues a short-lived token (enforces whitelist)
- `GET /api/download/:id/*` — download a file via a valid token (Worker proxies R2)
- `POST /api/crash` — upload a crash report (with consent)

Admin (dashboard, `Authorization: Bearer <ADMIN_TOKEN>`):
- `/api/admin/overview`, `/api/admin/instances` (CRUD),
  `/api/admin/instances/:id/whitelist`, `/api/admin/members`,
  `/api/admin/news`, `/api/admin/settings`

## Security notes (honest)

- In dev, the admin API is **open** when `ADMIN_TOKEN` is unset. Always set it
  in production.
- `download-token` currently trusts the UUID the launcher sends. Binding tokens
  to a **verified** Minecraft identity (and the per-user signed license) is the
  mod-protection phase; until then treat downloads as "whitelist-gated", not
  cryptographically bound.
- R2 "bring-your-own" keys entered in the dashboard are stored in D1 settings;
  move them to Worker **secrets** before any real deployment.
- No layer is 100%. See `docs/ROADMAP.md` phase 5 for the protection plan and
  what each layer does and does not stop.
