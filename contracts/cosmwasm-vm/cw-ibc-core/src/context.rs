use cosmwasm_std::Coin;

use super::*;
pub struct CwIbcCoreContext<'a> {
    block_height: Item<'a, u64>,
    cw_ibc_store: CwIbcStore<'a>,
    cw_ibc_router: CwIbcRouter<'a>,
    owner: Item<'a, Addr>,
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
            owner: Item::new(StorageKey::Owner.as_str()),
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
    pub fn owner(&self) -> &Item<'a, Addr> {
        &self.owner
    }

    pub fn check_sender_is_owner(
        &self,
        store: &dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        let owner = self.owner.load(store).map_err(ContractError::Std)?;

        if owner != address {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }
    pub fn set_owner(&self, store: &mut dyn Storage, address: Addr) -> Result<(), ContractError> {
        self.owner.save(store, &address).map_err(ContractError::Std)
    }
}
