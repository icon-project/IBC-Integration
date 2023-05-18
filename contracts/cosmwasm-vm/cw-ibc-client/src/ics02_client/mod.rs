//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;

use crate::context::CwIbcClientContext;
use crate::{traits::IbcClient, ContractError, IbcClientId, IbcClientType};
use common::icon::icon::lightclient::v1::{
    ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Addr, CosmosMsg, DepsMut, Event, MessageInfo, Reply, Response,
    Storage, SubMsg,
};
use cw_common::client_response::{
    CreateClientResponse, MisbehaviourResponse, UpdateClientResponse, UpgradeClientResponse,
};
use cw_common::commitment;
use cw_common::ibc_types::*;
use cw_common::raw_types::{Any, Protobuf};
use cw_common::types::{ClientId, ClientType};
use events::{create_client_event, update_client_event, upgrade_client_event};
use serde::{Deserialize, Serialize};
use std::time::Duration;
