
use crate::error::FaFsResult;
use crate::vfs::inode::Inode;

pub trait FileOperations {
    fn read(
        &self,
        inode: &Inode,
        offset: u64,
        buffer: &mut [u8],
    ) -> FaFsResult<usize>;

    fn write(
        &mut self,
        inode: &mut Inode,
        offset: u64,
        data: &[u8],
    ) -> FaFsResult<usize>;

    fn truncate(
        &mut self,
        inode: &mut Inode,
        new_size: u64,
    ) -> FaFsResult<()>;
}

