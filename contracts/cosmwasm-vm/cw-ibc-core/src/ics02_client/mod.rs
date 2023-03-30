//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;

use crate::traits::IbcClient;
use crate::{
    context::CwIbcCoreContext,
    types::{ClientId, ClientType},
    ContractError, IbcClientId, IbcClientType,
};
use cosmwasm_std::{Addr, Deps, DepsMut, Event, Storage};
use ibc::{
    core::ics02_client::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CLIENT_TYPE_ATTRIBUTE_KEY, CONSENSUS_HEIGHTS_ATTRIBUTE_KEY,
        CONSENSUS_HEIGHT_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc::{
    core::ics02_client::msgs::{
        create_client::MsgCreateClient, update_client::MsgUpdateClient,
        upgrade_client::MsgUpgradeClient,
    },
    Height,
};
