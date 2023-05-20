use std::str::FromStr;

use crate::ibc::core::ics24_host::identifier::ClientId;
use constants::{
    ICON_CLIENT_STATE_TYPE_URL, ICON_CONSENSUS_STATE_TYPE_URL, ICON_SIGNED_HEADER_TYPE_URL,
};
use cosmwasm_std::StdError;
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};
use ibc::core::{
    ics02_client::client_type::ClientType,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
    ics26_routing::context::ModuleId,
};
use icon::icon::{
    lightclient::v1::{ClientState, ConsensusState},
    types::v1::SignedHeader,
};
use traits::AnyTypes;
pub mod btp_header;
pub mod client_state;
pub mod consensus_state;
pub mod constants;
pub mod ibc;
pub mod icon;
pub mod rlp;
pub mod signed_header;
pub mod traits;
pub mod types;
pub mod utils;

impl AnyTypes for ClientState {
    fn get_type_url() -> String {
        ICON_CLIENT_STATE_TYPE_URL.to_string()
    }
}

impl AnyTypes for ConsensusState {
    fn get_type_url() -> String {
        ICON_CONSENSUS_STATE_TYPE_URL.to_string()
    }
}

impl AnyTypes for SignedHeader {
    fn get_type_url() -> String {
        ICON_SIGNED_HEADER_TYPE_URL.to_string()
    }
}

impl<'a> PrimaryKey<'a> for ClientId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for ClientId {
    type Output = ClientId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_id = ClientId::from_str(&result).unwrap();
        Ok(client_id)
    }
}

impl<'a> PrimaryKey<'a> for ClientType {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_str().as_bytes())]
    }
}

impl KeyDeserialize for ClientType {
    type Output = ClientType;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_type = ClientType::new(result);
        Ok(client_type)
    }
}
use prost::alloc::borrow::{Borrow};

impl<'a> PrimaryKey<'a> for ModuleId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(
            <ModuleId as Borrow<str>>::borrow(&self).as_bytes(),
        )]
    }
}

impl KeyDeserialize for ModuleId {
    type Output = ModuleId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let module_id = ModuleId::from_str(&result).unwrap();
        Ok(module_id)
    }
}

impl<'a> PrimaryKey<'a> for PortId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for PortId {
    type Output = PortId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let port_id = PortId::from_str(&result).unwrap();
        Ok(port_id)
    }
}

impl<'a> Prefixer<'a> for PortId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl<'a> PrimaryKey<'a> for ConnectionId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = ();

    type SuperSuffix = ();
    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_str().as_bytes())]
    }
}
impl<'a> Prefixer<'a> for ConnectionId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for ConnectionId {
    type Output = ConnectionId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let connection_id = ConnectionId::from_str(&result).unwrap();
        Ok(connection_id)
    }
}

impl<'a> PrimaryKey<'a> for ChannelId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for ChannelId {
    type Output = ChannelId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let chan_id = ChannelId::from_str(&result).unwrap();
        Ok(chan_id)
    }
}

impl From<ClientId> for ClientType {
    fn from(value: ClientId) -> Self {
        let data: Vec<&str> = value.as_str().split('-').collect();
        ClientType::new(data[0].to_string())
    }
}
