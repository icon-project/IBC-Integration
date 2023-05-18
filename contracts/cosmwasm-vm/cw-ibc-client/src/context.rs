use super::*;
/// The `CwIbcClientContext` struct represents the core context of a Cosmos SDK contract for
/// inter-blockchain communication.
///
/// Properties:
///
/// * `block_height`: A storage item representing the current block height.
/// * `cw_ibc_store`: `cw_ibc_store` is an instance of the `CwIbcStore` struct, which is used to manage
/// the storage of the contract. It likely contains methods for reading and writing data to the
/// contract's storage.
/// * `cw_ibc_router`: `cw_ibc_router` is a field of type `CwIbcRouter<'a>` in the `CwIbcClientContext`
/// struct. It is an instance of the `CwIbcRouter` struct, which is used for routing IBC packets between
/// different chains. The
/// * `owner`: `owner` is a field of type `Item<'a, Addr>` in the `CwIbcClientContext` struct. It
/// represents the address of the owner of the contract. The `Item` type is a wrapper around a value
/// stored in the contract's storage, with a key that
pub struct CwIbcClientContext<'a> {
    block_height: Item<'a, u64>,
    cw_ibc_store: CwIbcStore<'a>,
    owner: Item<'a, Addr>,
}
impl<'a> Default for CwIbcClientContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcClientContext<'a> {
    pub fn new() -> Self {
        Self {
            block_height: Item::new(StorageKey::BlockHeight.as_str()),
            cw_ibc_store: CwIbcStore::default(),
            owner: Item::new(StorageKey::Owner.as_str()),
        }
    }
    pub fn ibc_store(&self) -> &CwIbcStore {
        &self.cw_ibc_store
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
