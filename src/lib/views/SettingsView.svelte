<script lang="ts">
  import { t } from "../i18n";
  import { theme, setTheme, updateTheme, setColor, exportTheme, importTheme } from "../stores/theme";
  import { PRESETS, getPreset } from "../theme/presets";
  import { settings, patchSettings } from "../stores/settings";
  import { save, open } from "@tauri-apps/plugin-dialog";
  import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";

  let importError = $state("");

  const colorFields: { key: keyof typeof $theme.colors; label: string }[] = [
    { key: "accent", label: "settings.accentColor" },
  ];

  function applyMode(mode: "light" | "dark") {
    const preset = getPreset(mode === "dark" ? "sora-dark" : "sora-light");
    if (preset) setTheme(preset);
  }

  async function doExport() {
    try {
      const path = await save({
        defaultPath: "soraworks-theme.json",
        filters: [{ name: "Theme", extensions: ["json"] }],
      });
      if (path) await writeTextFile(path, exportTheme());
    } catch (e) {
      console.error(e);
    }
  }

  async function doImport() {
    importError = "";
    try {
      const path = await open({
        filters: [{ name: "Theme", extensions: ["json"] }],
        multiple: false,
      });
      if (typeof path === "string") {
        const content = await readTextFile(path);
        importTheme(content);
      }
    } catch (e) {
      importError = String(e);
    }
  }
</script>

<div class="stack settings">
  <section class="card stack">
    <h3>{$t("settings.appearance")}</h3>

    <div class="field">
      <span class="flabel">{$t("settings.theme")}</span>
      <div class="presets">
        {#each PRESETS as p}
          <button
            class="preset ghost"
            class:active={$theme.id === p.id}
            onclick={() => setTheme(p)}
            title={p.name}
          >
            <span class="swatch" style:background={`linear-gradient(135deg, ${p.colors.accent}, ${p.colors.accent2})`}></span>
            {p.name}
          </button>
        {/each}
      </div>
    </div>

    <div class="field">
      <span class="flabel">{$t("settings.mode")}</span>
      <div class="row">
        <button class="ghost" class:active={$theme.mode === "light"} onclick={() => applyMode("light")}>
          {$t("settings.modeLight")}
        </button>
        <button class="ghost" class:active={$theme.mode === "dark"} onclick={() => applyMode("dark")}>
          {$t("settings.modeDark")}
        </button>
      </div>
    </div>

    {#each colorFields as f}
      <div class="field row spread">
        <span class="flabel">{$t(f.label)}</span>
        <input
          type="color"
          value={$theme.colors[f.key]}
          oninput={(e) => setColor(f.key, (e.target as HTMLInputElement).value)}
        />
      </div>
    {/each}

    <div class="field">
      <span class="flabel">{$t("settings.fontScale")} — {$theme.fontScale.toFixed(2)}×</span>
      <input
        type="range" min="0.85" max="1.3" step="0.05"
        value={$theme.fontScale}
        oninput={(e) => updateTheme({ fontScale: +(e.target as HTMLInputElement).value })}
      />
    </div>

    <div class="field">
      <span class="flabel">{$t("settings.cornerRadius")} — {$theme.radius}px</span>
      <input
        type="range" min="0" max="24" step="1"
        value={$theme.radius}
        oninput={(e) => updateTheme({ radius: +(e.target as HTMLInputElement).value })}
      />
    </div>

    <div class="field">
      <span class="flabel">Surface opacity — {Math.round($theme.surfaceOpacity * 100)}%</span>
      <input
        type="range" min="0" max="0.4" step="0.02"
        value={$theme.surfaceOpacity}
        oninput={(e) => updateTheme({ surfaceOpacity: +(e.target as HTMLInputElement).value })}
      />
    </div>

    <div class="row">
      <button class="ghost" onclick={doExport}>Export theme</button>
      <button class="ghost" onclick={doImport}>Import theme</button>
    </div>
    {#if importError}<span class="error">{importError}</span>{/if}
  </section>

  <section class="card stack">
    <h3>{$t("settings.title")}</h3>

    <label class="field row spread">
      <div>
        <div>{$t("settings.lowSpec")}</div>
        <div class="muted small">{$t("settings.lowSpecDesc")}</div>
      </div>
      <input
        type="checkbox"
        checked={$settings.lowSpec}
        onchange={(e) => patchSettings({ lowSpec: (e.target as HTMLInputElement).checked })}
      />
    </label>

    <label class="field row spread">
      <span>{$t("settings.uiSounds")}</span>
      <input
        type="checkbox"
        checked={$settings.uiSounds}
        onchange={(e) => patchSettings({ uiSounds: (e.target as HTMLInputElement).checked })}
      />
    </label>

    <p class="muted small">🔒 {$t("instance.lockedHint")}</p>
  </section>
</div>

<style>
  .settings {
    max-width: 640px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .field.row {
    flex-direction: row;
    align-items: center;
  }

  .flabel {
    font-size: 0.95rem;
  }

  .presets {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .preset {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border: 1px solid var(--border);
  }

  .preset.active {
    border-color: var(--accent);
  }

  .swatch {
    width: 18px;
    height: 18px;
    border-radius: 5px;
  }

  button.active {
    border: 1px solid var(--accent);
    color: var(--text);
  }

  .small {
    font-size: 0.8rem;
  }

  .error {
    color: #ff9b9b;
  }
</style>
