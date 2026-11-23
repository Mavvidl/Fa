
use thiserror::Error;

pub type FaFsResult<T> = Result<T, FaFsError>;

#[derive(Debug, Error)]
pub enum FaFsError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Invalid block address: {0}")]
    InvalidBlockAddress(u64),

    #[error("Inode not found: {0}")]
    InodeNotFound(u64),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Fractal computation error: {0}")]
    FractalError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Operation not supported: {0}")]
    NotSupported(String),
}


