<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { toast } from "../lib/store";

  let settings = $state<Record<string, string>>({});

  async function load() {
    try {
      settings = await api.getSettings();
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
  onMount(load);

  async function save() {
    try {
      await api.putSettings(settings);
      toast("บันทึกแล้ว");
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
</script>

<div class="stack">
  <h1>ตั้งค่า</h1>

  <div class="card stack">
    <h3 style="margin:0">การแจ้งเตือน</h3>
    <label>Discord webhook (แจ้งเตือน crash / เหตุการณ์)
      <input bind:value={settings.discord_webhook} placeholder="https://discord.com/api/webhooks/…" />
    </label>
    <div><button class="primary" onclick={save}>บันทึก</button></div>
  </div>
</div>
