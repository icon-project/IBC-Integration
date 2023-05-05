//! ICS 24: Host defines the minimal set of interfaces that a
//! state machine hosting an IBC-enabled chain must implement.

pub mod host;

use crate::context::CwIbcCoreContext;
use cosmwasm_std::{MessageInfo, Storage};
use crate::ContractError;
use std::time::Duration;
