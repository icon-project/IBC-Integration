use cosmwasm_std::Addr;
use cw_xcall_lib::network_address::NetworkAddress;

use super::*;

#[cw_serde]

pub struct CallRequest {
    from: Addr,
    to: NetworkAddress,
    protocols: Vec<String>,
    rollback: Vec<u8>,
    enabled: bool,
}

impl CallRequest {
    pub fn new(
        from: Addr,
        to: NetworkAddress,
        protocols: Vec<String>,
        rollback: Vec<u8>,
        enabled: bool,
    ) -> Self {
        Self {
            from,
            to,
            rollback,
            protocols,
            enabled,
        }
    }

    pub fn from(&self) -> &Addr {
        &self.from
    }

    pub fn to(&self) -> &NetworkAddress {
        &self.to
    }

    pub fn rollback(&self) -> &[u8] {
        &self.rollback
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn protocols(&self) -> &Vec<String> {
        &self.protocols
    }

    pub fn is_null(&self) -> bool {
        let r = to_binary(self).unwrap();
        r.is_empty()
    }
    pub fn set_enabled(&mut self) {
        self.enabled = true;
    }
}
