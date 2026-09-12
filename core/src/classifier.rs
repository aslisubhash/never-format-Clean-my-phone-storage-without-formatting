use crate::protected_paths;
use crate::types::{FileCategory, SafetyClass};

pub fn classify_path(path: &str) -> FileCategory {
    let lower = path.to_lowercase();
    let name = lower.rsplit('/').next().unwrap_or("");

    if lower.contains("/whatsapp/") || lower.contains("com.whatsapp") {
        return FileCategory::WhatsApp;
    }
    if lower.contains("/cache/") || lower.contains("/.cache/") {
        return FileCategory::Cache;
    }
    if lower.contains("/temp/") || lower.contains("/tmp/") || name.ends_with(".tmp") {
        return FileCategory::TemporaryFiles;
    }
    if lower.contains("/screenshot") || name.contains("screenshot") {
        return FileCategory::Screenshots;
    }
    if lower.contains("/dcim/") || lower.contains("/pictures/") || is_ext(name, &["jpg", "jpeg", "png", "heic", "webp"]) {
        if lower.contains("/movies/") || is_ext(name, &["mp4", "mkv", "mov", "webm"]) {
            return FileCategory::Videos;
        }
        return FileCategory::Photos;
    }
    if lower.contains("/movies/") || lower.contains("/video") || is_ext(name, &["mp4", "mkv", "mov", "webm", "avi"]) {
        return FileCategory::Videos;
    }
    if lower.contains("/music/") || is_ext(name, &["mp3", "m4a", "flac", "wav", "aac", "ogg"]) {
        return FileCategory::Audio;
    }
    if lower.contains("/download") {
        return FileCategory::Downloads;
    }
    if is_ext(name, &["apk"]) {
        return FileCategory::Apks;
    }
    if is_ext(name, &["zip", "rar", "7z", "tar", "gz"]) {
        return FileCategory::Archives;
    }
    if is_ext(name, &["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt"]) {
        return FileCategory::Documents;
    }
    FileCategory::Other
}

pub fn safety_for(category: &FileCategory, path: &str) -> SafetyClass {
    if protected_paths::is_protected(path) {
        return SafetyClass::Protected;
    }
    match category {
        FileCategory::Cache | FileCategory::TemporaryFiles => SafetyClass::SafeToClean,
        FileCategory::Photos
        | FileCategory::Videos
        | FileCategory::Documents
        | FileCategory::Downloads
        | FileCategory::Audio
        | FileCategory::Screenshots => SafetyClass::SafeAfterBackup,
        FileCategory::Apks | FileCategory::Archives | FileCategory::LargeFiles | FileCategory::DuplicateFiles => {
            SafetyClass::ReviewRequired
        }
        FileCategory::WhatsApp => {
            let name = path.rsplit('/').next().unwrap_or("");
            if name.starts_with("msgstore-") {
                SafetyClass::SafeAfterBackup
            } else if name.starts_with("msgstore") {
                SafetyClass::Protected
            } else {
                SafetyClass::ReviewRequired
            }
        }
        FileCategory::Other => SafetyClass::Unknown,
    }
}

fn is_ext(name: &str, exts: &[&str]) -> bool {
    exts.iter().any(|e| name.ends_with(&format!(".{e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_photo() {
        assert_eq!(
            classify_path("/sdcard/DCIM/Camera/a.jpg"),
            FileCategory::Photos
        );
    }

    #[test]
    fn unknown_is_not_safe() {
        let s = safety_for(&FileCategory::Other, "/sdcard/mystery.bin");
        assert_eq!(s, SafetyClass::Unknown);
    }
}
