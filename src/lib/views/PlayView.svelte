<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { t } from "../i18n";
  import { activeAccount } from "../stores/accounts";
  import {
    activeInstance,
    effectiveJavaMajor,
    javaOverrides,
    setJavaOverride,
  } from "../stores/instances";
  import { settings } from "../stores/settings";
  import { launch } from "../tauri";
  import { loaderLabel, SELECTABLE_JAVA } from "../instances";
  import type { LaunchProgress } from "../types";
  import type { Route } from "../routes";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let { onNavigate }: { onNavigate: (r: Route) => void } = $props();

  let running = $state(false);
  let progress = $state<LaunchProgress | null>(null);
  let errorMsg = $state("");
  let unlisteners: UnlistenFn[] = [];

  onMount(async () => {
    unlisteners.push(await launch.onProgress((p) => { progress = p; running = true; }));
    unlisteners.push(
      await launch.onExit((e) => {
        running = false;
        progress = null;
        if (e.code && e.code !== 0) errorMsg = $t("launch.crashed", { code: e.code });
      }),
    );
    running = await launch.isRunning().catch(() => false);
  });
  onDestroy(() => unlisteners.forEach((u) => u()));

  const inst = $derived($activeInstance);
  const javaMajor = $derived(inst ? effectiveJavaMajor(inst) : 17);
  $effect(() => void $javaOverrides);

  async function play() {
    errorMsg = "";
    if (!inst) return;
    if (!$activeAccount) {
      errorMsg = $t("launch.needAccount");
      onNavigate("accounts");
      return;
    }
    if (inst.loader.type !== "vanilla") {
      errorMsg = $t("instance.loaderNotReady", { loader: loaderLabel(inst.loader) });
      return;
    }
    try {
      running = true;
      progress = { stage: "preparing", done: 0, total: 0 };
      await launch.vanilla(
        {
          versionId: inst.gameVersion,
          accountId: $activeAccount.id,
          maxMemoryMb: inst.memoryMb,
          extraJvmArgs: inst.jvmArgs,
          javaMajor: effectiveJavaMajor(inst),
        },
        $settings.gameDir,
      );
    } catch (e) {
      running = false;
      progress = null;
      errorMsg = String(e);
    }
  }

  function progressLabel(p: LaunchProgress): string {
    switch (p.stage) {
      case "assets": return $t("launch.downloadingAssets", { done: p.done, total: p.total });
      case "libraries": return $t("launch.downloadingLibraries", { done: p.done, total: p.total });
      case "java": return $t("java.downloading", { version: javaMajor });
      case "running": return $t("launch.playing");
      case "launching": return $t("launch.launching");
      default: return $t("launch.preparing");
    }
  }

  const pct = $derived(
    progress && progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : null,
  );
</script>

{#if inst}
  <div class="play">
    <div class="banner" style:background={inst.cover ?? "linear-gradient(135deg, var(--accent), var(--accent2))"}>
      <div class="banner-shade"></div>
      <div class="banner-content">
        <div class="title-row">
          <div class="bicon" style:--a={inst.accentColor} style:--b={inst.accentColor2}>
            {inst.iconText ?? inst.name.charAt(0)}
          </div>
          <div>
            <h1>{inst.name}</h1>
            {#if inst.description}<p class="desc">{inst.description}</p>{/if}
          </div>
        </div>
      </div>
    </div>

    <div class="specs">
      <div class="spec"><span class="k">{$t("instance.gameVersion")}</span><span class="v">{inst.gameVersion}</span></div>
      <div class="spec"><span class="k">{$t("instance.loader")}</span><span class="v">{loaderLabel(inst.loader)}</span></div>
      <div class="spec"><span class="k">{$t("instance.memory")}</span><span class="v">{inst.memoryMb === 0 ? $t("instance.auto") : `${inst.memoryMb} MB`}</span></div>
      <div class="spec"><span class="k">{$t("instance.javaRuntime")}</span>
        {#if inst.allowJavaOverride}
          <select value={javaMajor} onchange={(e) => setJavaOverride(inst.id, +(e.target as HTMLSelectElement).value)}>
            {#each SELECTABLE_JAVA as j}
              <option value={j.major}>{j.label}{j.major === inst.javaMajor ? ` · ${$t("instance.auto")}` : ""}</option>
            {/each}
          </select>
        {:else}<span class="v">Java {inst.javaMajor}</span>{/if}
      </div>
    </div>
    <p class="locked muted">🔒 {$t("instance.lockedHint")}</p>

    <div class="playbar">
      <div class="status">
        {#if progress}
          <div class="pstage">{progressLabel(progress)}</div>
          <div class="bar"><div class="fill" class:indeterminate={pct === null} style:width={pct !== null ? `${pct}%` : "40%"}></div></div>
        {:else if errorMsg}
          <div class="err">{errorMsg}</div>
        {:else if inst.serverAddress}
          <div class="muted">{$t("instance.server")}: {inst.serverAddress}</div>
        {/if}
      </div>
      <div class="actions">
        {#if running}<button class="ghost" onclick={() => launch.stop()}>{$t("launch.stop")}</button>{/if}
        <button class="primary play-btn" onclick={play} disabled={running}>
          {running ? $t("launch.playing") : $t("instance.play")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .play {
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
  }

  .banner {
    position: relative;
    height: 220px;
    border-radius: var(--radius-lg);
    overflow: hidden;
    background-size: cover;
    background-position: center;
    display: flex;
    align-items: flex-end;
  }

  .banner-shade {
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, rgba(0,0,0,0.8), transparent 70%);
  }

  .banner-content {
    position: relative;
    padding: 1.4rem;
    width: 100%;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .bicon {
    width: 64px;
    height: 64px;
    border-radius: var(--radius);
    display: grid;
    place-items: center;
    font-weight: 800;
    font-size: 1.8rem;
    color: #fff;
    background: linear-gradient(135deg, var(--a, var(--accent)), var(--b, var(--accent2)));
    box-shadow: 0 8px 24px -8px rgba(0,0,0,0.6);
  }

  .banner h1 {
    font-size: 1.9rem;
    margin: 0;
    color: #fff;
    text-shadow: 0 2px 10px rgba(0,0,0,0.6);
  }

  .desc {
    margin: 0.2rem 0 0;
    color: rgba(255,255,255,0.8);
  }

  .specs {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 0.8rem;
  }

  .spec {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.8rem 1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }

  .spec .k {
    font-size: 0.72rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .spec .v {
    font-weight: 650;
    font-size: 1.05rem;
  }

  .spec select {
    width: 100%;
  }

  .locked {
    font-size: 0.82rem;
    margin: 0;
  }

  .playbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 1rem 1.4rem;
  }

  .status { flex: 1; min-width: 0; }
  .pstage { font-size: 0.9rem; margin-bottom: 0.4rem; }

  .bar {
    height: 8px;
    background: var(--surface-2);
    border-radius: 999px;
    overflow: hidden;
  }

  .fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent), var(--accent2));
    transition: width 0.3s ease;
  }

  .fill.indeterminate {
    width: 40% !important;
    animation: slide 1.1s infinite ease-in-out;
  }

  @keyframes slide {
    0% { margin-left: -40%; }
    100% { margin-left: 100%; }
  }

  .actions { display: flex; gap: 0.6rem; }
  .play-btn { font-size: 1.25rem; padding: 0.7em 2.4em; }
  .err { color: #ff9b9b; font-size: 0.9rem; }
</style>
