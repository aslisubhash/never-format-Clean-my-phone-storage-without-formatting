//! Protected path policy — enforced in Rust, never trusted from UI alone.

use crate::error::{Error, Result};

const PROTECTED_PREFIXES: &[&str] = &[
    "/system",
    "/vendor",
    "/product",
    "/metadata",
    "/data/data",
    "/data/user",
    "/data/misc",
    "/proc",
    "/dev",
    "/sys",
];

const PROTECTED_SUFFIXES: &[&str] = &[
    "msgstore.db",
    "msgstore.db.crypt14",
    "msgstore.db.crypt15",
    "wa.db",
    "axolotl.db",
];

pub fn is_protected(path: &str) -> bool {
    let normalized = normalize(path);
    if PROTECTED_PREFIXES
        .iter()
        .any(|p| normalized == *p || normalized.starts_with(&format!("{p}/")))
    {
        return true;
    }
    // Exact current WhatsApp DB (not dated backups)
    let name = normalized.rsplit('/').next().unwrap_or("");
    if PROTECTED_SUFFIXES.contains(&name) {
        return true;
    }
    // Path traversal
    if normalized.contains("..") {
        return true;
    }
    false
}

pub fn assert_deletable(path: &str) -> Result<()> {
    if is_protected(path) {
        return Err(Error::ProtectedPath(path.to_string()));
    }
    Ok(())
}

pub fn normalize(path: &str) -> String {
    path.replace('\\', "/")
        .trim_end_matches('/')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protects_system() {
        assert!(is_protected("/system/bin/sh"));
        assert!(is_protected("/data/data/com.whatsapp"));
    }

    #[test]
    fn allows_user_media() {
        assert!(!is_protected("/sdcard/DCIM/Camera/a.jpg"));
        assert!(!is_protected("/sdcard/Download/file.pdf"));
    }

    #[test]
    fn protects_current_whatsapp_db() {
        assert!(is_protected(
            "/sdcard/WhatsApp/Databases/msgstore.db.crypt15"
        ));
        assert!(!is_protected(
            "/sdcard/WhatsApp/Databases/msgstore-2026-01-01.1.db.crypt15"
        ));
    }

    #[test]
    fn protects_traversal() {
        assert!(is_protected("/sdcard/../system"));
    }
}
