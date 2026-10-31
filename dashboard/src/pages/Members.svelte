<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { toast } from "../lib/store";

  let members = $state<{ uuid: string; name: string; role: string }[]>([]);
  let name = $state("");
  let uuid = $state("");
  let role = $state("player");

  async function load() {
    try {
      members = await api.listMembers();
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }
  onMount(load);

  async function add() {
    if (!uuid || !name) return;
    try {
      await api.addMember(uuid.trim(), name.trim(), role);
      name = uuid = "";
      await load();
      toast("เพิ่มสมาชิกแล้ว");
    } catch (e) {
      toast(String(e instanceof Error ? e.message : e), "error");
    }
  }

  async function remove(u: string) {
    await api.removeMember(u);
    await load();
  }
</script>

<div class="stack">
  <h1>สมาชิก</h1>
  <div class="card">
    <div class="row" style="margin-bottom:0.9rem;flex-wrap:wrap">
      <input placeholder="ชื่อ" bind:value={name} style="max-width:200px" />
      <input placeholder="UUID" bind:value={uuid} style="max-width:320px" />
      <select bind:value={role} style="max-width:140px">
        <option value="player">player</option>
        <option value="mod">mod</option>
        <option value="admin">admin</option>
      </select>
      <button class="primary" onclick={add}>เพิ่ม</button>
    </div>
    <table>
      <thead><tr><th>ชื่อ</th><th>UUID</th><th>บทบาท</th><th></th></tr></thead>
      <tbody>
        {#each members as m}
          <tr>
            <td>{m.name}</td>
            <td class="mono muted">{m.uuid}</td>
            <td><span class="pill">{m.role}</span></td>
            <td><button class="ghost danger" onclick={() => remove(m.uuid)}>ลบ</button></td>
          </tr>
        {/each}
        {#if members.length === 0}
          <tr><td colspan="4" class="muted" style="text-align:center;padding:1.6rem">ยังไม่มีสมาชิก</td></tr>
        {/if}
      </tbody>
    </table>
  </div>
</div>

<style>
  .mono { font-size: 0.74rem; }
</style>
