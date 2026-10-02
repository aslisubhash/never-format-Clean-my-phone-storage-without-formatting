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

        // Free-space gate using known sizes from scan (skip if all unknown).
        let need: u64 = op.items.iter().map(|i| i.size_bytes).sum();
        if need > 0 {
            if let Some(first_dest) = op.items.iter().find_map(|i| i.dest_path.as_ref()) {
                let dest_path = PathBuf::from(first_dest);
                let check_path = dest_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| dest_path.clone());
                std::fs::create_dir_all(&check_path)?;
                ensure_disk_space(&check_path, need)?;
            }
        }

        for item in &mut op.items {
            let dest = item
                .dest_path
                .as_ref()
                .ok_or_else(|| Error::Other("missing dest".into()))?;
            let dest_path = PathBuf::from(dest);

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
                            // After adb pull, local file is the verified copy.
                            item.source_hash = item.dest_hash.clone();
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

/// Best-effort free-space check. If space cannot be determined, allow and let IO fail later.
pub fn ensure_disk_space(path: &Path, need: u64) -> Result<()> {
    if need == 0 {
        return Ok(());
    }
    let Some(have) = free_bytes(path) else {
        return Ok(());
    };
    if have < need {
        return Err(Error::InsufficientSpace { need, have });
    }
    Ok(())
}

fn free_bytes(path: &Path) -> Option<u64> {
    #[cfg(windows)]
    {
        free_bytes_windows(path)
    }
    #[cfg(unix)]
    {
        free_bytes_unix(path)
    }
    #[cfg(not(any(windows, unix)))]
    {
        let _ = path;
        None
    }
}

#[cfg(windows)]
fn free_bytes_windows(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetDiskFreeSpaceExW(
            lp_directory_name: *const u16,
            lp_free_bytes_available_to_caller: *mut u64,
            lp_total_number_of_bytes: *mut u64,
            lp_total_number_of_free_bytes: *mut u64,
        ) -> i32;
    }

    let mut probe = path.to_path_buf();
    if !probe.exists() {
        // Walk up to an existing ancestor (volume root is fine).
        while let Some(parent) = probe.parent() {
            if parent == probe {
                break;
            }
            probe = parent.to_path_buf();
            if probe.exists() {
                break;
            }
        }
    }

    let wide: Vec<u16> = probe
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut free_for_caller = 0u64;
    let mut total = 0u64;
    let mut free_total = 0u64;
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut free_for_caller,
            &mut total,
            &mut free_total,
        )
    };
    if ok != 0 {
        Some(free_for_caller)
    } else {
        None
    }
}

#[cfg(unix)]
fn free_bytes_unix(path: &Path) -> Option<u64> {
    use std::process::Command;
    let output = Command::new("df")
        .args(["-k", "-P"])
        .arg(path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Filesystem 1024-blocks Used Available Capacity Mounted on
    let line = stdout.lines().nth(1)?;
    let cols: Vec<&str> = line.split_whitespace().collect();
    if cols.len() < 4 {
        return None;
    }
    let avail_kb: u64 = cols[3].parse().ok()?;
    Some(avail_kb.saturating_mul(1024))
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
