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
  background?: InstanceBackground;
  iconText?: string;
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
      value: "linear-gradient(135deg, #1a1f3a 0%, #2a1a3a 50%, #0e1016 100%)",
      dim: 0.1,
      blur: 0,
    },
    iconText: "S",
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
      value: "linear-gradient(135deg, #0c1411 0%, #10231a 60%, #0e1016 100%)",
      dim: 0.1,
    },
    iconText: "V",
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
