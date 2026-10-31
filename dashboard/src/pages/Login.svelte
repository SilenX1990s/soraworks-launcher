<script lang="ts">
  import { apiBase, adminToken, connected, toast } from "../lib/store";
  import { api } from "../lib/api";

  let base = $state($apiBase);
  let token = $state($adminToken);
  let busy = $state(false);
  let err = $state("");

  async function connect() {
    busy = true;
    err = "";
    apiBase.set(base.trim());
    adminToken.set(token.trim());
    try {
      await api.health();
      // Verify the token (if the API requires one, this 401s on a bad token).
      await api.overview();
      connected.set(true);
      toast("เชื่อมต่อสำเร็จ");
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    } finally {
      busy = false;
    }
  }
</script>

<div class="screen">
  <div class="box card">
    <div class="logo"><div class="mark">S</div></div>
    <h2>SoraWorks Dashboard</h2>
    <p class="muted">เชื่อมต่อกับ Worker API ของคุณเพื่อเริ่มจัดการ</p>

    <label>API base URL<input bind:value={base} placeholder="http://127.0.0.1:8787" /></label>

    <label>Admin token<input bind:value={token} type="password" placeholder="(ปล่อยว่างได้ตอน dev)" /></label>

    {#if err}<div class="err">{err}</div>{/if}

    <button class="primary" onclick={connect} disabled={busy}>
      {busy ? "กำลังเชื่อมต่อ…" : "เชื่อมต่อ"}
    </button>
    <p class="muted hint">
      ตอน dev รัน <code>npm run dev</code> ในโฟลเดอร์ <code>worker/</code> (ปกติพอร์ต 8787)
      แล้วปล่อย token ว่างได้ ส่วนโปรดักชันตั้ง <code>ADMIN_TOKEN</code> ด้วย
      <code>wrangler secret put ADMIN_TOKEN</code>
    </p>
  </div>
</div>

<style>
  .screen { min-height: 100vh; display: grid; place-items: center; padding: 2rem; }
  .box { width: 100%; max-width: 420px; display: flex; flex-direction: column; gap: 0.6rem; }
  .logo { display: flex; justify-content: center; margin-bottom: 0.4rem; }
  .mark {
    width: 54px; height: 54px; border-radius: 15px; display: grid; place-items: center;
    font-weight: 800; font-size: 1.5rem; color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent2));
  }
  h2 { text-align: center; margin: 0; }
  .box > .muted { text-align: center; margin: 0 0 0.6rem; }
  .box label { display: flex; flex-direction: column; gap: 0.3rem; margin-top: 0.4rem; }
  .err { color: #ff8a8a; font-size: 0.88rem; }
  .hint { font-size: 0.78rem; line-height: 1.5; margin-top: 0.5rem; }
  code { background: var(--surface-2); padding: 0.1em 0.4em; border-radius: 5px; font-size: 0.85em; }
</style>
