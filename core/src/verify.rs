use crate::error::{Error, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const BUF: usize = 1024 * 1024;

/// Stream SHA-256 of a local file. Never loads entire file into memory.
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; BUF];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn verify_local_file(path: &Path) -> Result<String> {
    if !path.exists() {
        return Err(Error::VerificationFailed(format!(
            "missing file: {}",
            path.display()
        )));
    }
    let meta = std::fs::metadata(path)?;
    if meta.len() == 0 {
        // Empty file may be valid; still hash it
    }
    // Readable check
    let hash = sha256_file(path)?;
    let _ = File::open(path)?;
    Ok(hash)
}

pub fn hashes_match(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

pub fn assert_eligible(source_hash: &Option<String>, dest_hash: &Option<String>) -> Result<()> {
    match (source_hash, dest_hash) {
        (Some(s), Some(d)) if hashes_match(s, d) => Ok(()),
        _ => Err(Error::NotEligible(
            "delete requires matching verified hashes".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn hashes_deterministic() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.bin");
        let mut f = File::create(&p).unwrap();
        f.write_all(b"never-format").unwrap();
        let h1 = sha256_file(&p).unwrap();
        let h2 = sha256_file(&p).unwrap();
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}
