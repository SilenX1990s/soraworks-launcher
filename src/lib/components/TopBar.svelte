<script lang="ts">
  import { t } from "../i18n";
  import LanguagePicker from "./LanguagePicker.svelte";
  import { activeAccount } from "../stores/accounts";
  import { instances } from "../stores/instances";
  import type { Route } from "../routes";

  let { route = $bindable() }: { route: Route } = $props();

  const totalOnline = $derived(
    $instances.reduce((sum, i) => sum + (i.playersOnline ?? 0), 0),
  );
</script>

<header class="topbar">
  <div class="search">
    <span class="ic">⌕</span>
    <input type="text" placeholder={$t("shell.search")} disabled />
  </div>

  <div class="right">
    <div class="online">
      <span class="dot"></span>
      {$t("shell.online", { n: totalOnline })}
    </div>

    <LanguagePicker />

    <button class="chip" onclick={() => (route = "accounts")}>
      <span class="avatar">
        {$activeAccount ? $activeAccount.name.charAt(0).toUpperCase() : "?"}
      </span>
      <span class="who">{$activeAccount?.name ?? $t("account.noAccount")}</span>
      <span class="caret">▾</span>
    </button>

    <button
      class="gear"
      class:active={route === "settings"}
      onclick={() => (route = "settings")}
      title={$t("nav.settings")}
    >
      ⚙
    </button>
  </div>
</header>

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.7rem 1.2rem;
    -webkit-app-region: drag;
  }

  .topbar :global(button),
  .topbar :global(select),
  .topbar :global(input) {
    -webkit-app-region: no-drag;
  }

  .search {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: color-mix(in srgb, var(--surface-solid) 70%, transparent);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 0.4rem 0.9rem;
    min-width: 220px;
  }

  .search .ic {
    color: var(--text-muted);
  }

  .search input {
    border: none;
    background: transparent;
    padding: 0;
    color: var(--text);
    width: 100%;
    outline: none;
  }

  .right {
    display: flex;
    align-items: center;
    gap: 0.7rem;
  }

  .online {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.85rem;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--surface-solid) 70%, transparent);
    border: 1px solid var(--border);
    padding: 0.35rem 0.7rem;
    border-radius: 999px;
  }

  .online .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #35d07f;
    box-shadow: 0 0 0 3px rgba(53, 208, 127, 0.2);
  }

  .chip {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: color-mix(in srgb, var(--surface-solid) 70%, transparent);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 0.3rem 0.7rem 0.3rem 0.35rem;
  }

  .avatar {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    font-weight: 700;
    font-size: 0.8rem;
    color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent2));
  }

  .who {
    font-size: 0.88rem;
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .caret {
    color: var(--text-muted);
    font-size: 0.7rem;
  }

  .gear {
    width: 38px;
    height: 38px;
    border-radius: 50%;
    padding: 0;
    font-size: 1.1rem;
    background: color-mix(in srgb, var(--surface-solid) 70%, transparent);
    border: 1px solid var(--border);
  }

  .gear.active {
    color: var(--accent);
    border-color: var(--accent);
  }
</style>
