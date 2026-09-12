import { invoke } from "@tauri-apps/api/core";
import type {
  DeviceInfo,
  Operation,
  Report,
  ScannedFile,
  StorageSummary,
  WhatsAppBackupInfo,
} from "./types";

export const api = {
  appInfo: () =>
    invoke<{ name: string; version: string; offline: boolean; safety_promise: string }>(
      "app_info",
    ),
  listDevices: () => invoke<DeviceInfo[]>("list_devices"),
  scanDevice: (deviceId: string) => invoke<StorageSummary>("scan_device", { deviceId }),
  listFiles: () => invoke<ScannedFile[]>("list_scanned_files"),
  largeFiles: (minMb = 100) => invoke<ScannedFile[]>("large_files", { minMb }),
  getBackupDest: () => invoke<string>("get_backup_destination"),
  setBackupDest: (path: string) => invoke("set_backup_destination", { path }),
  runBackup: (deviceId: string, paths: string[], dryRun: boolean) =>
    invoke<Operation>("run_backup", { deviceId, paths, dryRun }),
  runMove: (deviceId: string, paths: string[], confirmed: boolean) =>
    invoke<Operation>("run_move", { deviceId, paths, confirmed }),
  planCleanup: (deviceId: string, dryRun: boolean) =>
    invoke<Operation>("plan_cleanup", { deviceId, dryRun }),
  runCleanup: (deviceId: string, confirmed: boolean) =>
    invoke<Operation>("run_cleanup", { deviceId, confirmed }),
  listWhatsapp: (deviceId: string) =>
    invoke<WhatsAppBackupInfo[]>("list_whatsapp", { deviceId }),
  listOperations: () => invoke<Operation[]>("list_operations"),
  listReports: () => invoke<Report[]>("list_reports"),
  companionDispatch: (op: string, args: unknown = {}) =>
    invoke<{ ok: boolean; error: string | null; data: unknown }>("companion_dispatch", {
      req: { op, args },
    }),
};
