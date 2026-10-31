<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import type { Route } from "./lib/routes";
  import Background from "./lib/components/Background.svelte";
  import IconRail from "./lib/components/IconRail.svelte";
  import TopBar from "./lib/components/TopBar.svelte";
  import SocialDock from "./lib/components/SocialDock.svelte";
  import HomeView from "./lib/views/HomeView.svelte";
  import PlayView from "./lib/views/PlayView.svelte";
  import AccountsView from "./lib/views/AccountsView.svelte";
  import SettingsView from "./lib/views/SettingsView.svelte";
  import AboutView from "./lib/views/AboutView.svelte";
  import { refreshAccounts } from "./lib/stores/accounts";
  import { activeInstance, applyInstanceBranding } from "./lib/stores/instances";

  let route = $state<Route>("library");

  onMount(() => {
    refreshAccounts().catch((e) => console.error("refreshAccounts failed", e));
    const inst = get(activeInstance);
    if (inst) applyInstanceBranding(inst);
  });
</script>

<Background instance={$activeInstance} />

<div class="app">
  <IconRail bind:route />

  <main>
    <TopBar bind:route />

    <section class="content">
      {#if route === "library"}
        <HomeView onNavigate={(r) => (route = r)} />
      {:else if route === "play"}
        <PlayView onNavigate={(r) => (route = r)} />
      {:else if route === "accounts"}
        <AccountsView />
      {:else if route === "settings"}
        <SettingsView />
      {:else if route === "about"}
        <AboutView />
      {/if}
    </section>

    <SocialDock instance={$activeInstance} />
  </main>
</div>

<style>
  .app {
    display: grid;
    grid-template-columns: auto 1fr;
    height: 100vh;
  }

  main {
    position: relative;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 0.4rem 1.6rem 1.6rem;
  }
</style>
