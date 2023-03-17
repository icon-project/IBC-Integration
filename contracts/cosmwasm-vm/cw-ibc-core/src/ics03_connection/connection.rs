use crate::types::{ClientId, ConnectionId};
use crate::{state::CwIbcStore, ContractError};
use cosmwasm_std::{Deps, DepsMut, Response, Storage};
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc_proto::protobuf::Protobuf;

impl<'a> CwIbcStore<'a> {
    pub fn set_connection(
        &self,
        deps: DepsMut,
        conn_id: ConnectionId,
        conn_end: ConnectionEnd,
    ) -> Result<Response, ContractError> {
        self.add_connection(deps.storage, conn_id,conn_end)?;
        Ok(Response::new().add_attribute("method", "set_connection"))
    }

    pub fn get_connection(&self, deps: Deps, conn_id: ConnectionId) -> ConnectionEnd {
        self.query_connection(deps.storage, conn_id).unwrap()
    }

    pub fn get_next_connection_sequence(&self, store: &mut dyn Storage, sequence: u128) -> u128 {
        self.connection_next_sequence_init(store, sequence).unwrap();
        self.query_next_sequence(store).unwrap()
    }

    pub fn increment_connection_sequence(&self, store: &mut dyn Storage) -> u128 {
        self.increase_connection_sequence(store).unwrap()
    }

    pub fn store_connection_to_client(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        conn_id: ConnectionId,
    ) -> Result<Response, ContractError> {
        self.client_connection(store, client_id, conn_id)?;
        Ok(Response::new().add_attribute("method", "store_connection_to_client"))
    }

    pub fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::try_from(b"Ibc".to_vec()).unwrap_or_default()
    }
}

impl<'a> CwIbcStore<'a> {
    pub fn add_connection(
        &self,
        store: &mut dyn Storage,
        conn_id: ConnectionId,
        conn_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        let data = conn_end.encode_vec().unwrap();
        match self.connections().save(store, conn_id, &data) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn query_connection(
        &self,
        store: &dyn Storage,
        conn_id: ConnectionId,
    ) -> Result<ConnectionEnd, ContractError> {
        match self.connections().load(store, conn_id) {
            Ok(conn_end) => {
                let data: &[u8] = &conn_end;
                let data: ConnectionEnd = ConnectionEnd::decode(data).unwrap();
                Ok(data)
            }
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn connection_next_sequence_init(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
    ) -> Result<(), ContractError> {
        match self.next_connection_sequence().save(store, &sequence) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn query_next_sequence(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        match self.next_connection_sequence().load(store) {
            Ok(u128) => Ok(u128),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increase_connection_sequence(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence_no = self.next_connection_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        )?;

        Ok(sequence_no)
    }

    pub fn client_connection(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        conn_id: ConnectionId,
    ) -> Result<(), ContractError> {
        match self.client_connections().save(store, client_id, &conn_id) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
