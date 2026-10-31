<script lang="ts">
  import { t } from "../i18n";
  import { instances, activeInstanceId, selectInstance, applyInstanceBranding } from "../stores/instances";
  import { loaderLabel, type Instance } from "../instances";
  import type { Route } from "../routes";

  let { onNavigate }: { onNavigate?: (r: Route) => void } = $props();

  function pick(inst: Instance) {
    selectInstance(inst.id);
    applyInstanceBranding(inst);
    onNavigate?.("home");
  }
</script>

<div class="stack">
  <div class="head">
    <h2>{$t("instance.title")}</h2>
    <p class="muted">{$t("instance.lockedHint")}</p>
  </div>

  <div class="grid">
    {#each $instances as inst (inst.id)}
      <button
        class="card inst ghost"
        class:active={$activeInstanceId === inst.id}
        onclick={() => pick(inst)}
      >
        <div class="top">
          <div
            class="icon"
            style:--a={inst.accentColor}
            style:--b={inst.accentColor2}
          >
            {inst.iconText ?? inst.name.charAt(0)}
          </div>
          <div class="name">
            <strong>{inst.name}</strong>
            {#if inst.description}<span class="muted small">{inst.description}</span>{/if}
          </div>
          {#if $activeInstanceId === inst.id}<span class="dot">●</span>{/if}
        </div>
        <div class="tags">
          <span class="tag">{inst.gameVersion}</span>
          <span class="tag">{loaderLabel(inst.loader)}</span>
          {#if inst.serverAddress}<span class="tag srv">{inst.serverAddress}</span>{/if}
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .head h2 {
    margin-bottom: 0.2rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 0.8rem;
  }

  .inst {
    display: flex;
    flex-direction: column;
    gap: 0.9rem;
    align-items: stretch;
    text-align: left;
    border: 1px solid var(--border);
    padding: 1.1rem;
    transition: border-color 0.15s, transform 0.08s;
  }

  .inst:hover {
    transform: translateY(-2px);
  }

  .inst.active {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent) inset;
  }

  .top {
    display: flex;
    align-items: center;
    gap: 0.8rem;
  }

  .icon {
    width: 46px;
    height: 46px;
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    font-weight: 800;
    font-size: 1.3rem;
    color: #fff;
    background: linear-gradient(135deg, var(--a, var(--accent)), var(--b, var(--accent2)));
  }

  .name {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .dot {
    color: var(--accent);
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
  }

  .tag {
    font-size: 0.78rem;
    padding: 0.2em 0.6em;
    border-radius: 999px;
    background: var(--surface-2);
    color: var(--text-muted);
  }

  .tag.srv {
    color: var(--accent);
  }

  .small {
    font-size: 0.8rem;
  }
</style>
