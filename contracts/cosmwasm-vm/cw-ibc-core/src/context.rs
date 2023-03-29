use super::*;
pub struct CwIbcCoreContext<'a> {
    block_height: Item<'a, u64>,
    cw_ibc_store: CwIbcStore<'a>,
    cw_ibc_router: CwIbcRouter<'a>,
}
impl<'a> Default for CwIbcCoreContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn new() -> Self {
        Self {
            block_height: Item::new(StorageKey::BlockHeight.as_str()),
            cw_ibc_store: CwIbcStore::default(),
            cw_ibc_router: CwIbcRouter::default(),
        }
    }
    pub fn ibc_store(&self) -> &CwIbcStore {
        &self.cw_ibc_store
    }

    pub fn ibc_router(&self) -> &CwIbcRouter {
        &self.cw_ibc_router
    }
    pub fn block_height(&self) -> &Item<'a, u64> {
        &self.block_height
    }
}
