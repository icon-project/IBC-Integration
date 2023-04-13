use std::str::FromStr;

use cosmwasm_std::StdError;
use cw_storage_plus::{Key, KeyDeserialize, PrimaryKey};
use ibc::core::ics02_client::client_type::ClientType as IbcClientType;
use ibc::core::ics24_host::identifier::ClientId as IbcClientId;
use serde::{Deserialize, Serialize};

use crate::errors::CwErrors;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(IbcClientId);

impl Default for ClientId {
    fn default() -> Self {
        Self(IbcClientId::default())
    }
}
impl From<IbcClientId> for ClientId {
    fn from(value: IbcClientId) -> Self {
        Self(value)
    }
}
impl FromStr for ClientId {
    type Err = ibc::core::ics24_host::error::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = IbcClientId::from_str(s)?;
        Ok(Self(r))
    }
}
impl ClientId {
    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn new(client_type: ClientType, counter: u64) -> Result<Self, CwErrors> {
        match IbcClientId::new(client_type.client_type(), counter) {
            Ok(result) => Ok(Self(result)),
            Err(err) => Err(CwErrors::FailedToCreateClientId {
                client_type: client_type,
                counter,
                validation_error: err,
            }),
        }
    }

    pub fn ibc_client_id(&self) -> &IbcClientId {
        &self.0
    }

    pub fn from(client_id: IbcClientId) -> Self {
        Self(client_id)
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
    pub fn new(cleint_type: String) -> ClientType {
        ClientType(IbcClientType::new(cleint_type))
    }
    pub fn client_type(&self) -> IbcClientType {
        self.0.clone()
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<IbcClientType> for ClientType {
    fn from(value: IbcClientType) -> Self {
        Self(value)
    }
}

impl From<ClientId> for ClientType {
    fn from(value: ClientId) -> Self {
        let data: Vec<&str> = value.as_str().split("-").collect();
        ClientType::new(data[0].to_string())
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
