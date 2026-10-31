-- SoraWorks backend schema (Cloudflare D1 / SQLite).

-- Instances: owner-defined presets the launcher shows.
CREATE TABLE IF NOT EXISTS instances (
  id              TEXT PRIMARY KEY,
  name            TEXT NOT NULL,
  description     TEXT,
  game_version    TEXT NOT NULL,
  loader_type     TEXT NOT NULL DEFAULT 'vanilla',
  loader_version  TEXT,
  memory_mb       INTEGER NOT NULL DEFAULT 0,
  jvm_args        TEXT NOT NULL DEFAULT '[]',   -- JSON array
  java_major      INTEGER NOT NULL DEFAULT 17,
  allow_java_override INTEGER NOT NULL DEFAULT 1,
  server_address  TEXT,
  accent          TEXT,
  accent2         TEXT,
  cover           TEXT,     -- image URL or CSS gradient
  icon_image      TEXT,
  background      TEXT,     -- JSON InstanceBackground
  links           TEXT,     -- JSON {discord,website,...}
  -- access: 'public' (anyone signed in) or 'whitelist' (UUID must be approved)
  access          TEXT NOT NULL DEFAULT 'whitelist',
  -- publish state: 'draft' | 'published'
  status          TEXT NOT NULL DEFAULT 'draft',
  content_version TEXT NOT NULL DEFAULT '1.0.0', -- modpack content version
  sort_order      INTEGER NOT NULL DEFAULT 0,
  created_at      INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at      INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Files that make up an instance's modpack (for hash-diff sync from R2).
CREATE TABLE IF NOT EXISTS instance_files (
  instance_id  TEXT NOT NULL,
  path         TEXT NOT NULL,          -- relative path in the game dir
  sha1         TEXT NOT NULL,
  size         INTEGER NOT NULL,
  r2_key       TEXT NOT NULL,          -- object key in the R2 bucket
  owned        INTEGER NOT NULL DEFAULT 0, -- 1 = our own (protected) mod
  PRIMARY KEY (instance_id, path),
  FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE
);

-- Members / allowed players (by Minecraft UUID).
CREATE TABLE IF NOT EXISTS members (
  uuid        TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  role        TEXT NOT NULL DEFAULT 'player', -- player | mod | admin
  note        TEXT,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Per-instance whitelist (two-way synced with the server via RCON later).
CREATE TABLE IF NOT EXISTS whitelist (
  instance_id TEXT NOT NULL,
  uuid        TEXT NOT NULL,
  name        TEXT NOT NULL,
  status      TEXT NOT NULL DEFAULT 'approved', -- approved | pending | revoked
  device_limit INTEGER NOT NULL DEFAULT 0,      -- 0 = unlimited
  added_by    TEXT,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch()),
  PRIMARY KEY (instance_id, uuid),
  FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE
);

-- News / announcements shown in the launcher.
CREATE TABLE IF NOT EXISTS news (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  instance_id TEXT,                 -- NULL = global
  title       TEXT NOT NULL,
  body        TEXT NOT NULL,
  pinned      INTEGER NOT NULL DEFAULT 0,
  created_at  INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Crash reports uploaded by players (with consent).
CREATE TABLE IF NOT EXISTS crash_reports (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  instance_id TEXT,
  uuid        TEXT,
  r2_key      TEXT,
  summary     TEXT,
  cause       TEXT,          -- rule/AI classified cause
  status      TEXT NOT NULL DEFAULT 'new', -- new | triaged | resolved
  created_at  INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Short-lived download tokens (signed, per user+instance).
CREATE TABLE IF NOT EXISTS download_tokens (
  token       TEXT PRIMARY KEY,
  instance_id TEXT NOT NULL,
  uuid        TEXT NOT NULL,
  expires_at  INTEGER NOT NULL,
  uses        INTEGER NOT NULL DEFAULT 0,
  max_uses    INTEGER NOT NULL DEFAULT 50
);

-- Key/value settings (R2 bring-your-own config, Discord webhook, etc.).
CREATE TABLE IF NOT EXISTS settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_news_instance ON news(instance_id);
CREATE INDEX IF NOT EXISTS idx_crash_instance ON crash_reports(instance_id);
CREATE INDEX IF NOT EXISTS idx_dltok_expires ON download_tokens(expires_at);
