
pub mod backend;
pub mod config;
pub mod error;
pub mod fractal;
pub mod storage;
pub mod vfs;

// Réexportation des éléments principaux.
pub use config::FaFsConfig;
pub use error::{FaFsError, FaFsResult};

