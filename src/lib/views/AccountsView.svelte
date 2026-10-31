<script lang="ts">
  import { t } from "../i18n";
  import { auth } from "../tauri";
  import {
    accounts,
    refreshAccounts,
    removeAccount,
    setActiveAccount,
  } from "../stores/accounts";
  import type { DeviceCodeInfo } from "../types";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let busy = $state(false);
  let errorMsg = $state("");
  let deviceInfo = $state<DeviceCodeInfo | null>(null);

  function mapError(e: unknown): string {
    const s = String(e);
    // The backend returns stable error codes (see src-tauri/src/error.rs).
    for (const code of [
      "auth_cancelled",
      "auth_no_xbox",
      "auth_child",
      "auth_no_game",
      "auth_region",
      "network",
    ]) {
      if (s.includes(code)) return $t(`error.${code}`);
    }
    return $t("error.unknown", { detail: s });
  }

  async function loginBrowser() {
    busy = true;
    errorMsg = "";
    try {
      await auth.loginBrowser();
      await refreshAccounts();
    } catch (e) {
      errorMsg = mapError(e);
    } finally {
      busy = false;
    }
  }

  async function loginDeviceCode() {
    busy = true;
    errorMsg = "";
    try {
      deviceInfo = await auth.startDeviceCode();
      // Poll in the background until the user finishes.
      await auth.completeDeviceCode(deviceInfo.deviceCode);
      deviceInfo = null;
      await refreshAccounts();
    } catch (e) {
      errorMsg = mapError(e);
      deviceInfo = null;
    } finally {
      busy = false;
    }
  }

  async function copyCode() {
    if (deviceInfo) await navigator.clipboard.writeText(deviceInfo.userCode);
  }
</script>

<div class="stack">
  <h2>{$t("account.title")}</h2>
  <p class="muted">{$t("account.noOfflineMode")}</p>

  {#if deviceInfo}
    <div class="card device">
      <p>{$t("account.deviceCodeInstruction", { url: deviceInfo.verificationUri })}</p>
      <div class="code-row">
        <code class="code">{deviceInfo.userCode}</code>
        <button class="ghost" onclick={copyCode}>{$t("common.copy")}</button>
        <button class="ghost" onclick={() => openUrl(deviceInfo!.verificationUri)}>
          {$t("common.ok")}
        </button>
      </div>
      <p class="muted small">{$t("account.signingIn")}</p>
    </div>
  {/if}

  <div class="row actions">
    <button class="primary" onclick={loginBrowser} disabled={busy}>
      {$t("account.signInBrowser")}
    </button>
    <button onclick={loginDeviceCode} disabled={busy}>
      {$t("account.signInDeviceCode")}
    </button>
  </div>

  {#if errorMsg}
    <div class="card error">{errorMsg}</div>
  {/if}

  <div class="list">
    {#each $accounts as acc (acc.id)}
      <div class="card acc">
        <div class="avatar">{acc.name.charAt(0).toUpperCase()}</div>
        <div class="info">
          <strong>{acc.name}</strong>
          {#if acc.active}
            <span class="badge">{$t("account.active")}</span>
          {/if}
        </div>
        <div class="row">
          {#if !acc.active}
            <button class="ghost" onclick={() => setActiveAccount(acc.id)}>
              {$t("account.switchAccount")}
            </button>
          {/if}
          <button class="ghost" onclick={() => removeAccount(acc.id)}>
            {$t("account.signOut")}
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .actions {
    flex-wrap: wrap;
  }

  .device {
    border-color: var(--accent);
  }

  .code-row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin: 0.6rem 0;
  }

  .code {
    font-size: 1.6rem;
    letter-spacing: 0.15em;
    font-weight: 700;
    padding: 0.3em 0.6em;
    background: var(--surface-2);
    border-radius: var(--radius-sm);
    user-select: text;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .acc {
    display: flex;
    align-items: center;
    gap: 0.9rem;
  }

  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    font-weight: 700;
    background: linear-gradient(135deg, var(--accent), var(--accent2));
    color: #fff;
  }

  .info {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .badge {
    font-size: 0.7rem;
    padding: 0.15em 0.6em;
    border-radius: 999px;
    background: color-mix(in srgb, var(--accent) 25%, transparent);
    color: var(--accent);
  }

  .small {
    font-size: 0.8rem;
  }

  .error {
    color: #ff9b9b;
  }
</style>
