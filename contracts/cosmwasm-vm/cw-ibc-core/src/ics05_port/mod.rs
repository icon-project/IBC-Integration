//! ICS 05: Port implementation specifies the allocation scheme used by modules to
//! bind to uniquely named ports.
pub mod port;
use cosmwasm_std::Storage;
use ibc::core::{ics05_port::error::PortError, ics26_routing::context::ModuleId};

use crate::{context::CwIbcCoreContext, ContractError};
use cw_common::types::PortId;
use super::*;