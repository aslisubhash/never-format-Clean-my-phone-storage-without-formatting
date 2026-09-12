use crate::classifier;
use crate::error::Result;
use crate::transport::DeviceTransport;
use crate::types::{
    CategoryBreakdown, FileCategory, SafetyClass, ScannedFile, StorageSummary,
};
use std::collections::HashMap;

const SCAN_ROOTS: &[&str] = &[
    "/sdcard/DCIM",
    "/sdcard/Pictures",
    "/sdcard/Download",
    "/sdcard/Movies",
    "/sdcard/Music",
    "/sdcard/Documents",
    "/sdcard/WhatsApp",
    "/sdcard/Android/data",
];

pub struct StorageScanner;

impl StorageScanner {
    pub fn scan(transport: &dyn DeviceTransport, device_id: &str) -> Result<Vec<ScannedFile>> {
        let mut all = Vec::new();
        for root in SCAN_ROOTS {
            match transport.list_files(device_id, root) {
                Ok(mut files) => all.append(&mut files),
                Err(_) => continue,
            }
        }
        // Also accept mock full listing
        if all.is_empty() {
            all = transport.list_files(device_id, "/sdcard")?;
        }
        for f in &mut all {
            f.category = classifier::classify_path(&f.path);
            f.safety = classifier::safety_for(&f.category, &f.path);
            if f.size_bytes >= 100_u64 * 1024 * 1024
                && !matches!(f.category, FileCategory::Videos | FileCategory::WhatsApp)
            {
                // Tag oversized non-video as large for dashboard
                if matches!(f.category, FileCategory::Other | FileCategory::Downloads | FileCategory::Archives) {
                    f.category = FileCategory::LargeFiles;
                    f.safety = SafetyClass::ReviewRequired;
                }
            }
        }
        Ok(all)
    }

    pub fn summarize(
        device_id: &str,
        total: u64,
        used: u64,
        free: u64,
        files: &[ScannedFile],
    ) -> StorageSummary {
        let mut map: HashMap<FileCategory, (u64, u64)> = HashMap::new();
        let mut safe_to_clean = 0u64;
        let mut backup_then = 0u64;
        let mut review = 0u64;
        let mut protected = 0u64;

        for f in files {
            let e = map.entry(f.category).or_insert((0, 0));
            e.0 += f.size_bytes;
            e.1 += 1;
            match f.safety {
                SafetyClass::SafeToClean => safe_to_clean += f.size_bytes,
                SafetyClass::SafeAfterBackup => backup_then += f.size_bytes,
                SafetyClass::ReviewRequired => review += f.size_bytes,
                SafetyClass::Protected => protected += f.size_bytes,
                SafetyClass::Unknown => protected += f.size_bytes, // treat unknown as protected budget
            }
        }

        let mut categories: Vec<CategoryBreakdown> = map
            .into_iter()
            .map(|(category, (bytes, file_count))| {
                let safety = files
                    .iter()
                    .find(|f| f.category == category)
                    .map(|f| f.safety)
                    .unwrap_or(SafetyClass::Unknown);
                CategoryBreakdown {
                    category,
                    bytes,
                    file_count,
                    safety,
                }
            })
            .collect();
        categories.sort_by_key(|a| std::cmp::Reverse(a.bytes));

        StorageSummary {
            device_id: device_id.to_string(),
            total_bytes: total,
            used_bytes: used,
            free_bytes: free,
            categories,
            safe_to_clean_bytes: safe_to_clean,
            backup_then_remove_bytes: backup_then,
            needs_review_bytes: review,
            protected_bytes: protected,
        }
    }

    pub fn large_files(files: &[ScannedFile], min_bytes: u64) -> Vec<ScannedFile> {
        let mut large: Vec<_> = files
            .iter()
            .filter(|f| f.size_bytes >= min_bytes)
            .cloned()
            .collect();
        large.sort_by_key(|a| std::cmp::Reverse(a.size_bytes));
        large
    }
}
