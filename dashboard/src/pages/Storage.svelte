<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { toast } from "../lib/store";

  // Bring-your-own R2: the dashboard stores the owner's S3-compatible R2 config
  // so instance files can be uploaded directly. Keys are stored server-side.
  let s = $state<Record<string, string>>({});

  async function load() {
    try {
      s = await api.getSettings();
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
  onMount(load);

  async function save() {
    try {
      await api.putSettings({
        r2_endpoint: s.r2_endpoint ?? "",
        r2_bucket: s.r2_bucket ?? "",
        r2_access_key_id: s.r2_access_key_id ?? "",
        r2_secret_access_key: s.r2_secret_access_key ?? "",
      });
      toast("บันทึกการตั้งค่า R2 แล้ว");
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
</script>

<div class="layout">
  <div class="stack main">
    <h1>R2 storage</h1>
    <div class="card stack">
      <h3 style="margin:0">เชื่อมต่อ R2 ของคุณ</h3>
      <p class="muted" style="margin:0">
        กรอกค่าจากแดชบอร์ด Cloudflare กดทดสอบ แล้วบันทึก ไฟล์อินสแตนซ์ของคุณจะเก็บใน bucket ส่วนตัวนี้
      </p>

      <label>ชื่อ (เห็นเฉพาะคุณและทีม)<input bind:value={s.r2_name} placeholder="พื้นที่เก็บหลัก" /></label>

      <div class="lock">🔒 Bucket ส่วนตัวสำหรับไฟล์อินสแตนซ์</div>
      <label>S3 endpoint<input bind:value={s.r2_endpoint} placeholder="https://ACCOUNT_ID.r2.cloudflarestorage.com" /></label>
      <div class="two">
        <label>Access Key ID<input bind:value={s.r2_access_key_id} placeholder="32 ตัวอักษร" /></label>
        <label>Secret Access Key<input type="password" bind:value={s.r2_secret_access_key} placeholder="64 ตัวอักษร" /></label>
      </div>
      <label>Bucket ส่วนตัว<input bind:value={s.r2_bucket} placeholder="my-server-files" /></label>

      <div class="row">
        <button class="ghost" onclick={() => toast("การทดสอบจริงต้องมี endpoint ที่ตั้งค่าแล้ว")}>ทดสอบการเชื่อมต่อ</button>
        <button class="primary" onclick={save}>บันทึก</button>
      </div>
    </div>
  </div>

  <aside class="guide card">
    <h4>ต้องตั้ง</h4>
    <ol>
      <li><strong>สร้าง bucket</strong> ส่วนตัวหนึ่งอัน</li>
      <li><strong>API token: Object Read & Write</strong> จำกัดเฉพาะ bucket ของคุณ — คัดลอก Access Key ID กับ Secret Access Key</li>
      <li><strong>รูปแบบ endpoint</strong>: <code>https://ACCOUNT_ID.r2.cloudflarestorage.com</code> (ไม่ต้องต่อท้ายด้วยชื่อ bucket)</li>
      <li><strong>CORS</strong>: ให้แดชบอร์ดอัปโหลดตรงเข้า bucket ได้</li>
      <li><strong>Public access</strong>: ปิดไว้ — ดาวน์โหลดผ่าน signed URL จาก Worker เท่านั้น</li>
    </ol>
  </aside>
</div>

<style>
  .layout { display: grid; grid-template-columns: 1fr 320px; gap: 1rem; align-items: start; }
  .lock { font-weight: 600; margin-top: 0.3rem; }
  .two { display: grid; grid-template-columns: 1fr 1fr; gap: 0.7rem; }
  .guide { position: sticky; top: 1rem; }
  .guide h4 { margin: 0 0 0.6rem; }
  .guide ol { margin: 0; padding-left: 1.1rem; display: flex; flex-direction: column; gap: 0.6rem; font-size: 0.85rem; color: var(--text-muted); }
  code { background: var(--surface-2); padding: 0.1em 0.35em; border-radius: 5px; font-size: 0.85em; }
  @media (max-width: 900px) { .layout { grid-template-columns: 1fr; } .two { grid-template-columns: 1fr; } }
</style>
