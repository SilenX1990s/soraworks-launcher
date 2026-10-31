<script lang="ts">
  import { onMount } from "svelte";
  import { api, type ApiInstance } from "../lib/api";
  import { toast } from "../lib/store";

  let stats = $state<{ instances: number; members: number; whitelisted: number; pendingCrashes: number } | null>(null);
  let instances = $state<ApiInstance[]>([]);

  onMount(async () => {
    try {
      [stats, instances] = await Promise.all([api.overview(), api.listInstances()]);
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  });

  const cards = $derived([
    { label: "อินสแตนซ์", value: stats?.instances ?? 0, icon: "◼" },
    { label: "สมาชิก", value: stats?.members ?? 0, icon: "◉" },
    { label: "อยู่ใน whitelist", value: stats?.whitelisted ?? 0, icon: "✓" },
    { label: "รอตรวจ (crash)", value: stats?.pendingCrashes ?? 0, icon: "🧩" },
  ]);
</script>

<div class="stack">
  <h1>ภาพรวม</h1>

  <div class="cards">
    {#each cards as c}
      <div class="card stat">
        <span class="muted label">{c.icon} {c.label}</span>
        <span class="value">{c.value}</span>
      </div>
    {/each}
  </div>

  <div class="card">
    <div class="row spread" style="margin-bottom:0.6rem">
      <h3 style="margin:0">อินสแตนซ์ล่าสุด</h3>
      <a href="#/instances">ดูทั้งหมด →</a>
    </div>
    <table>
      <thead><tr><th>ชื่อ</th><th>เวอร์ชัน</th><th>loader</th><th>สถานะ</th></tr></thead>
      <tbody>
        {#each instances as i}
          <tr>
            <td><a href={`#/instances/${i.id}`}>{i.name}</a></td>
            <td>{i.gameVersion}</td>
            <td>{i.loader.type}{i.loader.version ? ` ${i.loader.version}` : ""}</td>
            <td><span class="pill" class:ok={i.status === "published"}>{i.status}</span></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(170px, 1fr)); gap: 0.9rem; }
  .stat { display: flex; flex-direction: column; gap: 0.4rem; }
  .stat .label { font-size: 0.82rem; }
  .stat .value { font-size: 1.9rem; font-weight: 700; }
</style>
