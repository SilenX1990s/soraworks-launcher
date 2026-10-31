<script lang="ts">
  let { path }: { path: string } = $props();

  const groups: { title: string; items: { href: string; label: string; icon: string }[] }[] = [
    {
      title: "ภาพรวม",
      items: [
        { href: "#/overview", label: "ภาพรวม", icon: "▦" },
        { href: "#/instances", label: "อินสแตนซ์", icon: "◼" },
      ],
    },
    {
      title: "จัดการเวิร์กสเปซ",
      items: [
        { href: "#/members", label: "สมาชิก", icon: "◉" },
        { href: "#/settings", label: "ตั้งค่า", icon: "⚙" },
        { href: "#/storage", label: "R2 storage", icon: "⛁" },
        { href: "#/developers", label: "นักพัฒนา", icon: "⌘" },
      ],
    },
  ];

  function active(href: string): boolean {
    const base = href.replace("#", "");
    return path === base || (base !== "/overview" && path.startsWith(base));
  }
</script>

<aside>
  <div class="brand">
    <div class="mark">S</div>
    <div class="bn">
      <strong>SoraWorks</strong>
      <span class="muted">คอนโซลเวิร์กสเปซ</span>
    </div>
  </div>

  {#each groups as g}
    <div class="group">
      <div class="gt">{g.title}</div>
      {#each g.items as it}
        <a class="item" class:active={active(it.href)} href={it.href}>
          <span class="ic">{it.icon}</span>{it.label}
        </a>
      {/each}
    </div>
  {/each}

  <div class="foot muted">อัปเดตม็อดแพ็กอย่างปลอดภัย แล้วกลับไปเล่นเกมได้เลย</div>
</aside>

<style>
  aside {
    width: 248px;
    flex-shrink: 0;
    background: var(--surface);
    border-right: 1px solid var(--border);
    padding: 1rem 0.8rem;
    display: flex;
    flex-direction: column;
    gap: 1.3rem;
    min-height: 100vh;
  }
  .brand { display: flex; align-items: center; gap: 0.6rem; padding: 0.3rem; }
  .mark {
    width: 38px; height: 38px; border-radius: 11px; display: grid; place-items: center;
    font-weight: 800; color: #fff; background: linear-gradient(135deg, var(--accent), var(--accent2));
  }
  .bn { display: flex; flex-direction: column; line-height: 1.15; }
  .bn .muted { font-size: 0.72rem; }
  .group { display: flex; flex-direction: column; gap: 0.15rem; }
  .gt { font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-muted); padding: 0.3rem 0.7rem; }
  .item {
    display: flex; align-items: center; gap: 0.7rem; padding: 0.55rem 0.8rem;
    border-radius: var(--radius-sm); color: var(--text); text-decoration: none; font-size: 0.92rem;
  }
  .item:hover { background: var(--surface-2); }
  .item.active { background: color-mix(in srgb, var(--accent) 18%, transparent); color: #fff; }
  .ic { width: 1.1em; text-align: center; opacity: 0.85; }
  .foot { margin-top: auto; font-size: 0.72rem; padding: 0.6rem; }
</style>
