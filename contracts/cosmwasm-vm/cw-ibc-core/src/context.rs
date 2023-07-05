use cosmwasm_std::{from_slice, to_vec};
use serde::{de::DeserializeOwned, Serialize};

use super::*;
/// The `CwIbcCoreContext` struct represents the core context of a Cosmos SDK contract for
/// inter-blockchain communication.
///
/// Properties:
///
/// * `block_height`: A storage item representing the current block height.
/// * `cw_ibc_store`: `cw_ibc_store` is an instance of the `CwIbcStore` struct, which is used to manage
/// the storage of the contract. It likely contains methods for reading and writing data to the
/// contract's storage.
/// * `cw_ibc_router`: `cw_ibc_router` is a field of type `CwIbcRouter<'a>` in the `CwIbcCoreContext`
/// struct. It is an instance of the `CwIbcRouter` struct, which is used for routing IBC packets between
/// different chains. The
/// * `owner`: `owner` is a field of type `Item<'a, Addr>` in the `CwIbcCoreContext` struct. It
/// represents the address of the owner of the contract. The `Item` type is a wrapper around a value
/// stored in the contract's storage, with a key that
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

    pub fn store_callback_data<T>(
        &self,
        store: &mut dyn Storage,
        id: u64,
        data: &T,
    ) -> Result<(), ContractError>
    where
        T: Serialize + ?Sized,
    {
        let bytes = to_vec(data).map_err(ContractError::Std)?;
        return self
            .cw_ibc_store
            .callback_data()
            .save(store, id, &bytes)
            .map_err(ContractError::Std);
    }

    pub fn get_callback_data<T: DeserializeOwned>(
        &self,
        store: &dyn Storage,
        id: u64,
    ) -> Result<T, ContractError> {
        let bytes = self
            .cw_ibc_store
            .callback_data()
            .load(store, id)
            .map_err(ContractError::Std)?;
        let data = from_slice::<T>(&bytes).map_err(ContractError::Std)?;
        Ok(data)
    }
}
