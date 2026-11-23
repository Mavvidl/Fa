
use crate::error::FaFsResult;

pub trait BlockDevice: Send + Sync {
    fn block_size(&self) -> u32;

    fn block_count(&self) -> u64;

    fn read_block(
        &self,
        block_id: u64,
        buffer: &mut [u8],
    ) -> FaFsResult<()>;

    fn write_block(
        &self,
        block_id: u64,
        data: &[u8],
    ) -> FaFsResult<()>;

    fn flush(&self) -> FaFsResult<()>;
}

