//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;

use crate::{
    context::CwIbcCoreContext, traits::IbcClient, ContractError, IbcClientId, IbcClientType,
};
use common::ibc::core::ics24_host::identifier::ClientId;

use cosmwasm_std::{
    from_binary, to_binary, to_vec, Addr, CosmosMsg, DepsMut, Event, MessageInfo, Reply, Response,
    Storage, SubMsg,
};
use cw_common::client_response::{
    CreateClientResponse, MisbehaviourResponse, UpdateClientResponse, UpgradeClientResponse,
};
use cw_common::commitment;
use cw_common::ibc_types::*;
use cw_common::raw_types::Any;

use events::{create_client_event, update_client_event, upgrade_client_event};

use std::time::Duration;
