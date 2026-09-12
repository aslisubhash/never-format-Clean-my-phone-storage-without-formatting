//! Allowlisted companion protocol. Never a remote shell.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompanionOp {
    ScanStorage,
    ListFiles,
    ReadFileMetadata,
    CopyFile,
    VerifyFile,
    RequestCacheCleanup,
    ExecuteApprovedCleanup,
    GetWhatsappBackups,
    ListPhotos,
    ExportPhoto,
    DeletePhotoAfterVerify,
    Handshake,
}

const FORBIDDEN: &[&str] = &[
    "FORMAT_DEVICE",
    "WIPE_STORAGE",
    "FACTORY_RESET",
    "DELETE_ARBITRARY_PATH",
    "UNLOCK_BOOTLOADER",
    "CLEAR_APP_DATA",
    "SHELL",
    "EXEC",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionRequest {
    pub op: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeInfo {
    pub protocol_version: u32,
    pub platform: String,
    pub capabilities: Vec<String>,
    pub app_version: String,
}

pub fn is_forbidden(op: &str) -> bool {
    let upper = op.to_uppercase();
    FORBIDDEN.iter().any(|f| upper == *f || upper.contains(f))
}

pub fn parse_op(op: &str) -> Option<CompanionOp> {
    if is_forbidden(op) {
        return None;
    }
    match op.to_uppercase().as_str() {
        "SCAN_STORAGE" => Some(CompanionOp::ScanStorage),
        "LIST_FILES" => Some(CompanionOp::ListFiles),
        "READ_FILE_METADATA" => Some(CompanionOp::ReadFileMetadata),
        "COPY_FILE" => Some(CompanionOp::CopyFile),
        "VERIFY_FILE" => Some(CompanionOp::VerifyFile),
        "REQUEST_CACHE_CLEANUP" => Some(CompanionOp::RequestCacheCleanup),
        "EXECUTE_APPROVED_CLEANUP" => Some(CompanionOp::ExecuteApprovedCleanup),
        "GET_WHATSAPP_BACKUPS" => Some(CompanionOp::GetWhatsappBackups),
        "LIST_PHOTOS" => Some(CompanionOp::ListPhotos),
        "EXPORT_PHOTO" => Some(CompanionOp::ExportPhoto),
        "DELETE_PHOTO_AFTER_VERIFY" => Some(CompanionOp::DeletePhotoAfterVerify),
        "HANDSHAKE" => Some(CompanionOp::Handshake),
        _ => None,
    }
}

pub fn dispatch(req: &CompanionRequest) -> CompanionResponse {
    if is_forbidden(&req.op) {
        return CompanionResponse {
            ok: false,
            error: Some(format!("rejected forbidden op: {}", req.op)),
            data: serde_json::Value::Null,
        };
    }
    let Some(op) = parse_op(&req.op) else {
        return CompanionResponse {
            ok: false,
            error: Some(format!("unknown op: {}", req.op)),
            data: serde_json::Value::Null,
        };
    };

    match op {
        CompanionOp::Handshake => CompanionResponse {
            ok: true,
            error: None,
            data: serde_json::to_value(HandshakeInfo {
                protocol_version: 1,
                platform: "desktop-simulator".into(),
                capabilities: android_capabilities(),
                app_version: env!("CARGO_PKG_VERSION").into(),
            })
            .unwrap_or(serde_json::Value::Null),
        },
        _ => CompanionResponse {
            ok: true,
            error: None,
            data: serde_json::json!({ "accepted": true, "op": req.op }),
        },
    }
}

pub fn android_capabilities() -> Vec<String> {
    vec![
        "scan_storage".into(),
        "list_files".into(),
        "read_file_metadata".into(),
        "copy_file".into(),
        "verify_file".into(),
        "request_cache_cleanup".into(),
        "execute_approved_cleanup".into(),
        "get_whatsapp_backups".into(),
    ]
}

pub fn ios_capabilities() -> Vec<String> {
    vec![
        "list_photos".into(),
        "export_photo".into(),
        "delete_photo_after_verify".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_factory_reset() {
        assert!(is_forbidden("FACTORY_RESET"));
        assert!(parse_op("FACTORY_RESET").is_none());
    }

    #[test]
    fn allows_scan() {
        assert_eq!(parse_op("SCAN_STORAGE"), Some(CompanionOp::ScanStorage));
    }
}
