
use crate::error::{FaFsError, FaFsResult};

#[derive(Debug, Clone)]
pub struct FaFsConfig {
    pub block_size: u32,
    pub fractal_depth: u32,
    pub max_inodes: u64,
    pub enable_compression: bool,
}

impl Default for FaFsConfig {
    fn default() -> Self {
        Self {
            block_size: 4096,
            fractal_depth: 8,
            max_inodes: 1_000_000,
            enable_compression: false,
        }
    }
}

impl FaFsConfig {
    pub fn validate(&self) -> FaFsResult<()> {
        if self.block_size < 512
            || !self.block_size.is_power_of_two()
        {
            return Err(FaFsError::InvalidConfiguration(
                "Block size must be a power of two >= 512".into(),
            ));
        }

        if self.fractal_depth == 0 {
            return Err(FaFsError::InvalidConfiguration(
                "Fractal depth must be greater than zero".into(),
            ));
        }

        if self.max_inodes == 0 {
            return Err(FaFsError::InvalidConfiguration(
                "Maximum inode count must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

