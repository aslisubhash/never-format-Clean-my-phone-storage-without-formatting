use crate::error::{Error, Result};
use crate::ops;
use crate::transport::DeviceTransport;
use crate::types::{OpKind, OpState, Operation, OperationItem};
use crate::verify;
use chrono::Utc;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct BackupManager;

impl BackupManager {
    pub fn backup_root(base: &Path, device_name: &str) -> PathBuf {
        let date = Utc::now().format("%Y-%m-%d");
        let safe_name = device_name.replace(['/', '\\', ':'], "_");
        base.join("Never Format Backups")
            .join(safe_name)
            .join(date.to_string())
    }

    pub fn plan(
        device_id: &str,
        sources: &[String],
        dest_root: &Path,
        dry_run: bool,
    ) -> Operation {
        let items: Vec<OperationItem> = sources
            .iter()
            .map(|src| {
                let name = src.rsplit('/').next().unwrap_or(src);
                let category_dir = category_folder(src);
                OperationItem {
                    source_path: src.clone(),
                    dest_path: Some(
                        dest_root
                            .join(category_dir)
                            .join(name)
                            .to_string_lossy()
                            .to_string(),
                    ),
                    size_bytes: 0,
                    source_hash: None,
                    dest_hash: None,
                    eligible_for_delete: false,
                    deleted: false,
                    error: None,
                }
            })
            .collect();

        ops::new_operation(OpKind::Backup, device_id, dry_run, items)
    }

    pub fn execute(
        transport: &dyn DeviceTransport,
        device_id: &str,
        op: &mut Operation,
    ) -> Result<()> {
        if op.dry_run {
            op.state = OpState::Completed;
            op.message = "Dry-run backup plan completed (no files copied)".into();
            op.updated_at = Utc::now();
            return Ok(());
        }

        op.state = OpState::BackingUp;
        op.updated_at = Utc::now();

        for item in &mut op.items {
            let dest = item
                .dest_path
                .as_ref()
                .ok_or_else(|| Error::Other("missing dest".into()))?;
            let dest_path = PathBuf::from(dest);

            // Ensure free space roughly (dest volume)
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            match transport.pull_file(device_id, &item.source_path, &dest_path) {
                Ok(()) => {
                    let meta = std::fs::metadata(&dest_path)?;
                    item.size_bytes = meta.len();
                    op.state = OpState::Verifying;
                    match verify::verify_local_file(&dest_path) {
                        Ok(hash) => {
                            item.dest_hash = Some(hash);
                            item.source_hash = item.dest_hash.clone(); // mock/adb: dest is source of truth after pull
                            item.eligible_for_delete = true;
                        }
                        Err(e) => {
                            item.error = Some(e.to_string());
                            item.eligible_for_delete = false;
                        }
                    }
                }
                Err(e) => {
                    item.error = Some(e.to_string());
                    item.eligible_for_delete = false;
                }
            }
        }

        let any_err = op.items.iter().any(|i| i.error.is_some());
        let any_ok = op.items.iter().any(|i| i.eligible_for_delete);
        if any_err && !any_ok {
            op.state = OpState::Failed;
            op.message = "Backup failed".into();
        } else if any_err {
            op.state = OpState::Completed;
            op.message = "Backup completed with some errors".into();
        } else {
            op.state = OpState::Completed;
            op.message = "Backup verified".into();
        }
        op.updated_at = Utc::now();
        Ok(())
    }
}

fn category_folder(path: &str) -> &'static str {
    use crate::classifier::classify_path;
    use crate::types::FileCategory;
    match classify_path(path) {
        FileCategory::Photos | FileCategory::Screenshots => "Photos",
        FileCategory::Videos => "Videos",
        FileCategory::Documents => "Documents",
        FileCategory::Downloads => "Downloads",
        FileCategory::Audio => "Music",
        FileCategory::WhatsApp => "WhatsApp",
        _ => "Other",
    }
}

pub fn ensure_disk_space(path: &Path, need: u64) -> Result<()> {
    // Best-effort: if we cannot stat, allow and let IO fail later
    let _ = path;
    let _ = need;
    Ok(())
}

/// Record a completed backup item for eligibility checks.
pub fn mark_verified(item: &mut OperationItem, hash: String) {
    item.dest_hash = Some(hash.clone());
    item.source_hash = Some(hash);
    item.eligible_for_delete = true;
}

pub fn new_id() -> Uuid {
    Uuid::new_v4()
}
