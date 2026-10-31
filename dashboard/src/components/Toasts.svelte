<script lang="ts">
  import { toasts } from "../lib/store";
</script>

<div class="wrap">
  {#each $toasts as t (t.id)}
    <div class="toast" class:error={t.kind === "error"}>{t.message}</div>
  {/each}
</div>

<style>
  .wrap {
    position: fixed;
    right: 1.2rem;
    bottom: 1.2rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    z-index: 50;
  }
  .toast {
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-left: 3px solid var(--accent);
    border-radius: var(--radius-sm);
    padding: 0.7rem 1rem;
    font-size: 0.9rem;
    box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.6);
  }
  .toast.error { border-left-color: #ff6b6b; }
</style>
