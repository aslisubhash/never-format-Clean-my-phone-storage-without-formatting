use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("device not found")]
    DeviceNotFound,

    #[error("adb not available: {0}")]
    AdbUnavailable(String),

    #[error("transport error: {0}")]
    Transport(String),

    #[error("path is protected: {0}")]
    ProtectedPath(String),

    #[error("operation not eligible: {0}")]
    NotEligible(String),

    #[error("verification failed: {0}")]
    VerificationFailed(String),

    #[error("unsupported capability: {0}")]
    UnsupportedCapability(String),

    #[error("operation rejected: {0}")]
    Rejected(String),

    #[error("insufficient disk space: need {need} bytes, have {have}")]
    InsufficientSpace { need: u64, have: u64 },

    #[error("lock poisoned")]
    LockPoisoned,

    #[error("{0}")]
    Other(String),
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::LockPoisoned
    }
}
