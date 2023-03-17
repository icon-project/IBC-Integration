use cosmwasm_std::StdError;
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};

use ibc::core::ics24_host::error::ValidationError;
use std::{
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(IbcClientId);

impl ClientId {
    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn new(client_type: ClientType, counter: u64) -> Result<Self, ValidationError> {
        match IbcClientId::new(client_type.client_type(), counter) {
            Ok(result) => Ok(Self(result)),
            Err(error) => Err(error),
        }
    }
    pub fn default() -> Self {
        Self(IbcClientId::default())
    }
}

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

impl ClientType {
    pub fn client_type(&self) -> IbcClientType {
        self.0.clone()
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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
pub struct ConnectionId(IbcConnectionId);

impl ConnectionId {
    pub fn new(identifier: u64) -> Self {
        Self(IbcConnectionId::new(identifier))
    }

    /// Returns the static prefix to be used across all connection identifiers.
    pub fn prefix() -> &'static str {
        IbcConnectionId::prefix()
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    pub fn connection_id(&self) -> &IbcConnectionId {
        &self.0
    }
}

impl<'a> PrimaryKey<'a> for ConnectionId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = ();

    type SuperSuffix = ();
    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_str().as_bytes())]
    }
}
impl<'a> Prefixer<'a> for ConnectionId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
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
pub struct ChannelId(IbcChannelId);

impl<'a> PrimaryKey<'a> for ChannelId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for ChannelId {
    type Output = ChannelId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let port_id = IbcChannelId::from_str(&result).unwrap();
        Ok(ChannelId(port_id))
    }
}

impl ChannelId {
    /// function for create new channel id
    pub fn new(identifier: u64) -> Self {
        Self(IbcChannelId::new(identifier))
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn default() -> Self {
        Self(IbcChannelId::default())
    }

    pub fn channel_id(&self) -> &IbcChannelId {
        &self.0
    }
}

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortId(IbcPortId);

impl<'a> PrimaryKey<'a> for PortId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl PortId {
    /// Infallible creation of the well-known transfer port
    pub fn transfer() -> Self {
        Self(IbcPortId::transfer())
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn dafault() -> Self {
        Self(IbcPortId::default())
    }
    pub fn port_id(&self) -> &IbcPortId {
        &self.0
    }
}

impl Display for PortId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl<'a> Prefixer<'a> for PortId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for PortId {
    type Output = PortId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let port_id = IbcPortId::from_str(&result).unwrap();
        Ok(PortId(port_id))
    }
}
