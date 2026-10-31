<script lang="ts">
  import { t } from "../i18n";
  import { activeAccount } from "../stores/accounts";
  import { instances, selectInstance, applyInstanceBranding } from "../stores/instances";
  import { loaderLabel, type Instance } from "../instances";
  import type { Route } from "../routes";

  let { onNavigate }: { onNavigate: (r: Route) => void } = $props();

  function greetingKey(): string {
    const h = new Date().getHours();
    if (h < 12) return "home.greetMorning";
    if (h < 18) return "home.greetAfternoon";
    return "home.greetEvening";
  }

  const totalOnline = $derived(
    $instances.reduce((s, i) => s + (i.playersOnline ?? 0), 0),
  );

  function open(inst: Instance) {
    selectInstance(inst.id);
    applyInstanceBranding(inst);
    onNavigate("play");
  }
</script>

<div class="library">
  <header class="greet">
    <div class="eyebrow">{$t(greetingKey())}</div>
    <h1>{$activeAccount?.name ?? "SoraWorks"}</h1>
    <p class="muted summary">
      {$t("home.summary", { instances: $instances.length, players: totalOnline })}
    </p>
  </header>

  <div class="grid">
    {#each $instances as inst (inst.id)}
      <button class="card" onclick={() => open(inst)} style:--a={inst.accentColor} style:--b={inst.accentColor2}>
        <div class="cover" style:background={inst.cover ?? "linear-gradient(135deg, var(--a), var(--b))"}>
          {#if inst.iconImage}
            <img class="badge" src={inst.iconImage} alt="" />
          {:else}
            <div class="badge txt">{inst.iconText ?? inst.name.charAt(0)}</div>
          {/if}
          <div class="shade"></div>
          <div class="info">
            <strong class="name">{inst.name}</strong>
            <div class="tags">
              <span class="tag">{inst.gameVersion}</span>
              <span class="tag">{loaderLabel(inst.loader)}</span>
            </div>
          </div>
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .library {
    display: flex;
    flex-direction: column;
    gap: 1.6rem;
  }

  .greet .eyebrow {
    color: var(--accent);
    font-weight: 700;
    letter-spacing: 0.12em;
    font-size: 0.78rem;
  }

  .greet h1 {
    font-size: 2.3rem;
    margin: 0.1rem 0 0.2rem;
  }

  .summary {
    font-size: 0.95rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    gap: 1.1rem;
  }

  .card {
    padding: 0;
    border: none;
    background: none;
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: transform 0.12s ease;
  }

  .card:hover {
    transform: translateY(-4px);
  }

  .cover {
    position: relative;
    aspect-ratio: 3 / 4;
    background-size: cover;
    background-position: center;
    display: flex;
    align-items: flex-end;
  }

  .badge {
    position: absolute;
    top: 0.8rem;
    left: 0.8rem;
    width: 44px;
    height: 44px;
    border-radius: 12px;
    object-fit: cover;
    box-shadow: 0 4px 14px -4px rgba(0, 0, 0, 0.6);
  }

  .badge.txt {
    display: grid;
    place-items: center;
    font-weight: 800;
    font-size: 1.2rem;
    color: #fff;
    background: rgba(0, 0, 0, 0.35);
    backdrop-filter: blur(6px);
  }

  .shade {
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.82) 0%, rgba(0, 0, 0, 0.1) 55%, transparent 100%);
  }

  .info {
    position: relative;
    padding: 1rem;
    width: 100%;
    text-align: left;
  }

  .name {
    display: block;
    font-size: 1.15rem;
    color: #fff;
    margin-bottom: 0.4rem;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.6);
  }

  .tags {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
  }

  .tag {
    font-size: 0.72rem;
    padding: 0.18em 0.6em;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.16);
    color: #fff;
    backdrop-filter: blur(4px);
  }
</style>
