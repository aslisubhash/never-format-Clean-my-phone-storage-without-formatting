use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformKind {
    Android,
    Ios,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyClass {
    SafeToClean,
    SafeAfterBackup,
    ReviewRequired,
    Protected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileCategory {
    Photos,
    Videos,
    Audio,
    Documents,
    Downloads,
    WhatsApp,
    Apks,
    Archives,
    Screenshots,
    LargeFiles,
    DuplicateFiles,
    TemporaryFiles,
    Cache,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub model: String,
    pub serial: String,
    pub platform: PlatformKind,
    pub transport: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub adb_available: bool,
    pub mtp_available: bool,
    pub companion_connected: bool,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub category: FileCategory,
    pub bytes: u64,
    pub file_count: u64,
    pub safety: SafetyClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSummary {
    pub device_id: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub categories: Vec<CategoryBreakdown>,
    pub safe_to_clean_bytes: u64,
    pub backup_then_remove_bytes: u64,
    pub needs_review_bytes: u64,
    pub protected_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub category: FileCategory,
    pub safety: SafetyClass,
    pub modified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpKind {
    Backup,
    Move,
    Cleanup,
    WhatsAppBackup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpState {
    Planned,
    Validated,
    BackingUp,
    Verifying,
    AwaitingConfirmation,
    Deleting,
    VerifyingResult,
    Completed,
    Failed,
    Cancelled,
    AbortedSafe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: Uuid,
    pub kind: OpKind,
    pub state: OpState,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message: String,
    pub dry_run: bool,
    pub items: Vec<OperationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationItem {
    pub source_path: String,
    pub dest_path: Option<String>,
    pub size_bytes: u64,
    pub source_hash: Option<String>,
    pub dest_hash: Option<String>,
    pub eligible_for_delete: bool,
    pub deleted: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppBackupInfo {
    pub path: String,
    pub size_bytes: u64,
    pub is_latest: bool,
    pub modified_at: Option<DateTime<Utc>>,
    pub safety: SafetyClass,
}
