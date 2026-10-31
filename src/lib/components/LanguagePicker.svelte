<script lang="ts">
  import { locale, setLocale, LOCALES, type Locale } from "../i18n";

  function onChange(e: Event) {
    const value = (e.target as HTMLSelectElement).value as Locale;
    setLocale(value);
  }
</script>

<select value={$locale} onchange={onChange} aria-label="Language">
  {#each LOCALES as l}
    <option value={l.id}>{l.flag} {l.label}</option>
  {/each}
</select>

<style>
  select {
    min-width: 120px;
  }
</style>
