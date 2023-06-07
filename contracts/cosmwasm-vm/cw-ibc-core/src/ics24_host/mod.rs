//! ICS 24: Host defines the minimal set of interfaces that a
//! state machine hosting an IBC-enabled chain must implement.

pub mod host;

use crate::context::CwIbcCoreContext;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{MessageInfo, Storage};
use std::time::Duration;

#[cw_serde]
pub struct LastProcessedOn {
    pub height: u64,
    pub timestamp: u64,
}
