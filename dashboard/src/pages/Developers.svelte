<script lang="ts">
  import { apiBase, adminToken, connected } from "../lib/store";

  function disconnect() {
    connected.set(false);
  }
</script>

<div class="stack">
  <h1>นักพัฒนา</h1>

  <div class="card stack">
    <h3 style="margin:0">การเชื่อมต่อ API</h3>
    <label>API base URL<input bind:value={$apiBase} /></label>
    <label>Admin token<input type="password" bind:value={$adminToken} /></label>
    <p class="muted small">
      เก็บไว้ในเบราว์เซอร์นี้เท่านั้น ตั้ง token ฝั่งเซิร์ฟเวอร์ด้วย
      <code>wrangler secret put ADMIN_TOKEN</code>
    </p>
    <div><button class="ghost" onclick={disconnect}>ตัดการเชื่อมต่อ</button></div>
  </div>

  <div class="card stack">
    <h3 style="margin:0">Endpoints</h3>
    <table>
      <tbody>
        <tr><td class="mono">GET /api/instances</td><td class="muted">รายการอินสแตนซ์ (ลันเชอร์)</td></tr>
        <tr><td class="mono">GET /api/instances/:id/manifest</td><td class="muted">manifest ไฟล์ม็อดแพ็ก</td></tr>
        <tr><td class="mono">POST /api/download-token</td><td class="muted">ขอ token ดาวน์โหลด (เช็ค whitelist)</td></tr>
        <tr><td class="mono">GET /api/download/:id/*</td><td class="muted">ดาวน์โหลดไฟล์ผ่าน token</td></tr>
        <tr><td class="mono">POST /api/crash</td><td class="muted">อัปโหลด crash report</td></tr>
        <tr><td class="mono">/api/admin/*</td><td class="muted">ต้องมี Bearer ADMIN_TOKEN</td></tr>
      </tbody>
    </table>
  </div>
</div>

<style>
  .mono { font-family: ui-monospace, monospace; font-size: 0.82rem; }
  .small { font-size: 0.82rem; }
  code { background: var(--surface-2); padding: 0.1em 0.35em; border-radius: 5px; }
</style>
