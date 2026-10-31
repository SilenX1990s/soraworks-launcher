<script lang="ts">
  import { t } from "../i18n";
  import { openUrl } from "@tauri-apps/plugin-opener";

  const version = "0.1.0";
</script>

<div class="stack about">
  <div class="card stack">
    <h2>{$t("app.name")}</h2>
    <p class="muted">{$t("app.tagline")}</p>
    <p>v{version} — phase 1 foundation</p>
    <p class="muted small">
      {$t("account.noOfflineMode")}
    </p>
    <div class="row">
      <button
        class="ghost"
        onclick={() => openUrl("https://github.com/SilenX1990s/soraworks-launcher")}
      >
        GitHub
      </button>
    </div>
  </div>
</div>

<style>
  .about {
    max-width: 560px;
  }
  .small {
    font-size: 0.8rem;
  }
</style>
