use crate::error::{Error, Result};
use crate::ops;
use crate::protected_paths;
use crate::transport::DeviceTransport;
use crate::types::{OpKind, OpState, Operation, OperationItem, SafetyClass, ScannedFile};
use crate::verify;
use chrono::Utc;

pub struct CleanupManager;

impl CleanupManager {
    /// Build a dry-run or executable plan. Never includes Protected/Unknown.
    pub fn plan(device_id: &str, files: &[ScannedFile], dry_run: bool) -> Operation {
        let items: Vec<OperationItem> = files
            .iter()
            .filter(|f| {
                matches!(
                    f.safety,
                    SafetyClass::SafeToClean | SafetyClass::SafeAfterBackup
                ) && !protected_paths::is_protected(&f.path)
            })
            .filter(|f| {
                // SafeAfterBackup only if already eligible — caller should set; for dry-run show candidates
                matches!(f.safety, SafetyClass::SafeToClean)
                    || matches!(f.safety, SafetyClass::SafeAfterBackup)
            })
            .map(|f| OperationItem {
                source_path: f.path.clone(),
                dest_path: None,
                size_bytes: f.size_bytes,
                source_hash: None,
                dest_hash: None,
                eligible_for_delete: matches!(f.safety, SafetyClass::SafeToClean),
                deleted: false,
                error: None,
            })
            .collect();

        let mut op = ops::new_operation(OpKind::Cleanup, device_id, dry_run, items);
        op.state = OpState::Planned;
        op.message = format!(
            "Cleanup plan: {} item(s). Dry-run={}",
            op.items.len(),
            dry_run
        );
        op
    }

    pub fn execute(
        transport: &dyn DeviceTransport,
        device_id: &str,
        op: &mut Operation,
        confirmed: bool,
    ) -> Result<()> {
        if op.dry_run {
            op.state = OpState::Completed;
            op.message = "Dry-run only — no deletions performed".into();
            op.updated_at = Utc::now();
            return Ok(());
        }
        if !confirmed {
            op.state = OpState::AwaitingConfirmation;
            op.message = "Waiting for user confirmation".into();
            return Ok(());
        }

        op.state = OpState::Deleting;
        for item in &mut op.items {
            if !item.eligible_for_delete {
                item.error = Some("not eligible".into());
                continue;
            }
            if let Err(e) = protected_paths::assert_deletable(&item.source_path) {
                item.error = Some(e.to_string());
                continue;
            }
            match transport.delete_file(device_id, &item.source_path) {
                Ok(()) => {
                    item.deleted = true;
                }
                Err(e) => item.error = Some(e.to_string()),
            }
        }
        op.state = OpState::VerifyingResult;
        let deleted = op.items.iter().filter(|i| i.deleted).count();
        let failed = op.items.iter().filter(|i| i.error.is_some()).count();
        op.state = if failed > 0 && deleted == 0 {
            OpState::Failed
        } else {
            OpState::Completed
        };
        op.message = format!("Deleted {deleted}; failed {failed}");
        op.updated_at = Utc::now();
        Ok(())
    }
}

/// Verified Move: Copy → Verify → Confirm → Delete → Verify result
pub struct MoveManager;

impl MoveManager {
    pub fn execute(
        transport: &dyn DeviceTransport,
        device_id: &str,
        op: &mut Operation,
        confirmed: bool,
    ) -> Result<()> {
        use crate::backup::BackupManager;

        if op.kind != OpKind::Move {
            return Err(Error::Other("not a move operation".into()));
        }

        // Phase: backup/copy
        if matches!(
            op.state,
            OpState::Planned | OpState::Validated | OpState::BackingUp
        ) {
            let mut backup_op = op.clone();
            backup_op.kind = OpKind::Backup;
            BackupManager::execute(transport, device_id, &mut backup_op)?;
            op.items = backup_op.items;
            if backup_op.state == OpState::Failed {
                op.state = OpState::Failed;
                op.message = backup_op.message;
                return Ok(());
            }
            op.state = OpState::AwaitingConfirmation;
            op.message = "Copy verified — confirm delete from device".into();
            op.updated_at = Utc::now();
            if !confirmed {
                return Ok(());
            }
        }

        if op.state == OpState::AwaitingConfirmation && !confirmed {
            return Ok(());
        }

        op.state = OpState::Deleting;
        for item in &mut op.items {
            verify::assert_eligible(&item.source_hash, &item.dest_hash)?;
            protected_paths::assert_deletable(&item.source_path)?;
            match transport.delete_file(device_id, &item.source_path) {
                Ok(()) => item.deleted = true,
                Err(e) => item.error = Some(e.to_string()),
            }
        }
        op.state = OpState::Completed;
        op.message = "Verified move completed".into();
        op.updated_at = Utc::now();
        Ok(())
    }
}
