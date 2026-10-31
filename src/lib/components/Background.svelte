<script lang="ts">
  import type { Instance } from "../instances";
  import { settings } from "../stores/settings";

  let { instance }: { instance: Instance | null } = $props();

  const bg = $derived(instance?.background ?? null);
  // Low-spec mode turns off video and the animated blobs.
  const lite = $derived($settings.lowSpec);

  const dim = $derived(bg?.dim ?? 0.1);
  const blur = $derived(bg?.blur ?? 0);
</script>

<div class="bg" aria-hidden="true">
  {#if bg?.type === "video" && !lite}
    <video class="layer media" src={bg.value} autoplay muted loop playsinline></video>
  {:else if bg?.type === "image"}
    <div class="layer media" style:background-image={`url("${bg.value}")`}></div>
  {:else if bg?.type === "gradient"}
    <div class="layer" style:background={bg.value}></div>
  {:else if bg?.type === "color"}
    <div class="layer" style:background={bg.value}></div>
  {:else}
    <div class="layer default"></div>
  {/if}

  {#if !lite}
    <div class="blob b1"></div>
    <div class="blob b2"></div>
  {/if}

  <div
    class="overlay"
    style:backdrop-filter={blur > 0 ? `blur(${blur}px)` : "none"}
    style:background={`rgba(0,0,0,${dim})`}
  ></div>
</div>

<style>
  .bg {
    position: fixed;
    inset: 0;
    z-index: -1;
    overflow: hidden;
    background: var(--bg);
  }

  .layer {
    position: absolute;
    inset: 0;
  }

  .media {
    width: 100%;
    height: 100%;
    object-fit: cover;
    background-size: cover;
    background-position: center;
  }

  .default {
    background:
      radial-gradient(1200px 600px at 20% -10%, color-mix(in srgb, var(--accent) 22%, transparent), transparent),
      radial-gradient(900px 500px at 100% 0%, color-mix(in srgb, var(--accent2) 18%, transparent), transparent),
      var(--bg);
  }

  .overlay {
    position: absolute;
    inset: 0;
  }

  /* Soft floating color blobs for a bit of life (disabled in low-spec). */
  .blob {
    position: absolute;
    width: 42vw;
    height: 42vw;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.28;
    will-change: transform;
  }

  .b1 {
    top: -12vw;
    left: -8vw;
    background: var(--accent);
    animation: drift1 18s ease-in-out infinite alternate;
  }

  .b2 {
    bottom: -14vw;
    right: -8vw;
    background: var(--accent2);
    animation: drift2 22s ease-in-out infinite alternate;
  }

  @keyframes drift1 {
    from { transform: translate(0, 0) scale(1); }
    to { transform: translate(6vw, 5vw) scale(1.15); }
  }

  @keyframes drift2 {
    from { transform: translate(0, 0) scale(1.1); }
    to { transform: translate(-5vw, -4vw) scale(1); }
  }

  @media (prefers-reduced-motion: reduce) {
    .blob { animation: none; }
  }
</style>
