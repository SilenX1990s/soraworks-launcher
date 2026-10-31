export interface Env {
  DB: D1Database;
  BUCKET: R2Bucket;
  APP_NAME: string;
  /** Owner/admin bearer token for the dashboard. If unset, the API runs in
   *  open "dev" mode (local only) — always set this in production. */
  ADMIN_TOKEN?: string;
  /** Discord webhook for crash/alert notifications (optional). */
  DISCORD_WEBHOOK?: string;
}

/** DB row as stored (snake_case). */
export interface InstanceRow {
  id: string;
  name: string;
  description: string | null;
  game_version: string;
  loader_type: string;
  loader_version: string | null;
  memory_mb: number;
  jvm_args: string;
  java_major: number;
  allow_java_override: number;
  server_address: string | null;
  accent: string | null;
  accent2: string | null;
  cover: string | null;
  icon_image: string | null;
  background: string | null;
  links: string | null;
  access: string;
  status: string;
  content_version: string;
  sort_order: number;
  created_at: number;
  updated_at: number;
}
