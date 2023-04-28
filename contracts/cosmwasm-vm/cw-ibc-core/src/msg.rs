use ibc::{
    core::{
        ics03_connection::{msgs::conn_open_try::MsgConnectionOpenTry, version::Version},
        ics04_channel::{channel::Order, msgs::acknowledgement::Acknowledgement, packet::Packet},
    },
    signer::Signer,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};

use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
