use super::*;
pub struct CwIbcCoreContext<'a> {
    cw_ibc_store: CwIbcStore<'a>,
    cw_ibc_router: CwIbcRouter<'a>,
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn new() -> Self {
        Self {
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
}
