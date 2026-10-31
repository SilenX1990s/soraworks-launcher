// Types shared with the Rust backend. Keep field names in sync with the
// `serde` structs in src-tauri/src (all serialized as camelCase).

export interface Account {
  /** Minecraft profile UUID (no dashes). */
  id: string;
  /** Minecraft username (gamertag). */
  name: string;
  /** Whether this is the currently selected account. */
  active: boolean;
  /** Unix seconds when the Minecraft access token expires. */
  expiresAt: number;
}

export interface DeviceCodeInfo {
  /** Opaque code the app uses to poll for completion. */
  deviceCode: string;
  /** Short code the user types into the verification page. */
  userCode: string;
  /** URL the user opens to enter the code. */
  verificationUri: string;
  /** Seconds until the codes expire. */
  expiresIn: number;
  /** Minimum seconds between polls. */
  interval: number;
}

export type VersionType = "release" | "snapshot" | "old_beta" | "old_alpha";

export interface VersionEntry {
  id: string;
  type: VersionType;
  /** ISO-8601 release time. */
  releaseTime: string;
}

export interface VersionManifest {
  latestRelease: string;
  latestSnapshot: string;
  versions: VersionEntry[];
}

export interface JavaInfo {
  /** Major Java version, e.g. 8, 17, 21. */
  major: number;
  /** Absolute path to the java executable. */
  path: string;
}

export interface LaunchOptions {
  versionId: string;
  accountId: string;
  /** Max heap in MB. */
  maxMemoryMb: number;
  /** Extra JVM args (optimizer flags, etc.). */
  extraJvmArgs?: string[];
  /** Force a specific Java major version (player override); 0 = auto by version. */
  javaMajor?: number;
}

/** Payload of the "launch://progress" event. */
export interface LaunchProgress {
  stage:
    | "preparing"
    | "java"
    | "libraries"
    | "assets"
    | "natives"
    | "launching"
    | "running";
  done: number;
  total: number;
}

/** Payload of the "launch://exit" event. */
export interface LaunchExit {
  code: number | null;
}
