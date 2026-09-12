//! Device transport abstraction. UI never talks to ADB/shell directly.

use crate::error::{Error, Result};
use crate::types::{DeviceInfo, PlatformKind, ScannedFile};
use std::path::Path;
use std::process::Command;

pub trait DeviceTransport: Send + Sync {
    fn platform(&self) -> PlatformKind;
    fn name(&self) -> &str;
    fn list_devices(&self) -> Result<Vec<DeviceInfo>>;
    fn storage_totals(&self, device_id: &str) -> Result<(u64, u64, u64)>;
    fn list_files(&self, device_id: &str, remote_path: &str) -> Result<Vec<ScannedFile>>;
    fn pull_file(&self, device_id: &str, remote: &str, local: &Path) -> Result<()>;
    fn delete_file(&self, device_id: &str, remote: &str) -> Result<()>;
    fn capabilities(&self) -> Vec<String>;
}

/// ADB-backed Android transport. Uses structured argv — never shell string concat.
pub struct AdbTransport {
    adb_path: String,
}

impl AdbTransport {
    pub fn detect() -> Result<Self> {
        let candidates = ["adb", "adb.exe"];
        for c in candidates {
            if Command::new(c).arg("version").output().is_ok() {
                return Ok(Self {
                    adb_path: c.to_string(),
                });
            }
        }
        // Common Windows / Homebrew locations still try `adb` via PATH failure message
        Err(Error::AdbUnavailable(
            "adb not found on PATH. Install Android platform-tools.".into(),
        ))
    }

    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            adb_path: path.into(),
        }
    }

    fn run(&self, args: &[&str]) -> Result<String> {
        let output = Command::new(&self.adb_path)
            .args(args)
            .output()
            .map_err(|e| Error::AdbUnavailable(e.to_string()))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Transport(stderr));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn run_device(&self, serial: &str, args: &[&str]) -> Result<String> {
        let mut full = vec!["-s", serial];
        full.extend_from_slice(args);
        self.run(&full)
    }
}

impl DeviceTransport for AdbTransport {
    fn platform(&self) -> PlatformKind {
        PlatformKind::Android
    }

    fn name(&self) -> &str {
        "adb"
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>> {
        let out = self.run(&["devices", "-l"])?;
        let mut devices = Vec::new();
        for line in out.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 || parts[1] != "device" {
                continue;
            }
            let serial = parts[0].to_string();
            let model = parts
                .iter()
                .find_map(|p| p.strip_prefix("model:"))
                .unwrap_or("Android Device")
                .replace('_', " ");
            let (total, used, free) = self
                .storage_totals(&serial)
                .unwrap_or((0, 0, 0));
            devices.push(DeviceInfo {
                id: serial.clone(),
                name: model.clone(),
                model,
                serial,
                platform: PlatformKind::Android,
                transport: "adb".into(),
                total_bytes: total,
                used_bytes: used,
                free_bytes: free,
                adb_available: true,
                mtp_available: false,
                companion_connected: false,
                capabilities: self.capabilities(),
            });
        }
        Ok(devices)
    }

    fn storage_totals(&self, device_id: &str) -> Result<(u64, u64, u64)> {
        let out = self.run_device(device_id, &["shell", "df", "/data"])?;
        // Parse df output; last data line typically has 1K-blocks
        for line in out.lines().skip(1) {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() >= 4 {
                let total = cols[1].parse::<u64>().unwrap_or(0) * 1024;
                let used = cols[2].parse::<u64>().unwrap_or(0) * 1024;
                let free = cols[3].parse::<u64>().unwrap_or(0) * 1024;
                if total > 0 {
                    return Ok((total, used, free));
                }
            }
        }
        Ok((0, 0, 0))
    }

    fn list_files(&self, device_id: &str, remote_path: &str) -> Result<Vec<ScannedFile>> {
        // Use ls -l; avoid shell metacharacters by validating path
        if remote_path.contains('`') || remote_path.contains('$') || remote_path.contains('|') {
            return Err(Error::Rejected("invalid remote path".into()));
        }
        let out = self.run_device(device_id, &["shell", "ls", "-la", remote_path])?;
        let mut files = Vec::new();
        for line in out.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 8 {
                continue;
            }
            // permissions links owner group size date time name
            let size: u64 = cols[4].parse().unwrap_or(0);
            let name = cols[7..].join(" ");
            if name == "." || name == ".." || name.is_empty() {
                continue;
            }
            if cols[0].starts_with('d') {
                continue;
            }
            let path = format!(
                "{}/{}",
                remote_path.trim_end_matches('/'),
                name
            );
            let category = crate::classifier::classify_path(&path);
            let safety = crate::classifier::safety_for(&category, &path);
            files.push(ScannedFile {
                path,
                name,
                size_bytes: size,
                category,
                safety,
                modified_at: None,
            });
        }
        Ok(files)
    }

    fn pull_file(&self, device_id: &str, remote: &str, local: &Path) -> Result<()> {
        if let Some(parent) = local.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let local_str = local.to_string_lossy();
        self.run_device(device_id, &["pull", remote, local_str.as_ref()])?;
        Ok(())
    }

    fn delete_file(&self, device_id: &str, remote: &str) -> Result<()> {
        crate::protected_paths::assert_deletable(remote)?;
        self.run_device(device_id, &["shell", "rm", "--", remote])?;
        Ok(())
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "scan_storage".into(),
            "list_files".into(),
            "pull_file".into(),
            "delete_file".into(),
            "whatsapp".into(),
            "cache_cleanup".into(),
        ]
    }
}

/// MTP probe placeholder — signals availability; full MTP in later iterations.
pub struct MtpTransport {
    available: bool,
}

impl MtpTransport {
    pub fn probe() -> Self {
        // On Windows, MTP devices appear via WPD; without native bindings we report unknown.
        Self { available: false }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }
}

impl DeviceTransport for MtpTransport {
    fn platform(&self) -> PlatformKind {
        PlatformKind::Android
    }

    fn name(&self) -> &str {
        "mtp"
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>> {
        Ok(Vec::new())
    }

    fn storage_totals(&self, _device_id: &str) -> Result<(u64, u64, u64)> {
        Err(Error::UnsupportedCapability("mtp_storage".into()))
    }

    fn list_files(&self, _device_id: &str, _remote_path: &str) -> Result<Vec<ScannedFile>> {
        Err(Error::UnsupportedCapability("mtp_list".into()))
    }

    fn pull_file(&self, _device_id: &str, _remote: &str, _local: &Path) -> Result<()> {
        Err(Error::UnsupportedCapability("mtp_pull".into()))
    }

    fn delete_file(&self, _device_id: &str, _remote: &str) -> Result<()> {
        Err(Error::UnsupportedCapability("mtp_delete".into()))
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["media_browse".into()]
    }
}

/// iOS transport stub (libimobiledevice-class). P1: Photos/Files subset only.
pub struct IosTransport {
    available: bool,
}

impl IosTransport {
    pub fn probe() -> Self {
        let available = Command::new("idevice_id")
            .arg("-l")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        Self { available }
    }
}

impl DeviceTransport for IosTransport {
    fn platform(&self) -> PlatformKind {
        PlatformKind::Ios
    }

    fn name(&self) -> &str {
        "ios"
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>> {
        if !self.available {
            return Ok(Vec::new());
        }
        let output = Command::new("idevice_id").arg("-l").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();
        for serial in stdout.lines().filter(|l| !l.trim().is_empty()) {
            devices.push(DeviceInfo {
                id: serial.to_string(),
                name: "iPhone".into(),
                model: "iPhone".into(),
                serial: serial.to_string(),
                platform: PlatformKind::Ios,
                transport: "ios".into(),
                total_bytes: 0,
                used_bytes: 0,
                free_bytes: 0,
                adb_available: false,
                mtp_available: false,
                companion_connected: false,
                capabilities: self.capabilities(),
            });
        }
        Ok(devices)
    }

    fn storage_totals(&self, _device_id: &str) -> Result<(u64, u64, u64)> {
        Err(Error::UnsupportedCapability(
            "ios_full_storage_scan".into(),
        ))
    }

    fn list_files(&self, _device_id: &str, _remote_path: &str) -> Result<Vec<ScannedFile>> {
        Err(Error::UnsupportedCapability(
            "ios_requires_companion_photos".into(),
        ))
    }

    fn pull_file(&self, _device_id: &str, _remote: &str, _local: &Path) -> Result<()> {
        Err(Error::UnsupportedCapability("ios_pull_via_companion".into()))
    }

    fn delete_file(&self, _device_id: &str, _remote: &str) -> Result<()> {
        Err(Error::UnsupportedCapability(
            "ios_delete_via_companion_only".into(),
        ))
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "list_photos".into(),
            "export_photo".into(),
            "delete_photo_after_verify".into(),
        ]
    }
}

/// Local mock transport for offline demos and unit tests.
pub struct MockTransport {
    pub device: DeviceInfo,
    pub files: Vec<ScannedFile>,
}

impl Default for MockTransport {
    fn default() -> Self {
        use crate::types::{FileCategory, SafetyClass};
        Self {
            device: DeviceInfo {
                id: "mock-device-001".into(),
                name: "Mock Galaxy".into(),
                model: "Mock Galaxy S24".into(),
                serial: "MOCK001".into(),
                platform: PlatformKind::Android,
                transport: "mock".into(),
                total_bytes: 128_u64 * 1024 * 1024 * 1024,
                used_bytes: 90_u64 * 1024 * 1024 * 1024,
                free_bytes: 38_u64 * 1024 * 1024 * 1024,
                adb_available: false,
                mtp_available: false,
                companion_connected: false,
                capabilities: vec![
                    "scan_storage".into(),
                    "list_files".into(),
                    "pull_file".into(),
                    "delete_file".into(),
                    "whatsapp".into(),
                    "cache_cleanup".into(),
                ],
            },
            files: vec![
                ScannedFile {
                    path: "/sdcard/DCIM/Camera/IMG_001.jpg".into(),
                    name: "IMG_001.jpg".into(),
                    size_bytes: 4_820_000,
                    category: FileCategory::Photos,
                    safety: SafetyClass::SafeAfterBackup,
                    modified_at: None,
                },
                ScannedFile {
                    path: "/sdcard/DCIM/Camera/VID_001.mp4".into(),
                    name: "VID_001.mp4".into(),
                    size_bytes: 420_000_000,
                    category: FileCategory::Videos,
                    safety: SafetyClass::ReviewRequired,
                    modified_at: None,
                },
                ScannedFile {
                    path: "/sdcard/Download/setup.apk".into(),
                    name: "setup.apk".into(),
                    size_bytes: 55_000_000,
                    category: FileCategory::Apks,
                    safety: SafetyClass::ReviewRequired,
                    modified_at: None,
                },
                ScannedFile {
                    path: "/sdcard/Android/data/com.example/cache/tmp.bin".into(),
                    name: "tmp.bin".into(),
                    size_bytes: 120_000_000,
                    category: FileCategory::Cache,
                    safety: SafetyClass::SafeToClean,
                    modified_at: None,
                },
                ScannedFile {
                    path: "/sdcard/WhatsApp/Databases/msgstore.db.crypt15".into(),
                    name: "msgstore.db.crypt15".into(),
                    size_bytes: 800_000_000,
                    category: FileCategory::WhatsApp,
                    safety: SafetyClass::Protected,
                    modified_at: None,
                },
                ScannedFile {
                    path: "/sdcard/WhatsApp/Databases/msgstore-2026-01-01.1.db.crypt15".into(),
                    name: "msgstore-2026-01-01.1.db.crypt15".into(),
                    size_bytes: 750_000_000,
                    category: FileCategory::WhatsApp,
                    safety: SafetyClass::SafeAfterBackup,
                    modified_at: None,
                },
            ],
        }
    }
}

impl DeviceTransport for MockTransport {
    fn platform(&self) -> PlatformKind {
        PlatformKind::Android
    }

    fn name(&self) -> &str {
        "mock"
    }

    fn list_devices(&self) -> Result<Vec<DeviceInfo>> {
        Ok(vec![self.device.clone()])
    }

    fn storage_totals(&self, _device_id: &str) -> Result<(u64, u64, u64)> {
        Ok((
            self.device.total_bytes,
            self.device.used_bytes,
            self.device.free_bytes,
        ))
    }

    fn list_files(&self, _device_id: &str, remote_path: &str) -> Result<Vec<ScannedFile>> {
        Ok(self
            .files
            .iter()
            .filter(|f| f.path.starts_with(remote_path) || remote_path == "/" || remote_path == "/sdcard")
            .cloned()
            .collect())
    }

    fn pull_file(&self, _device_id: &str, remote: &str, local: &Path) -> Result<()> {
        let file = self
            .files
            .iter()
            .find(|f| f.path == remote)
            .ok_or(Error::DeviceNotFound)?;
        if let Some(parent) = local.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Write deterministic mock content matching size (capped for tests)
        let content = format!("NEVERFORMAT_MOCK:{remote}:{}", file.size_bytes);
        std::fs::write(local, content.as_bytes())?;
        Ok(())
    }

    fn delete_file(&self, _device_id: &str, remote: &str) -> Result<()> {
        crate::protected_paths::assert_deletable(remote)?;
        let safety = self
            .files
            .iter()
            .find(|f| f.path == remote)
            .map(|f| f.safety)
            .unwrap_or(crate::types::SafetyClass::Unknown);
        if matches!(
            safety,
            crate::types::SafetyClass::Protected | crate::types::SafetyClass::Unknown
        ) {
            return Err(Error::NotEligible(format!(
                "refusing to delete {remote} ({safety:?})"
            )));
        }
        Ok(())
    }

    fn capabilities(&self) -> Vec<String> {
        self.device.capabilities.clone()
    }
}
