// Instances are owner-defined presets. A player picks an instance and plays it;
// every launch setting (game version, loader, memory, JVM flags, mods, branding)
// is fixed by the instance. The ONLY thing a player may change is the Java
// runtime — and even that has a default supplied by the instance.
//
// Instances are bundled here for now; later phases sync them from the SoraWorks
// backend (Cloudflare R2 manifest) so the owner can publish changes remotely.

export type LoaderType = "vanilla" | "forge" | "neoforge" | "fabric" | "quilt";

export interface LoaderSpec {
  type: LoaderType;
  /** Loader version; omitted/ignored for vanilla. */
  version?: string;
}

export interface InstanceBackground {
  type: "gradient" | "image" | "video" | "color";
  /** CSS gradient/color string, or an asset URL for image/video. */
  value: string;
  /** 0..1 dark overlay strength over the background. */
  dim?: number;
  /** Background blur in px. */
  blur?: number;
}

export interface Instance {
  id: string;
  name: string;
  description?: string;

  /** Locked: the game version this instance runs. */
  gameVersion: string;
  /** Locked: the mod loader and its version. */
  loader: LoaderSpec;
  /** Locked: heap in MB (0 = auto-size from system RAM). */
  memoryMb: number;
  /** Locked: extra JVM args (optimizer flags etc.). */
  jvmArgs: string[];

  /** Recommended Java major; the player may override this (and only this). */
  javaMajor: number;
  /** Whether the player is allowed to override the Java runtime. */
  allowJavaOverride: boolean;

  /** Optional quick-join server address. */
  serverAddress?: string;

  // --- Branding (the instance's look) ---
  accentColor?: string;
  accentColor2?: string;
  /** Full-window ambient background (the big art behind everything). */
  background?: InstanceBackground;
  /** Card/banner cover: an image URL, or a CSS gradient string. */
  cover?: string;
  iconText?: string;
  /** Small square icon image URL (shown in the left rail); falls back to iconText. */
  iconImage?: string;

  // --- Live-ish stats (synced from backend later; stubbed now) ---
  playersOnline?: number;

  // --- Community links (shown in the social dock) ---
  links?: { discord?: string; website?: string; instagram?: string; email?: string };
}

/**
 * Bundled default instances. The primary one is the SoraWorks Forge 1.20.1
 * modpack/server. A plain vanilla instance is included so players can launch
 * immediately while loader support is finalized.
 */
export const BUNDLED_INSTANCES: Instance[] = [
  {
    id: "soraworks-forge-1201",
    name: "SoraWorks",
    description: "เซิร์ฟเวอร์หลัก — Forge 1.20.1",
    gameVersion: "1.20.1",
    loader: { type: "forge", version: "47.3.0" },
    memoryMb: 0,
    jvmArgs: [],
    javaMajor: 17,
    allowJavaOverride: true,
    serverAddress: "play.soraworks.net",
    accentColor: "#6c8cff",
    accentColor2: "#9d6cff",
    background: {
      type: "gradient",
      value:
        "radial-gradient(900px 600px at 75% 20%, rgba(108,140,255,0.35), transparent), radial-gradient(700px 500px at 20% 90%, rgba(157,108,255,0.3), transparent), linear-gradient(135deg, #12152a, #0e1016)",
      dim: 0.18,
      blur: 0,
    },
    cover:
      "radial-gradient(400px 300px at 70% 25%, rgba(108,140,255,0.55), transparent), linear-gradient(135deg, #2a2f5a, #1a1430)",
    iconText: "S",
    playersOnline: 0,
    links: { discord: "https://discord.gg/soraworks", website: "https://soraworks.net" },
  },
  {
    id: "vanilla-1201",
    name: "Vanilla 1.20.1",
    description: "ไม่มีม็อด — ทดสอบการเปิดเกม",
    gameVersion: "1.20.1",
    loader: { type: "vanilla" },
    memoryMb: 2048,
    jvmArgs: [],
    javaMajor: 17,
    allowJavaOverride: true,
    accentColor: "#2fcf7f",
    accentColor2: "#17b0c0",
    background: {
      type: "gradient",
      value:
        "radial-gradient(800px 600px at 70% 15%, rgba(47,207,127,0.3), transparent), linear-gradient(135deg, #0d1a14, #0e1016)",
      dim: 0.16,
    },
    cover: "radial-gradient(400px 300px at 70% 25%, rgba(47,207,127,0.5), transparent), linear-gradient(135deg, #123026, #0e1a16)",
    iconText: "V",
    playersOnline: 0,
  },
];

/** Java runtimes a player may pick from when overriding is allowed. */
export const SELECTABLE_JAVA: { major: number; label: string }[] = [
  { major: 8, label: "Java 8" },
  { major: 11, label: "Java 11" },
  { major: 17, label: "Java 17" },
  { major: 21, label: "Java 21" },
];

export function loaderLabel(loader: LoaderSpec): string {
  switch (loader.type) {
    case "forge":
      return `Forge ${loader.version ?? ""}`.trim();
    case "neoforge":
      return `NeoForge ${loader.version ?? ""}`.trim();
    case "fabric":
      return `Fabric ${loader.version ?? ""}`.trim();
    case "quilt":
      return `Quilt ${loader.version ?? ""}`.trim();
    default:
      return "Vanilla";
  }
}
