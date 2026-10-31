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
    unlisteners.push(
      await launch.onProgress((p) => {
        progress = p;
        running = true;
      }),
    );
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
  // Reference javaOverrides so the shown value reacts to changes.
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
      case "assets":
        return $t("launch.downloadingAssets", { done: p.done, total: p.total });
      case "libraries":
        return $t("launch.downloadingLibraries", { done: p.done, total: p.total });
      case "java":
        return $t("java.downloading", { version: javaMajor });
      case "running":
        return $t("launch.playing");
      case "launching":
        return $t("launch.launching");
      default:
        return $t("launch.preparing");
    }
  }

  const pct = $derived(
    progress && progress.total > 0
      ? Math.round((progress.done / progress.total) * 100)
      : null,
  );
</script>

{#if inst}
  <div class="home">
    <div class="hero">
      <div class="hero-top">
        <div class="inst-icon" style:--a={inst.accentColor} style:--b={inst.accentColor2}>
          {inst.iconText ?? inst.name.charAt(0)}
        </div>
        <div class="hero-text">
          <h1>{inst.name}</h1>
          {#if inst.description}<p class="muted">{inst.description}</p>{/if}
        </div>
        <button class="switch ghost" onclick={() => onNavigate("instances")}>
          {$t("instance.select")}
        </button>
      </div>

      <div class="specs">
        <div class="spec">
          <span class="k">{$t("instance.gameVersion")}</span>
          <span class="v">{inst.gameVersion}</span>
        </div>
        <div class="spec">
          <span class="k">{$t("instance.loader")}</span>
          <span class="v">{loaderLabel(inst.loader)}</span>
        </div>
        <div class="spec">
          <span class="k">{$t("instance.memory")}</span>
          <span class="v">{inst.memoryMb === 0 ? $t("instance.auto") : `${inst.memoryMb} MB`}</span>
        </div>
        <div class="spec java">
          <span class="k">{$t("instance.javaRuntime")}</span>
          {#if inst.allowJavaOverride}
            <select
              value={javaMajor}
              onchange={(e) => setJavaOverride(inst.id, +(e.target as HTMLSelectElement).value)}
            >
              {#each SELECTABLE_JAVA as j}
                <option value={j.major}>
                  {j.label}{j.major === inst.javaMajor ? ` · ${$t("instance.auto")}` : ""}
                </option>
              {/each}
            </select>
          {:else}
            <span class="v">Java {inst.javaMajor}</span>
          {/if}
        </div>
      </div>
      <p class="locked-note muted">🔒 {$t("instance.lockedHint")}</p>
    </div>

    <div class="playbar">
      <div class="status">
        {#if progress}
          <div class="pstage">{progressLabel(progress)}</div>
          <div class="bar">
            <div
              class="fill"
              class:indeterminate={pct === null}
              style:width={pct !== null ? `${pct}%` : "40%"}
            ></div>
          </div>
        {:else if errorMsg}
          <div class="err">{errorMsg}</div>
        {:else if inst.serverAddress}
          <div class="muted">{$t("instance.server")}: {inst.serverAddress}</div>
        {/if}
      </div>
      <div class="actions">
        {#if running}
          <button class="stop ghost" onclick={() => launch.stop()}>{$t("launch.stop")}</button>
        {/if}
        <button class="primary play" onclick={play} disabled={running}>
          {running ? $t("launch.playing") : $t("instance.play")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .home {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 1rem;
  }

  .hero {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1.4rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 1.6rem;
    position: relative;
    overflow: hidden;
  }

  .hero-top {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .inst-icon {
    width: 64px;
    height: 64px;
    border-radius: var(--radius);
    display: grid;
    place-items: center;
    font-size: 1.8rem;
    font-weight: 800;
    color: #fff;
    background: linear-gradient(135deg, var(--a, var(--accent)), var(--b, var(--accent2)));
    box-shadow: 0 8px 24px -8px color-mix(in srgb, var(--a, var(--accent)) 60%, transparent);
  }

  .hero-text {
    flex: 1;
    min-width: 0;
  }

  .hero-text h1 {
    font-size: 1.8rem;
    margin: 0;
  }

  .switch {
    border: 1px solid var(--border);
    white-space: nowrap;
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
    background: var(--surface-2);
    border-radius: var(--radius-sm);
  }

  .spec .k {
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .spec .v {
    font-weight: 650;
    font-size: 1.05rem;
  }

  .spec.java select {
    width: 100%;
  }

  .locked-note {
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

  .status {
    flex: 1;
    min-width: 0;
  }

  .pstage {
    font-size: 0.9rem;
    margin-bottom: 0.4rem;
  }

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

  .actions {
    display: flex;
    gap: 0.6rem;
  }

  .play {
    font-size: 1.25rem;
    padding: 0.7em 2.4em;
  }

  .err {
    color: #ff9b9b;
    font-size: 0.9rem;
  }
</style>
