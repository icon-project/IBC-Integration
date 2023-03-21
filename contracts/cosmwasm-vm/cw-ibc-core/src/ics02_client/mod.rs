//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;
use cosmwasm_std::Storage;

use crate::{
    context::CwIbcCoreContext,
    types::{ClientId, ClientType},
    ContractError, IbcClientId, IbcClientType,
};
use cosmwasm_std::{Deps, DepsMut};

use crate::traits::IbcClient;

use cosmwasm_std::Event;
use ibc::{
    core::ics02_client::msgs::{
        create_client::MsgCreateClient, update_client::MsgUpdateClient,
        upgrade_client::MsgUpgradeClient,
    },
    Height,
};

use ibc::{
    core::ics02_client::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CLIENT_TYPE_ATTRIBUTE_KEY, CONSENSUS_HEIGHTS_ATTRIBUTE_KEY,
        CONSENSUS_HEIGHT_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
