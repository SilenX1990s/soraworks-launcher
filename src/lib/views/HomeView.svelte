<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { t } from "../i18n";
  import { activeAccount } from "../stores/accounts";
  import { selection } from "../stores/selection";
  import { settings } from "../stores/settings";
  import { launch } from "../tauri";
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
        if (e.code && e.code !== 0) {
          errorMsg = $t("launch.crashed", { code: e.code });
        }
      }),
    );
    running = await launch.isRunning().catch(() => false);
  });

  onDestroy(() => {
    unlisteners.forEach((u) => u());
  });

  async function play() {
    errorMsg = "";
    if (!$activeAccount) {
      errorMsg = $t("launch.needAccount");
      return;
    }
    if (!$selection.versionId) {
      onNavigate("versions");
      return;
    }
    try {
      running = true;
      progress = { stage: "preparing", done: 0, total: 0 };
      await launch.vanilla(
        {
          versionId: $selection.versionId,
          accountId: $activeAccount.id,
          maxMemoryMb: $settings.maxMemoryMb,
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
      case "running":
        return $t("launch.playing");
      case "launching":
        return $t("launch.launching");
      default:
        return $t("launch.preparing");
    }
  }
</script>

<div class="home stack">
  <div class="hero card">
    <div class="hero-info">
      <h1>{$t("app.name")}</h1>
      <p class="muted">{$t("app.tagline")}</p>

      <div class="selects">
        <button class="chip ghost" onclick={() => onNavigate("versions")}>
          <span class="muted">{$t("version.title")}:</span>
          <strong>{$selection.versionId || $t("version.select")}</strong>
        </button>
        <button class="chip ghost" onclick={() => onNavigate("accounts")}>
          <span class="muted">{$t("account.title")}:</span>
          <strong>{$activeAccount?.name ?? $t("account.noAccount")}</strong>
        </button>
      </div>
    </div>

    <div class="play-area">
      <button class="primary play" onclick={play} disabled={running}>
        {running ? $t("launch.playing") : $t("launch.play")}
      </button>
      {#if running}
        <button class="ghost" onclick={() => launch.stop()}>{$t("launch.stop")}</button>
      {/if}
    </div>
  </div>

  {#if progress}
    <div class="card">
      <div class="row spread">
        <span>{progressLabel(progress)}</span>
        {#if progress.total > 0}
          <span class="muted">{progress.done}/{progress.total}</span>
        {/if}
      </div>
      <div class="bar">
        <div
          class="fill"
          style:width={progress.total > 0
            ? `${Math.round((progress.done / progress.total) * 100)}%`
            : "35%"}
          class:indeterminate={progress.total === 0}
        ></div>
      </div>
    </div>
  {/if}

  {#if errorMsg}
    <div class="card error">{errorMsg}</div>
  {/if}
</div>

<style>
  .hero {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.5rem;
    background: linear-gradient(
      135deg,
      color-mix(in srgb, var(--accent) 16%, var(--surface)),
      var(--surface)
    );
  }

  .selects {
    display: flex;
    gap: 0.6rem;
    margin-top: 1rem;
    flex-wrap: wrap;
  }

  .chip {
    display: flex;
    gap: 0.4rem;
    align-items: center;
    border: 1px solid var(--border);
  }

  .play {
    font-size: 1.3rem;
    padding: 0.7em 2.2em;
  }

  .play-area {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: stretch;
  }

  .bar {
    margin-top: 0.6rem;
    height: 8px;
    background: var(--surface-2);
    border-radius: 999px;
    overflow: hidden;
  }

  .fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent), var(--accent2));
    transition: width 0.3s;
  }

  .fill.indeterminate {
    animation: slide 1.2s infinite ease-in-out;
  }

  @keyframes slide {
    0% {
      margin-left: -35%;
    }
    100% {
      margin-left: 100%;
    }
  }

  .error {
    border-color: color-mix(in srgb, #ff5c5c 50%, var(--border));
    color: #ff9b9b;
  }
</style>
