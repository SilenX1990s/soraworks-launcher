<script lang="ts">
  import { t } from "../i18n";
  import type { Route } from "../routes";
  import { activeAccount } from "../stores/accounts";

  let { route = $bindable() }: { route: Route } = $props();

  const items: { id: Route; icon: string }[] = [
    { id: "home", icon: "▶" },
    { id: "instances", icon: "▣" },
    { id: "accounts", icon: "◉" },
    { id: "settings", icon: "⚙" },
    { id: "about", icon: "ⓘ" },
  ];
</script>

<aside>
  <div class="logo">
    <div class="mark">S</div>
    <div class="name">
      <strong>SoraWorks</strong>
      <span class="muted">{$t("app.tagline")}</span>
    </div>
  </div>

  <nav>
    {#each items as item}
      <button
        class="nav-item ghost"
        class:active={route === item.id}
        onclick={() => (route = item.id)}
      >
        <span class="icon">{item.icon}</span>
        <span>{$t(`nav.${item.id}`)}</span>
      </button>
    {/each}
  </nav>

  <div class="account">
    {#if $activeAccount}
      <div class="avatar">{$activeAccount.name.charAt(0).toUpperCase()}</div>
      <span class="who">{$activeAccount.name}</span>
    {:else}
      <span class="muted small">{$t("account.noAccount")}</span>
    {/if}
  </div>
</aside>

<style>
  aside {
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-right: 1px solid var(--border);
    padding: 1rem 0.8rem;
    gap: 1rem;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.3rem;
  }

  .mark {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    font-weight: 800;
    color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent2));
  }

  .name {
    display: flex;
    flex-direction: column;
    line-height: 1.15;
  }

  .name .muted {
    font-size: 0.72rem;
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.7rem;
    justify-content: flex-start;
    text-align: left;
    padding: 0.6em 0.8em;
  }

  .nav-item.active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
    color: var(--text);
  }

  .nav-item .icon {
    width: 1.2em;
    text-align: center;
    opacity: 0.85;
  }

  .account {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.6rem;
    border-top: 1px solid var(--border);
  }

  .avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    font-weight: 700;
    background: var(--surface-2);
  }

  .who {
    font-size: 0.9rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .small {
    font-size: 0.8rem;
  }
</style>
