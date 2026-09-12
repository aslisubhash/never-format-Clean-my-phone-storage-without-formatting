use never_format_core::backup::BackupManager;
use never_format_core::cleanup::{CleanupManager, MoveManager};
use never_format_core::companion::{self, CompanionRequest};
use never_format_core::device::DeviceManager;
use never_format_core::reports;
use never_format_core::scanner::StorageScanner;
use never_format_core::types::*;
use never_format_core::whatsapp::WhatsAppManager;
use never_format_core::AppState;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

struct Inner {
    state: AppState,
    last_files: Vec<ScannedFile>,
    last_summary: Option<StorageSummary>,
    backup_dest: PathBuf,
}

struct AppHandle(Mutex<Inner>);

#[derive(Serialize)]
struct AppInfo {
    name: &'static str,
    version: &'static str,
    offline: bool,
    safety_promise: &'static str,
}

#[tauri::command]
fn app_info() -> AppInfo {
    AppInfo {
        name: "Never Format",
        version: env!("CARGO_PKG_VERSION"),
        offline: true,
        safety_promise: "If Never Format cannot verify that an operation is safe, it does nothing.",
    }
}

#[tauri::command]
fn list_devices(app: State<AppHandle>) -> Result<Vec<DeviceInfo>, String> {
    let mgr = DeviceManager {
        include_mock: true,
    };
    let devices = mgr.discover().map_err(|e| e.to_string())?;
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let db = inner.state.db.lock().map_err(|e| e.to_string())?;
    for d in &devices {
        let _ = db.upsert_device(
            &d.id,
            &d.name,
            &d.model,
            &format!("{:?}", d.platform),
            &d.transport,
        );
    }
    Ok(devices)
}

#[tauri::command]
fn scan_device(app: State<AppHandle>, device_id: String) -> Result<StorageSummary, String> {
    let devices = DeviceManager {
        include_mock: true,
    }
    .discover()
    .map_err(|e| e.to_string())?;
    let device = devices
        .into_iter()
        .find(|d| d.id == device_id)
        .ok_or_else(|| "device not found".to_string())?;
    let transport = DeviceManager::transport_for(&device).map_err(|e| e.to_string())?;
    let files = StorageScanner::scan(transport.as_ref(), &device_id).map_err(|e| e.to_string())?;
    let (total, used, free) = transport
        .storage_totals(&device_id)
        .unwrap_or((device.total_bytes, device.used_bytes, device.free_bytes));
    let summary = StorageScanner::summarize(&device_id, total, used, free, &files);
    let mut inner = app.0.lock().map_err(|e| e.to_string())?;
    inner.last_files = files;
    inner.last_summary = Some(summary.clone());
    Ok(summary)
}

#[tauri::command]
fn list_scanned_files(app: State<AppHandle>) -> Result<Vec<ScannedFile>, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    Ok(inner.last_files.clone())
}

#[tauri::command]
fn large_files(app: State<AppHandle>, min_mb: u64) -> Result<Vec<ScannedFile>, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    Ok(StorageScanner::large_files(
        &inner.last_files,
        min_mb * 1024 * 1024,
    ))
}

#[tauri::command]
fn set_backup_destination(app: State<AppHandle>, path: String) -> Result<(), String> {
    let mut inner = app.0.lock().map_err(|e| e.to_string())?;
    inner.backup_dest = PathBuf::from(path);
    let db = inner.state.db.lock().map_err(|e| e.to_string())?;
    db.set_pref("backup_dest", &inner.backup_dest.to_string_lossy())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_backup_destination(app: State<AppHandle>) -> Result<String, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    Ok(inner.backup_dest.to_string_lossy().to_string())
}

#[tauri::command]
fn run_backup(
    app: State<AppHandle>,
    device_id: String,
    paths: Vec<String>,
    dry_run: bool,
) -> Result<Operation, String> {
    let devices = DeviceManager {
        include_mock: true,
    }
    .discover()
    .map_err(|e| e.to_string())?;
    let device = devices
        .into_iter()
        .find(|d| d.id == device_id)
        .ok_or_else(|| "device not found".to_string())?;
    let transport = DeviceManager::transport_for(&device).map_err(|e| e.to_string())?;

    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let dest_root = BackupManager::backup_root(&inner.backup_dest, &device.name);
    let mut op = BackupManager::plan(&device_id, &paths, &dest_root, dry_run);
    BackupManager::execute(transport.as_ref(), &device_id, &mut op).map_err(|e| e.to_string())?;

    let report = reports::from_operation(&op);
    {
        let db = inner.state.db.lock().map_err(|e| e.to_string())?;
        db.save_operation(&op).map_err(|e| e.to_string())?;
        db.save_report(&report).map_err(|e| e.to_string())?;
    }
    Ok(op)
}

#[tauri::command]
fn run_move(
    app: State<AppHandle>,
    device_id: String,
    paths: Vec<String>,
    confirmed: bool,
) -> Result<Operation, String> {
    let devices = DeviceManager {
        include_mock: true,
    }
    .discover()
    .map_err(|e| e.to_string())?;
    let device = devices
        .into_iter()
        .find(|d| d.id == device_id)
        .ok_or_else(|| "device not found".to_string())?;
    let transport = DeviceManager::transport_for(&device).map_err(|e| e.to_string())?;

    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let dest_root = BackupManager::backup_root(&inner.backup_dest, &device.name);
    let mut op = BackupManager::plan(&device_id, &paths, &dest_root, false);
    op.kind = OpKind::Move;
    MoveManager::execute(transport.as_ref(), &device_id, &mut op, confirmed)
        .map_err(|e| e.to_string())?;

    let report = reports::from_operation(&op);
    {
        let db = inner.state.db.lock().map_err(|e| e.to_string())?;
        db.save_operation(&op).map_err(|e| e.to_string())?;
        db.save_report(&report).map_err(|e| e.to_string())?;
    }
    Ok(op)
}

#[tauri::command]
fn plan_cleanup(app: State<AppHandle>, device_id: String, dry_run: bool) -> Result<Operation, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let mut op = CleanupManager::plan(&device_id, &inner.last_files, dry_run);
    if dry_run {
        CleanupManager::execute(
            DeviceManager::transport_for(&DeviceInfo {
                id: device_id.clone(),
                name: String::new(),
                model: String::new(),
                serial: String::new(),
                platform: PlatformKind::Android,
                transport: "mock".into(),
                total_bytes: 0,
                used_bytes: 0,
                free_bytes: 0,
                adb_available: false,
                mtp_available: false,
                companion_connected: false,
                capabilities: vec![],
            })
            .map_err(|e| e.to_string())?
            .as_ref(),
            &device_id,
            &mut op,
            false,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(op)
}

#[tauri::command]
fn run_cleanup(
    app: State<AppHandle>,
    device_id: String,
    confirmed: bool,
) -> Result<Operation, String> {
    let devices = DeviceManager {
        include_mock: true,
    }
    .discover()
    .map_err(|e| e.to_string())?;
    let device = devices
        .into_iter()
        .find(|d| d.id == device_id)
        .ok_or_else(|| "device not found".to_string())?;
    let transport = DeviceManager::transport_for(&device).map_err(|e| e.to_string())?;

    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let mut op = CleanupManager::plan(&device_id, &inner.last_files, false);
    // Only SafeToClean auto-eligible
    for item in &mut op.items {
        if let Some(f) = inner.last_files.iter().find(|f| f.path == item.source_path) {
            item.eligible_for_delete = matches!(f.safety, SafetyClass::SafeToClean);
        }
    }
    CleanupManager::execute(transport.as_ref(), &device_id, &mut op, confirmed)
        .map_err(|e| e.to_string())?;
    let report = reports::from_operation(&op);
    {
        let db = inner.state.db.lock().map_err(|e| e.to_string())?;
        db.save_operation(&op).map_err(|e| e.to_string())?;
        db.save_report(&report).map_err(|e| e.to_string())?;
    }
    Ok(op)
}

#[tauri::command]
fn list_whatsapp(device_id: String) -> Result<Vec<WhatsAppBackupInfo>, String> {
    let devices = DeviceManager {
        include_mock: true,
    }
    .discover()
    .map_err(|e| e.to_string())?;
    let device = devices
        .into_iter()
        .find(|d| d.id == device_id)
        .ok_or_else(|| "device not found".to_string())?;
    let transport = DeviceManager::transport_for(&device).map_err(|e| e.to_string())?;
    WhatsAppManager::discover(transport.as_ref(), &device_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_operations(app: State<AppHandle>) -> Result<Vec<Operation>, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let db = inner.state.db.lock().map_err(|e| e.to_string())?;
    db.list_operations(50).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_reports(app: State<AppHandle>) -> Result<Vec<Report>, String> {
    let inner = app.0.lock().map_err(|e| e.to_string())?;
    let db = inner.state.db.lock().map_err(|e| e.to_string())?;
    db.list_reports(50).map_err(|e| e.to_string())
}

#[tauri::command]
fn companion_dispatch(req: CompanionRequest) -> companion::CompanionResponse {
    companion::dispatch(&req)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_dir = AppState::default_data_dir();
    let state = AppState::open(&data_dir).expect("failed to open database");
    let backup_dest = {
        let db = state.db.lock().expect("db lock");
        db.get_pref("backup_dest")
            .ok()
            .flatten()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                directories::UserDirs::new()
                    .and_then(|u| u.document_dir().map(|d| d.to_path_buf()))
                    .unwrap_or_else(|| PathBuf::from("."))
            })
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppHandle(Mutex::new(Inner {
            state,
            last_files: Vec::new(),
            last_summary: None,
            backup_dest,
        })))
        .invoke_handler(tauri::generate_handler![
            app_info,
            list_devices,
            scan_device,
            list_scanned_files,
            large_files,
            set_backup_destination,
            get_backup_destination,
            run_backup,
            run_move,
            plan_cleanup,
            run_cleanup,
            list_whatsapp,
            list_operations,
            list_reports,
            companion_dispatch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Never Format");
}
