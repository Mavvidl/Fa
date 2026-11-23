
use crate::fractal::block_alloc::BlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub id: u64,
    pub inode_type: InodeType,
    pub size: u64,
    pub blocks: Vec<BlockId>,
    pub created_at: u64,
    pub modified_at: u64,
}

impl Inode {
    pub fn new(id: u64, inode_type: InodeType) -> Self {
        Self {
            id,
            inode_type,
            size: 0,
            blocks: Vec::new(),
            created_at: 0,
            modified_at: 0,
        }
    }

    pub fn add_block(&mut self, block: BlockId) {
        self.blocks.push(block);
    }
}

