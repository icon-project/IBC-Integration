use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdError};

#[cw_serde]
#[derive(Eq)]
pub struct NetId(String);

impl From<String> for NetId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl ToString for NetId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl NetId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for NetId {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

#[cw_serde]
#[derive(Eq)]
pub struct NetworkAddress(String);

impl NetworkAddress {
    pub fn new(nid: &str, address: &str) -> Self {
        Self(format!("{}/{}", nid, address))
    }
    pub fn get_nid(&self) -> NetId {
        NetId(self.get_parts()[0].to_string())
    }

    pub fn get_account(&self) -> Addr {
        Addr::unchecked(self.get_parts()[1])
    }

    pub fn get_parts(&self) -> Vec<&str> {
        let parts = self.0.split("/").collect::<Vec<&str>>();
        return parts;
    }
}

impl ToString for NetworkAddress {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for NetworkAddress {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(StdError::GenericErr {
                msg: "Invalid Network Address".to_owned(),
            });
        }
        let na = format!("{}/{}", parts[0], parts[1]);
        Ok(Self(na))
    }
}
