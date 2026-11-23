
use crate::error::{FaFsError, FaFsResult};

#[derive(Debug, Clone)]
pub struct SierpinskiNode {
    pub id: u64,
    pub depth: u32,
    pub children: Vec<SierpinskiNode>,
}

impl SierpinskiNode {
    pub fn new(id: u64, depth: u32) -> Self {
        Self {
            id,
            depth,
            children: Vec::new(),
        }
    }

    pub fn add_child(
        &mut self,
        child: SierpinskiNode,
    ) -> FaFsResult<()> {
        if child.depth <= self.depth {
            return Err(FaFsError::FractalError(
                "Child depth must be greater than parent depth".into(),
            ));
        }

        self.children.push(child);

        Ok(())
    }

    pub fn find(&self, id: u64) -> Option<&SierpinskiNode> {
        if self.id == id {
            return Some(self);
        }

        self.children.iter().find_map(|child| {
            child.find(id)
        })
    }
}

