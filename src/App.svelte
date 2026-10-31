<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "./lib/i18n";
  import type { Route } from "./lib/routes";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import LanguagePicker from "./lib/components/LanguagePicker.svelte";
  import Background from "./lib/components/Background.svelte";
  import HomeView from "./lib/views/HomeView.svelte";
  import InstancesView from "./lib/views/InstancesView.svelte";
  import AccountsView from "./lib/views/AccountsView.svelte";
  import SettingsView from "./lib/views/SettingsView.svelte";
  import AboutView from "./lib/views/AboutView.svelte";
  import { refreshAccounts } from "./lib/stores/accounts";
  import { activeInstance, applyInstanceBranding } from "./lib/stores/instances";
  import { get } from "svelte/store";

  let route = $state<Route>("home");

  onMount(() => {
    // Load existing accounts from the OS keychain on startup.
    refreshAccounts().catch((e) => console.error("refreshAccounts failed", e));
    // Apply the active instance's branding on launch.
    const inst = get(activeInstance);
    if (inst) applyInstanceBranding(inst);
  });
</script>

<Background instance={$activeInstance} />

<div class="shell">
  <Sidebar bind:route />

  <main>
    <header class="topbar">
      <div class="brand">
        <span class="title">{$t(`nav.${route}`)}</span>
      </div>
      <LanguagePicker />
    </header>

    <section class="content">
      {#if route === "home"}
        <HomeView onNavigate={(r) => (route = r)} />
      {:else if route === "instances"}
        <InstancesView onNavigate={(r) => (route = r)} />
      {:else if route === "accounts"}
        <AccountsView />
      {:else if route === "settings"}
        <SettingsView />
      {:else if route === "about"}
        <AboutView />
      {/if}
    </section>
  </main>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 220px 1fr;
    height: 100vh;
  }

  main {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.9rem 1.4rem;
    border-bottom: 1px solid var(--border);
    -webkit-app-region: drag;
  }

  .topbar :global(button),
  .topbar :global(select) {
    -webkit-app-region: no-drag;
  }

  .title {
    font-size: 1.15rem;
    font-weight: 650;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.4rem;
  }
</style>
