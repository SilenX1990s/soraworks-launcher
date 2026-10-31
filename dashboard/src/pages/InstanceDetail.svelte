<script lang="ts">
  import { onMount } from "svelte";
  import { api, type ApiInstance } from "../lib/api";
  import { toast } from "../lib/store";

  let { id }: { id: string } = $props();

  let inst = $state<ApiInstance | null>(null);
  let tab = $state<"general" | "appearance" | "runtime" | "access" | "advanced">("general");
  let whitelist = $state<{ uuid: string; name: string; status: string }[]>([]);
  let wlName = $state("");
  let wlUuid = $state("");

  async function load() {
    try {
      inst = await api.getInstance(id);
      whitelist = await api.listWhitelist(id);
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
  onMount(load);

  async function save() {
    if (!inst) return;
    try {
      await api.updateInstance(id, inst);
      toast("บันทึกแล้ว");
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }

  async function togglePublish() {
    if (!inst) return;
    inst.status = inst.status === "published" ? "draft" : "published";
    await save();
  }

  async function addWl() {
    if (!wlUuid || !wlName) return;
    try {
      await api.addWhitelist(id, wlUuid.trim(), wlName.trim());
      wlName = wlUuid = "";
      whitelist = await api.listWhitelist(id);
      toast("เพิ่มใน whitelist แล้ว");
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }

  async function removeWl(uuid: string) {
    await api.removeWhitelist(id, uuid);
    whitelist = await api.listWhitelist(id);
  }

  async function del() {
    if (!confirm("ลบอินสแตนซ์นี้?")) return;
    await api.deleteInstance(id);
    toast("ลบแล้ว");
    location.hash = "#/instances";
  }

  const tabs: { id: typeof tab; label: string }[] = [
    { id: "general", label: "ทั่วไป" },
    { id: "appearance", label: "หน้าตาในลันเชอร์" },
    { id: "runtime", label: "รันไทม์" },
    { id: "access", label: "การเข้าถึง" },
    { id: "advanced", label: "ขั้นสูง" },
  ];
</script>

{#if inst}
  <div class="stack">
    <a href="#/instances" class="muted">← กลับไปที่อินสแตนซ์</a>

    <div class="banner card" style={`background:${inst.cover ?? `linear-gradient(135deg, ${inst.accentColor ?? "var(--accent)"}, ${inst.accentColor2 ?? "var(--accent2)"})`}`}>
      <div class="bshade"></div>
      <div class="brow">
        <div class="bic">{inst.name.charAt(0)}</div>
        <div>
          <h1>{inst.name}</h1>
          <div class="row" style="gap:0.4rem">
            <span class="pill" class:ok={inst.status === "published"}>{inst.status}</span>
            <span class="pill">{inst.gameVersion}</span>
            <span class="pill">{inst.loader.type}</span>
          </div>
        </div>
        <button class="primary" onclick={togglePublish}>
          {inst.status === "published" ? "ยกเลิกเผยแพร่" : "เผยแพร่"}
        </button>
      </div>
    </div>

    <div class="tabs">
      {#each tabs as t}
        <button class="tab ghost" class:active={tab === t.id} onclick={() => (tab = t.id)}>{t.label}</button>
      {/each}
    </div>

    <div class="card">
      {#if tab === "general"}
        <div class="grid">
          <label>ชื่อ<input bind:value={inst.name} /></label>
          <label>คำอธิบาย<input bind:value={inst.description} /></label>
          <label>เวอร์ชันเกม<input bind:value={inst.gameVersion} /></label>
          <label>เซิร์ฟเวอร์<input bind:value={inst.serverAddress} placeholder="play.example.net" /></label>
          <label>Content version<input bind:value={inst.contentVersion} /></label>
        </div>
        <button class="primary" style="margin-top:0.9rem" onclick={save}>บันทึก</button>
      {:else if tab === "appearance"}
        <div class="grid">
          <label>สีหลัก<input type="color" bind:value={inst.accentColor} /></label>
          <label>สีรอง<input type="color" bind:value={inst.accentColor2} /></label>
          <label class="wide">Cover (URL รูป หรือ CSS gradient)<input bind:value={inst.cover} /></label>
          <label class="wide">Icon image URL<input bind:value={inst.iconImage} /></label>
        </div>
        <p class="muted small">ภาพปกจะแสดงเป็นแบนเนอร์ในลันเชอร์ ใส่ลิงก์รูปจาก R2 หรือ URL ก็ได้</p>
        <button class="primary" onclick={save}>บันทึก</button>
      {:else if tab === "runtime"}
        <div class="grid">
          <label>Loader
            <select bind:value={inst.loader.type}>
              <option value="vanilla">Vanilla</option>
              <option value="forge">Forge</option>
              <option value="neoforge">NeoForge</option>
              <option value="fabric">Fabric</option>
              <option value="quilt">Quilt</option>
            </select>
          </label>
          <label>เวอร์ชัน loader<input bind:value={inst.loader.version} /></label>
          <label>Java (ค่าเริ่มต้น)<input type="number" bind:value={inst.javaMajor} /></label>
          <label>RAM (MB, 0=auto)<input type="number" bind:value={inst.memoryMb} /></label>
          <label class="check"><input type="checkbox" bind:checked={inst.allowJavaOverride} /> ให้ผู้เล่นเลือก Java เองได้</label>
        </div>
        <button class="primary" onclick={save}>บันทึก</button>
      {:else if tab === "access"}
        <div class="row spread" style="margin-bottom:0.8rem">
          <label style="max-width:260px">การเข้าถึง
            <select bind:value={inst.access} onchange={save}>
              <option value="whitelist">Whitelist เท่านั้น</option>
              <option value="public">สาธารณะ</option>
            </select>
          </label>
          <span class="pill">{whitelist.length} ใน whitelist</span>
        </div>
        <div class="row" style="margin-bottom:0.8rem">
          <input placeholder="ชื่อผู้เล่น" bind:value={wlName} />
          <input placeholder="UUID" bind:value={wlUuid} />
          <button class="primary" onclick={addWl}>เพิ่ม</button>
        </div>
        <table>
          <thead><tr><th>ชื่อ</th><th>UUID</th><th>สถานะ</th><th></th></tr></thead>
          <tbody>
            {#each whitelist as w}
              <tr>
                <td>{w.name}</td>
                <td class="mono muted">{w.uuid}</td>
                <td><span class="pill" class:ok={w.status === "approved"}>{w.status}</span></td>
                <td><button class="ghost danger" onclick={() => removeWl(w.uuid)}>ลบ</button></td>
              </tr>
            {/each}
            {#if whitelist.length === 0}
              <tr><td colspan="4" class="muted" style="text-align:center;padding:1.4rem">ยังไม่มีใครใน whitelist</td></tr>
            {/if}
          </tbody>
        </table>
      {:else if tab === "advanced"}
        <p class="muted">การลบจะลบอินสแตนซ์และ whitelist ของมันทั้งหมด ย้อนกลับไม่ได้</p>
        <button class="danger" onclick={del}>ลบอินสแตนซ์นี้</button>
      {/if}
    </div>
  </div>
{:else}
  <p class="muted">กำลังโหลด…</p>
{/if}

<style>
  .banner { position: relative; overflow: hidden; min-height: 150px; display: flex; align-items: flex-end; padding: 0; }
  .bshade { position: absolute; inset: 0; background: linear-gradient(to top, rgba(0,0,0,0.75), transparent 70%); }
  .brow { position: relative; display: flex; align-items: center; gap: 1rem; padding: 1.2rem; width: 100%; }
  .bic { width: 58px; height: 58px; border-radius: 13px; display: grid; place-items: center; font-weight: 800; font-size: 1.5rem; color: #fff; background: rgba(0,0,0,0.3); backdrop-filter: blur(6px); }
  .brow h1 { margin: 0 0 0.3rem; color: #fff; }
  .brow > button { margin-left: auto; }
  .tabs { display: flex; gap: 0.3rem; flex-wrap: wrap; }
  .tab { border: 1px solid var(--border); font-size: 0.88rem; }
  .tab.active { border-color: var(--accent); color: #fff; background: color-mix(in srgb, var(--accent) 16%, transparent); }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: 0.8rem; }
  .grid label { display: flex; flex-direction: column; gap: 0.3rem; }
  .grid label.wide { grid-column: 1 / -1; }
  .grid label.check { flex-direction: row; align-items: center; gap: 0.5rem; }
  .grid label.check input { width: auto; }
  .mono { font-size: 0.74rem; }
  .small { font-size: 0.82rem; }
</style>
