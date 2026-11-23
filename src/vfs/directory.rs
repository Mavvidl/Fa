
use std::collections::BTreeMap;

use crate::error::{FaFsError, FaFsResult};

#[derive(Debug, Clone)]
pub struct Directory {
    pub inode_id: u64,
    entries: BTreeMap<String, u64>,
}

impl Directory {
    pub fn new(inode_id: u64) -> Self {
        Self {
            inode_id,
            entries: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        name: String,
        inode_id: u64,
    ) -> FaFsResult<()> {
        if name.is_empty()
            || name == "."
            || name == ".."
            || name.contains('/')
        {
            return Err(FaFsError::InvalidConfiguration(
                "Invalid directory entry name".into(),
            ));
        }

        if self.entries.contains_key(&name) {
            return Err(FaFsError::InvalidConfiguration(
                "Directory entry already exists".into(),
            ));
        }

        self.entries.insert(name, inode_id);

        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<u64> {
        self.entries.get(name).copied()
    }

    pub fn remove(&mut self, name: &str) -> Option<u64> {
        self.entries.remove(name)
    }

    pub fn entries(&self) -> &BTreeMap<String, u64> {
        &self.entries
    }
}

