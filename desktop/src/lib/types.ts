export type PlatformKind = "android" | "ios" | "unknown";
export type SafetyClass =
  | "safe_to_clean"
  | "safe_after_backup"
  | "review_required"
  | "protected"
  | "unknown";

export type FileCategory =
  | "photos"
  | "videos"
  | "audio"
  | "documents"
  | "downloads"
  | "whats_app"
  | "apks"
  | "archives"
  | "screenshots"
  | "large_files"
  | "duplicate_files"
  | "temporary_files"
  | "cache"
  | "other";

export interface DeviceInfo {
  id: string;
  name: string;
  model: string;
  serial: string;
  platform: PlatformKind;
  transport: string;
  total_bytes: number;
  used_bytes: number;
  free_bytes: number;
  adb_available: boolean;
  mtp_available: boolean;
  companion_connected: boolean;
  capabilities: string[];
}

export interface CategoryBreakdown {
  category: FileCategory;
  bytes: number;
  file_count: number;
  safety: SafetyClass;
}

export interface StorageSummary {
  device_id: string;
  total_bytes: number;
  used_bytes: number;
  free_bytes: number;
  categories: CategoryBreakdown[];
  safe_to_clean_bytes: number;
  backup_then_remove_bytes: number;
  needs_review_bytes: number;
  protected_bytes: number;
}

export interface ScannedFile {
  path: string;
  name: string;
  size_bytes: number;
  category: FileCategory;
  safety: SafetyClass;
  modified_at: string | null;
}

export interface OperationItem {
  source_path: string;
  dest_path: string | null;
  size_bytes: number;
  source_hash: string | null;
  dest_hash: string | null;
  eligible_for_delete: boolean;
  deleted: boolean;
  error: string | null;
}

export interface Operation {
  id: string;
  kind: string;
  state: string;
  device_id: string;
  created_at: string;
  updated_at: string;
  message: string;
  dry_run: boolean;
  items: OperationItem[];
}

export interface Report {
  id: string;
  operation_id: string;
  title: string;
  body: string;
  created_at: string;
  success: boolean;
}

export interface WhatsAppBackupInfo {
  path: string;
  size_bytes: number;
  is_latest: boolean;
  modified_at: string | null;
  safety: SafetyClass;
}

export function formatBytes(n: number): string {
  if (n <= 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let v = n;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i += 1;
  }
  return `${v.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

export function safetyLabel(s: SafetyClass): string {
  switch (s) {
    case "safe_to_clean":
      return "Safe";
    case "safe_after_backup":
      return "Backup required";
    case "review_required":
      return "Review";
    case "protected":
      return "Protected";
    default:
      return "Unknown";
  }
}

export function safetyColor(s: SafetyClass): string {
  switch (s) {
    case "safe_to_clean":
      return "text-pine bg-mist";
    case "safe_after_backup":
      return "text-amber bg-amber/10";
    case "review_required":
      return "text-amber bg-amber/15";
    case "protected":
      return "text-coral bg-coral/10";
    default:
      return "text-slate bg-mist";
  }
}
