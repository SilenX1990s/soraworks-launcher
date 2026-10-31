<script lang="ts">
  import { onMount } from "svelte";
  import { api, type ApiInstance } from "../lib/api";
  import { toast } from "../lib/store";

  let instances = $state<ApiInstance[]>([]);
  let creating = $state(false);

  // New instance form.
  let form = $state({
    name: "",
    gameVersion: "1.20.1",
    loaderType: "forge",
    loaderVersion: "",
    javaMajor: 17,
    memoryMb: 0,
    access: "whitelist",
  });

  async function load() {
    try {
      instances = await api.listInstances();
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
  onMount(load);

  async function create() {
    try {
      await api.createInstance({
        name: form.name,
        gameVersion: form.gameVersion,
        loader: { type: form.loaderType, version: form.loaderVersion || undefined },
        javaMajor: Number(form.javaMajor),
        memoryMb: Number(form.memoryMb),
        access: form.access,
      });
      toast("สร้างอินสแตนซ์แล้ว");
      creating = false;
      form.name = "";
      await load();
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
</script>

<div class="stack">
  <div class="row spread">
    <div>
      <h1 style="margin:0">อินสแตนซ์</h1>
      <p class="muted" style="margin:0.2rem 0 0">อัปโหลดไฟล์ม็อดแพ็ก เผยแพร่เวอร์ชัน และย้อนกลับได้อย่างปลอดภัย</p>
    </div>
    <button class="primary" onclick={() => (creating = !creating)}>＋ สร้างอินสแตนซ์</button>
  </div>

  {#if creating}
    <div class="card stack">
      <h3 style="margin:0">อินสแตนซ์ใหม่</h3>
      <div class="grid">
        <label>ชื่อ<input bind:value={form.name} placeholder="เช่น SoraWorks SMP" /></label>
        <label>เวอร์ชันเกม<input bind:value={form.gameVersion} /></label>
        <label>Loader
          <select bind:value={form.loaderType}>
            <option value="vanilla">Vanilla</option>
            <option value="forge">Forge</option>
            <option value="neoforge">NeoForge</option>
            <option value="fabric">Fabric</option>
            <option value="quilt">Quilt</option>
          </select>
        </label>
        <label>เวอร์ชัน loader<input bind:value={form.loaderVersion} placeholder="เช่น 47.3.0" /></label>
        <label>Java<input type="number" bind:value={form.javaMajor} /></label>
        <label>RAM (MB, 0=auto)<input type="number" bind:value={form.memoryMb} /></label>
        <label>การเข้าถึง
          <select bind:value={form.access}>
            <option value="whitelist">Whitelist</option>
            <option value="public">สาธารณะ (ใครล็อกอินก็เล่นได้)</option>
          </select>
        </label>
      </div>
      <div class="row">
        <button class="primary" onclick={create} disabled={!form.name}>บันทึก</button>
        <button class="ghost" onclick={() => (creating = false)}>ยกเลิก</button>
      </div>
    </div>
  {/if}

  <div class="card">
    <table>
      <thead><tr><th>อินสแตนซ์</th><th>เป้าหมาย</th><th>เวอร์ชัน</th><th>การเข้าถึง</th><th>สถานะ</th></tr></thead>
      <tbody>
        {#each instances as i}
          <tr>
            <td>
              <a href={`#/instances/${i.id}`} class="iname">
                <span class="ic" style={`background:linear-gradient(135deg, ${i.accentColor ?? "var(--accent)"}, ${i.accentColor2 ?? "var(--accent2)"})`}>{i.name.charAt(0)}</span>
                <span>
                  <strong>{i.name}</strong>
                  <span class="muted mono">{i.id}</span>
                </span>
              </a>
            </td>
            <td><span class="pill">{i.gameVersion}</span> <span class="pill">{i.loader.type}</span></td>
            <td class="muted">{i.contentVersion}</td>
            <td><span class="pill" class:warn={i.access === "whitelist"}>{i.access}</span></td>
            <td><span class="pill" class:ok={i.status === "published"}>{i.status}</span></td>
          </tr>
        {/each}
        {#if instances.length === 0}
          <tr><td colspan="5" class="muted" style="text-align:center;padding:2rem">ยังไม่มีอินสแตนซ์ — กด "สร้างอินสแตนซ์"</td></tr>
        {/if}
      </tbody>
    </table>
  </div>
</div>

<style>
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: 0.8rem; }
  .grid label { display: flex; flex-direction: column; gap: 0.3rem; }
  .iname { display: flex; align-items: center; gap: 0.7rem; text-decoration: none; color: var(--text); }
  .ic { width: 34px; height: 34px; border-radius: 9px; display: grid; place-items: center; font-weight: 800; color: #fff; }
  .iname span span { display: block; }
  .mono { font-size: 0.72rem; }
</style>
