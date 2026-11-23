
use std::sync::Mutex;

use crate::error::{FaFsError, FaFsResult};
use crate::storage::block_device::BlockDevice;

pub struct MemoryDevice {
    block_size: u32,
    storage: Mutex<Vec<u8>>,
    block_count: u64,
}

impl MemoryDevice {
    pub fn new(
        block_size: u32,
        block_count: u64,
    ) -> FaFsResult<Self> {
        if block_size == 0 || block_count == 0 {
            return Err(FaFsError::InvalidConfiguration(
                "Invalid memory device dimensions".into(),
            ));
        }

        let total_size = usize::try_from(
            u64::from(block_size) * block_count
        ).map_err(|_| {
            FaFsError::InvalidConfiguration(
                "Memory device is too large".into(),
            )
        })?;

        Ok(Self {
            block_size,
            storage: Mutex::new(vec![0; total_size]),
            block_count,
        })
    }
}

impl BlockDevice for MemoryDevice {
    fn block_size(&self) -> u32 {
        self.block_size
    }

    fn block_count(&self) -> u64 {
        self.block_count
    }

    fn read_block(
        &self,
        block_id: u64,
        buffer: &mut [u8],
    ) -> FaFsResult<()> {
        if block_id >= self.block_count {
            return Err(FaFsError::InvalidBlockAddress(block_id));
        }

        if buffer.len() != self.block_size as usize {
            return Err(FaFsError::StorageError(
                "Invalid buffer size".into(),
            ));
        }

        let offset = block_id as usize * self.block_size as usize;
        let storage = self.storage.lock().map_err(|_| {
            FaFsError::StorageError(
                "Memory device lock poisoned".into(),
            )
        })?;

        buffer.copy_from_slice(
            &storage[offset..offset + buffer.len()]
        );

        Ok(())
    }

    fn write_block(
        &self,
        block_id: u64,
        data: &[u8],
    ) -> FaFsResult<()> {
        if block_id >= self.block_count {
            return Err(FaFsError::InvalidBlockAddress(block_id));
        }

        if data.len() != self.block_size as usize {
            return Err(FaFsError::StorageError(
                "Invalid data size".into(),
            ));
        }

        let offset = block_id as usize * self.block_size as usize;
        let mut storage = self.storage.lock().map_err(|_| {
            FaFsError::StorageError(
                "Memory device lock poisoned".into(),
            )
        })?;

        storage[offset..offset + data.len()].copy_from_slice(data);

        Ok(())
    }

    fn flush(&self) -> FaFsResult<()> {
        Ok(())
    }
}

