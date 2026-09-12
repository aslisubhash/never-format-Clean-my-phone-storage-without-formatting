//! Never Format core library.
//!
//! Safety rule: if an operation cannot be proven safe, do nothing.

pub mod backup;
pub mod classifier;
pub mod cleanup;
pub mod companion;
pub mod db;
pub mod device;
pub mod error;
pub mod ops;
pub mod protected_paths;
pub mod reports;
pub mod scanner;
pub mod transport;
pub mod types;
pub mod verify;
pub mod whatsapp;

pub use error::{Error, Result};
pub use types::*;

use db::Database;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<Database>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn open(data_dir: impl AsRef<Path>) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("never-format.db");
        let db = Database::open(&db_path)?;
        Ok(Self {
            db: Mutex::new(db),
            data_dir,
        })
    }

    pub fn default_data_dir() -> PathBuf {
        directories::ProjectDirs::from("com", "NeverFormat", "Never Format")
            .map(|d| d.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("./never-format-data"))
    }
}
