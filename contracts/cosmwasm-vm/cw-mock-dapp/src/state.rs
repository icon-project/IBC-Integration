use crate::types::StorageKey;

use super::*;

pub struct CwMockService<'a> {
    sequence: Item<'a, u64>,
    xcall_address: Item<'a, String>,
    rollback: Map<'a, u64, Vec<u8>>,
}

impl<'a> Default for CwMockService<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwMockService<'a> {
    pub fn new() -> Self {
        Self {
            sequence: Item::new(StorageKey::SequenceNo.as_str()),
            xcall_address: Item::new(StorageKey::Address.as_str()),
            rollback: Map::new(StorageKey::RollBack.as_str()),
        }
    }

    pub fn sequence(&self) -> &Item<'a, u64> {
        &self.sequence
    }

    pub fn xcall_address(&self) -> &Item<'a, String> {
        &self.xcall_address
    }

    pub fn roll_back(&self) -> &Map<'a, u64, Vec<u8>> {
        &self.rollback
    }
}
