<script lang="ts">
  import { t } from "../i18n";
  import { instances, activeInstanceId, selectInstance, applyInstanceBranding } from "../stores/instances";
  import type { Instance } from "../instances";
  import type { Route } from "../routes";

  let { route = $bindable() }: { route: Route } = $props();

  function openInstance(inst: Instance) {
    selectInstance(inst.id);
    applyInstanceBranding(inst);
    route = "play";
  }
</script>

<nav class="rail">
  <button class="logo" onclick={() => (route = "library")} title={$t("shell.library")}>
    <span class="mark">S</span>
  </button>

  <div class="instances">
    {#each $instances as inst (inst.id)}
      <button
        class="inst"
        class:active={$activeInstanceId === inst.id && route === "play"}
        onclick={() => openInstance(inst)}
        title={inst.name}
        style:--a={inst.accentColor}
        style:--b={inst.accentColor2}
      >
        {#if inst.iconImage}
          <img src={inst.iconImage} alt={inst.name} />
        {:else}
          <span class="txt">{inst.iconText ?? inst.name.charAt(0)}</span>
        {/if}
      </button>
    {/each}

    <button class="rail-btn add" title={$t("shell.addInstance")} disabled>
      <span>＋</span>
    </button>
  </div>

  <div class="spacer"></div>

  <button class="rail-btn" title={$t("shell.explore")} disabled>
    <span>◎</span>
  </button>
  <button
    class="rail-btn"
    class:active={route === "library"}
    title={$t("shell.library")}
    onclick={() => (route = "library")}
  >
    <span>▦</span>
  </button>
</nav>

<style>
  .rail {
    width: 76px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.6rem;
    padding: 0.9rem 0;
    background: color-mix(in srgb, var(--surface-solid) 86%, transparent);
    border-right: 1px solid var(--border);
    -webkit-app-region: no-drag;
  }

  .logo {
    padding: 0;
    width: 46px;
    height: 46px;
    border-radius: 14px;
    margin-bottom: 0.3rem;
  }

  .mark {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    border-radius: 14px;
    font-weight: 800;
    color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent2));
  }

  .instances {
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
    align-items: center;
  }

  .inst {
    position: relative;
    width: 48px;
    height: 48px;
    padding: 0;
    border-radius: 15px;
    overflow: hidden;
    background: linear-gradient(135deg, var(--a, var(--accent)), var(--b, var(--accent2)));
    transition: border-radius 0.18s, transform 0.08s;
  }

  .inst:hover {
    border-radius: 12px;
  }

  .inst img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .inst .txt {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    color: #fff;
    font-weight: 800;
    font-size: 1.15rem;
  }

  .inst.active::before {
    content: "";
    position: absolute;
    left: -14px;
    top: 50%;
    transform: translateY(-50%);
    width: 5px;
    height: 60%;
    border-radius: 0 4px 4px 0;
    background: var(--accent);
  }

  .rail-btn {
    width: 44px;
    height: 44px;
    border-radius: 14px;
    padding: 0;
    font-size: 1.2rem;
    background: var(--surface-2);
    color: var(--text-muted);
  }

  .rail-btn.add {
    color: var(--accent);
    font-size: 1.4rem;
  }

  .rail-btn.active {
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .spacer {
    flex: 1;
  }
</style>
