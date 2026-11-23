
use crate::error::FaFsResult;

pub type BlockId = u64;

pub trait BlockAllocator {
    fn allocate(&mut self) -> FaFsResult<BlockId>;

    fn release(&mut self, block: BlockId) -> FaFsResult<()>;

    fn is_allocated(&self, block: BlockId) -> bool;

    fn allocated_count(&self) -> usize;
}

