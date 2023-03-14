use cosmwasm_std::{Response, Storage};
use ibc::core::{
    ics03_connection::connection::{self, ConnectionEnd},
    ics24_host::identifier::ConnectionId,
};

use crate::{state::CwIbcStore, types, ContractError};

impl<'a> CwIbcStore<'a> {
    pub fn set_connection(
        &self,
        store: &mut dyn Storage,
        conn_end: ConnectionEnd,
        conn_id: ConnectionId,
    ) -> Result<Response, ContractError> {
        self.add_connection(store, conn_end, conn_id)?;
        Ok(Response::new().add_attribute("method", "set_connection"))
    }

    pub fn get_connection(&self, store: &mut dyn Storage, conn_id: ConnectionId) -> ConnectionEnd {
        self.query_connection(store, conn_id).unwrap()
    }
}

impl<'a> CwIbcStore<'a> {
    pub fn add_connection(
        &self,
        store: &mut dyn Storage,
        conn_end: ConnectionEnd,
        conn_id: ConnectionId,
    ) -> Result<(), ContractError> {
        match self
            .connections()
            .save(store, types::ConnectionId(conn_id), &conn_end)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn query_connection(
        &self,
        store: &mut dyn Storage,
        conn_id: ConnectionId,
    ) -> Result<ConnectionEnd, ContractError> {
        match self.connections().load(store, types::ConnectionId(conn_id)) {
            Ok(conn_end) => Ok(conn_end),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
