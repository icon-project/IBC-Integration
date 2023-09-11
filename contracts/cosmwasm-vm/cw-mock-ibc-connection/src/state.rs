use cw_storage_plus::Map;
use cw_xcall_lib::network_address::NetId;

use crate::types::{
    config::Config,
    network_fees::NetworkFees,
};

use super::*;

pub struct CwIbcConnection<'a> {
    owner: Item<'a, String>,
    config: Item<'a, Config>,
    admin: Item<'a, String>,
    ibc_host: Item<'a, Addr>,
    xcall_host: Item<'a, Addr>,
    network_fees: Map<'a, NetId, NetworkFees>,
}

impl<'a> Default for CwIbcConnection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcConnection<'a> {
    pub fn new() -> Self {
        Self {
            owner: Item::new(StorageKey::Owner.as_str()),
            config: Item::new(StorageKey::Config.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            ibc_host: Item::new(StorageKey::IbcHost.as_str()),
            xcall_host: Item::new(StorageKey::XCallHost.as_str()),
            network_fees: Map::new(StorageKey::NetworkFees.as_str()),
        }
    }

    pub fn owner(&self) -> &Item<'a, String> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, String> {
        &self.admin
    }

    pub fn get_config(&self, store: &dyn Storage) -> Result<Config, ContractError> {
        self.config.load(store).map_err(ContractError::Std)
    }

    pub fn store_config(
        &self,
        store: &mut dyn Storage,

        config: &Config,
    ) -> Result<(), ContractError> {
        self.config.save(store, config).map_err(ContractError::Std)
    }
    pub fn set_ibc_host(
        &self,
        store: &mut dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        self.ibc_host
            .save(store, &address)
            .map_err(ContractError::Std)
    }
    pub fn get_ibc_host(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        self.ibc_host.load(store).map_err(ContractError::Std)
    }

    pub fn set_xcall_host(
        &self,
        store: &mut dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        self.xcall_host
            .save(store, &address)
            .map_err(ContractError::Std)
    }
    pub fn get_xcall_host(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        self.xcall_host.load(store).map_err(ContractError::Std)
    }

    pub fn get_network_fees(&self, store: &dyn Storage, nid: NetId) -> NetworkFees {
        self.network_fees
            .load(store, nid)
            .unwrap_or(NetworkFees::default())
    }

    pub fn store_network_fees(
        &self,
        store: &mut dyn Storage,
        nid: NetId,
        network_fees: &NetworkFees,
    ) -> Result<(), ContractError> {
        self.network_fees
            .save(store, nid, network_fees)
            .map_err(ContractError::Std)
    }
}
