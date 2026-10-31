<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import type { Instance } from "../instances";

  let { instance }: { instance: Instance | null } = $props();

  const links = $derived(instance?.links ?? {});

  function open(url?: string) {
    if (url) openUrl(url);
  }
</script>

<div class="dock">
  {#if links.discord}
    <button class="s" title="Discord" onclick={() => open(links.discord)}>✦</button>
  {/if}
  {#if links.website}
    <button class="s" title="Website" onclick={() => open(links.website)}>◍</button>
  {/if}
  {#if links.email}
    <button class="s" title="Email" onclick={() => open(`mailto:${links.email}`)}>✉</button>
  {/if}
  {#if links.instagram}
    <button class="s" title="Instagram" onclick={() => open(links.instagram)}>◉</button>
  {/if}
</div>

<style>
  .dock {
    position: absolute;
    right: 1.4rem;
    bottom: 1.2rem;
    display: flex;
    gap: 0.5rem;
  }

  .s {
    width: 38px;
    height: 38px;
    border-radius: 50%;
    padding: 0;
    font-size: 1.05rem;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--surface-solid) 60%, transparent);
    border: 1px solid var(--border);
    backdrop-filter: blur(6px);
  }

  .s:hover {
    color: var(--accent);
    border-color: var(--accent);
  }
</style>
