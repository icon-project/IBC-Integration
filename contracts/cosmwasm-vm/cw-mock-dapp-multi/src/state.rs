use crate::types::StorageKey;

use super::*;

#[cw_serde]
pub struct Connection {
    pub src_endpoint: String,
    pub dest_endpoint: String,
}

pub struct CwMockService<'a> {
    sequence: Item<'a, u64>,
    ibc_data: Map<'a, u64, Vec<u8>>,
    xcall_address: Item<'a, String>,
    rollback: Map<'a, u64, Vec<u8>>,
    connections: Map<'a, String, Vec<Connection>>,
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
            ibc_data: Map::new(StorageKey::Request.as_str()),
            xcall_address: Item::new(StorageKey::Address.as_str()),
            rollback: Map::new(StorageKey::RollBack.as_str()),
            connections: Map::new(StorageKey::Connections.as_str()),
        }
    }

    pub fn sequence(&self) -> &Item<'a, u64> {
        &self.sequence
    }

    pub fn ibc_data(&self) -> &Map<'a, u64, Vec<u8>> {
        &self.ibc_data
    }

    pub fn xcall_address(&self) -> &Item<'a, String> {
        &self.xcall_address
    }

    pub fn roll_back(&self) -> &Map<'a, u64, Vec<u8>> {
        &self.rollback
    }

    pub fn connections(&self) -> &Map<'a, String, Vec<Connection>> {
        &self.connections
    }

    pub fn add_connection(
        &self,
        store: &mut dyn Storage,
        network_id: String,
        conn: Connection,
    ) -> Result<(), ContractError> {
        let mut connections = self
            .connections
            .load(store, network_id.clone())
            .unwrap_or(Vec::<Connection>::new());
        connections.push(conn);
        self.connections
            .save(store, network_id, &connections)
            .map_err(ContractError::Std)
    }
    pub fn get_connections(
        &self,
        store: &dyn Storage,
        network_id: String,
    ) -> Result<Vec<Connection>, ContractError> {
        self.connections
            .load(store, network_id.clone())
            .map_err(|_e| ContractError::ConnectionNotFound { network_id })
    }
}
