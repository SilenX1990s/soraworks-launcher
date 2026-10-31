<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "../i18n";
  import { mojang } from "../tauri";
  import type { VersionManifest, VersionEntry } from "../types";
  import { selection, selectVersion } from "../stores/selection";
  import { settings, patchSettings } from "../stores/settings";

  let manifest = $state<VersionManifest | null>(null);
  let loading = $state(true);
  let errorMsg = $state("");

  const visible = $derived<VersionEntry[]>(
    manifest
      ? manifest.versions.filter(
          (v) => $settings.showSnapshots || v.type === "release",
        )
      : [],
  );

  function typeKey(type: string): string {
    switch (type) {
      case "snapshot":
        return "snapshot";
      case "old_beta":
        return "oldBeta";
      case "old_alpha":
        return "oldAlpha";
      default:
        return "release";
    }
  }

  async function load(force = false) {
    loading = true;
    errorMsg = "";
    try {
      manifest = await mojang.versions(force);
    } catch (e) {
      errorMsg = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => load(false));
</script>

<div class="stack">
  <div class="row spread">
    <h2>{$t("version.title")}</h2>
    <div class="row">
      <label class="row">
        <input
          type="checkbox"
          checked={$settings.showSnapshots}
          onchange={(e) =>
            patchSettings({ showSnapshots: (e.target as HTMLInputElement).checked })}
        />
        {$t("version.showSnapshots")}
      </label>
      <button class="ghost" onclick={() => load(true)}>{$t("common.refresh")}</button>
    </div>
  </div>

  {#if loading}
    <div class="card muted">{$t("version.refreshing")}</div>
  {:else if errorMsg}
    <div class="card error">{errorMsg}</div>
  {:else if manifest}
    <div class="list">
      {#each visible as v (v.id)}
        <button
          class="ver ghost"
          class:active={$selection.versionId === v.id}
          onclick={() => selectVersion(v.id)}
        >
          <span class="id">{v.id}</span>
          <span class="type muted">{$t(`version.${typeKey(v.type)}`)}</span>
          {#if $selection.versionId === v.id}
            <span class="check">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .ver {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    justify-content: flex-start;
    border: 1px solid transparent;
  }

  .ver.active {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 14%, transparent);
  }

  .id {
    font-weight: 600;
    min-width: 8rem;
  }

  .check {
    margin-left: auto;
    color: var(--accent);
  }

  .error {
    color: #ff9b9b;
  }
</style>
