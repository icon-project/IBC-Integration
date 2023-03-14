use std::str::FromStr;

use cosmwasm_std::StdError;
use cw_storage_plus::{Key, KeyDeserialize, PrimaryKey};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(IbcClientId);

impl<'a> PrimaryKey<'a> for ClientId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for ClientId {
    type Output = ClientId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_id = IbcClientId::from_str(&result).unwrap();
        Ok(ClientId(client_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientType(IbcClientType);

impl<'a> PrimaryKey<'a> for ClientType {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_str().as_bytes())]
    }
}

impl KeyDeserialize for ClientType {
    type Output = ClientType;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_type = IbcClientType::new(result);
        Ok(ClientType(client_type))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelId(IbcChannelId);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionId(pub IbcConnectionId);

impl<'a> PrimaryKey<'a> for ConnectionId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;
    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_str().as_bytes())]
    }
}
impl KeyDeserialize for ConnectionId {
    type Output = ConnectionId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let connection_id = IbcConnectionId::from_str(&result).unwrap();
        Ok(ConnectionId(connection_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortId(IbcPortId);
