<script lang="ts">
  import { onMount } from "svelte";
  import { connected, adminToken } from "./lib/store";
  import { api } from "./lib/api";
  import Sidebar from "./components/Sidebar.svelte";
  import Toasts from "./components/Toasts.svelte";
  import Login from "./pages/Login.svelte";
  import Overview from "./pages/Overview.svelte";
  import Instances from "./pages/Instances.svelte";
  import InstanceDetail from "./pages/InstanceDetail.svelte";
  import Members from "./pages/Members.svelte";
  import Settings from "./pages/Settings.svelte";
  import Storage from "./pages/Storage.svelte";
  import Developers from "./pages/Developers.svelte";

  let hash = $state(location.hash || "#/overview");

  onMount(() => {
    const onHash = () => (hash = location.hash || "#/overview");
    window.addEventListener("hashchange", onHash);

    // Try to auto-connect using the persisted token so a reload stays signed in.
    api
      .overview()
      .then(() => connected.set(true))
      .catch(() => connected.set(false));

    return () => window.removeEventListener("hashchange", onHash);
  });

  // Current path + optional :id segment.
  const path = $derived(hash.replace(/^#/, "") || "/overview");
  const instanceId = $derived.by(() => {
    const m = path.match(/^\/instances\/(.+)$/);
    return m ? m[1] : null;
  });
</script>

{#if !$connected}
  <Login />
{:else}
  <div class="app">
    <Sidebar {path} />
    <main>
      {#if instanceId}
        {#key instanceId}
          <InstanceDetail id={instanceId} />
        {/key}
      {:else if path.startsWith("/instances")}
        <Instances />
      {:else if path.startsWith("/members")}
        <Members />
      {:else if path.startsWith("/settings")}
        <Settings />
      {:else if path.startsWith("/storage")}
        <Storage />
      {:else if path.startsWith("/developers")}
        <Developers />
      {:else}
        <Overview />
      {/if}
    </main>
  </div>
{/if}

<Toasts />

<style>
  .app { display: flex; min-height: 100vh; }
  main { flex: 1; min-width: 0; padding: 1.6rem 2rem; }
  @media (max-width: 760px) { main { padding: 1rem; } }
</style>
