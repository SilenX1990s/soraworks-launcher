-- Optional dev seed data so the dashboard and launcher have something to show.

INSERT OR REPLACE INTO instances
  (id, name, description, game_version, loader_type, loader_version, memory_mb,
   java_major, server_address, accent, accent2, access, status, content_version, sort_order)
VALUES
  ('soraworks-forge-1201', 'SoraWorks', 'เซิร์ฟเวอร์หลัก — Forge 1.20.1',
   '1.20.1', 'forge', '47.3.0', 0, 17, 'play.soraworks.net',
   '#6c8cff', '#9d6cff', 'whitelist', 'published', '1.0.0', 0),
  ('vanilla-1201', 'Vanilla 1.20.1', 'ไม่มีม็อด — ทดสอบการเปิดเกม',
   '1.20.1', 'vanilla', NULL, 2048, 17, NULL,
   '#2fcf7f', '#17b0c0', 'public', 'published', '1.0.0', 1);

INSERT OR REPLACE INTO settings (key, value) VALUES
  ('discord_webhook', ''),
  ('r2_endpoint', ''),
  ('r2_bucket', '');
