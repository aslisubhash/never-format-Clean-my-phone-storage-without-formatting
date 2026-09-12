use crate::classifier;
use crate::error::Result;
use crate::transport::DeviceTransport;
use crate::types::{SafetyClass, WhatsAppBackupInfo};

const WA_DB_PATHS: &[&str] = &[
    "/sdcard/WhatsApp/Databases",
    "/sdcard/Android/media/com.whatsapp/WhatsApp/Databases",
];

pub struct WhatsAppManager;

impl WhatsAppManager {
    pub fn discover(
        transport: &dyn DeviceTransport,
        device_id: &str,
    ) -> Result<Vec<WhatsAppBackupInfo>> {
        let mut backups = Vec::new();
        for root in WA_DB_PATHS {
            let files = match transport.list_files(device_id, root) {
                Ok(f) => f,
                Err(_) => continue,
            };
            for f in files {
                if !f.name.contains("msgstore") {
                    continue;
                }
                backups.push(WhatsAppBackupInfo {
                    path: f.path.clone(),
                    size_bytes: f.size_bytes,
                    is_latest: false,
                    modified_at: f.modified_at,
                    safety: classifier::safety_for(&f.category, &f.path),
                });
            }
        }

        // Also scan mock listing for WhatsApp category
        if backups.is_empty() {
            let files = transport.list_files(device_id, "/sdcard")?;
            for f in files {
                if f.name.contains("msgstore") {
                    backups.push(WhatsAppBackupInfo {
                        path: f.path.clone(),
                        size_bytes: f.size_bytes,
                        is_latest: false,
                        modified_at: f.modified_at,
                        safety: classifier::safety_for(&f.category, &f.path),
                    });
                }
            }
        }

        // Mark current (non-dated) as latest/protected
        for b in &mut backups {
            let name = b.path.rsplit('/').next().unwrap_or("");
            if name.starts_with("msgstore.db") && !name.starts_with("msgstore-") {
                b.is_latest = true;
                b.safety = SafetyClass::Protected;
            }
        }

        // If none marked latest, mark largest as latest protected
        if !backups.iter().any(|b| b.is_latest) {
            if let Some(max) = backups.iter_mut().max_by_key(|b| b.size_bytes) {
                max.is_latest = true;
                max.safety = SafetyClass::Protected;
            }
        }

        Ok(backups)
    }

    pub fn removable(backups: &[WhatsAppBackupInfo]) -> Vec<&WhatsAppBackupInfo> {
        backups
            .iter()
            .filter(|b| !b.is_latest && b.safety != SafetyClass::Protected)
            .collect()
    }
}
