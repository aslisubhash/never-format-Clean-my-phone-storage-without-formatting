use never_format_core::backup::BackupManager;
use never_format_core::cleanup::{CleanupManager, MoveManager};
use never_format_core::companion::{self, CompanionRequest};
use never_format_core::protected_paths;
use never_format_core::scanner::StorageScanner;
use never_format_core::transport::{DeviceTransport, MockTransport};
use never_format_core::types::{OpKind, OpState, SafetyClass};
use never_format_core::verify;
use never_format_core::whatsapp::WhatsAppManager;

#[test]
fn end_to_end_backup_verify_move_dry_cleanup() {
    let transport = MockTransport::default();
    let device_id = transport.device.id.clone();
    let files = StorageScanner::scan(&transport, &device_id).unwrap();
    assert!(!files.is_empty());

    let summary = StorageScanner::summarize(
        &device_id,
        transport.device.total_bytes,
        transport.device.used_bytes,
        transport.device.free_bytes,
        &files,
    );
    assert!(summary.safe_to_clean_bytes > 0 || summary.backup_then_remove_bytes > 0);

    let dir = tempfile::tempdir().unwrap();
    let dest = BackupManager::backup_root(dir.path(), &transport.device.name);
    let photo = files
        .iter()
        .find(|f| f.safety == SafetyClass::SafeAfterBackup)
        .expect("photo");
    let mut op = BackupManager::plan(&device_id, &[photo.path.clone()], &dest, false);
    BackupManager::execute(&transport, &device_id, &mut op).unwrap();
    assert_eq!(op.state, OpState::Completed);
    assert!(op.items[0].eligible_for_delete);
    assert!(op.items[0].dest_hash.is_some());

    let dest_path = op.items[0].dest_path.clone().unwrap();
    let hash = verify::sha256_file(std::path::Path::new(&dest_path)).unwrap();
    assert_eq!(op.items[0].dest_hash.as_deref(), Some(hash.as_str()));

    let mut move_op = BackupManager::plan(&device_id, &[photo.path.clone()], &dest, false);
    move_op.kind = OpKind::Move;
    MoveManager::execute(&transport, &device_id, &mut move_op, false).unwrap();
    assert_eq!(move_op.state, OpState::AwaitingConfirmation);
    MoveManager::execute(&transport, &device_id, &mut move_op, true).unwrap();
    assert_eq!(move_op.state, OpState::Completed);

    let mut clean = CleanupManager::plan(&device_id, &files, true);
    CleanupManager::execute(&transport, &device_id, &mut clean, true).unwrap();
    assert!(clean.items.iter().all(|i| !i.deleted));
}

#[test]
fn protected_whatsapp_latest_not_deleted() {
    let transport = MockTransport::default();
    let backups = WhatsAppManager::discover(&transport, &transport.device.id).unwrap();
    let latest = backups.iter().find(|b| b.is_latest).expect("latest");
    assert_eq!(latest.safety, SafetyClass::Protected);
    assert!(protected_paths::is_protected(&latest.path));
    assert!(transport
        .delete_file(&transport.device.id, &latest.path)
        .is_err());
}

#[test]
fn companion_rejects_wipe() {
    let resp = companion::dispatch(&CompanionRequest {
        op: "FACTORY_RESET".into(),
        args: serde_json::json!({}),
    });
    assert!(!resp.ok);
}

#[test]
fn path_traversal_blocked() {
    assert!(protected_paths::is_protected("/sdcard/../../system/bin"));
}
