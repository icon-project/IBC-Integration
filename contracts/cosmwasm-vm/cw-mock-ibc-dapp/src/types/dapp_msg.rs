use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_common::cw_types::{
    CwChannelCloseMsg, CwChannelConnectMsg, CwChannelOpenMsg, CwPacketAckMsg, CwPacketReceiveMsg,
    CwPacketTimeoutMsg,
};

#[cw_serde]
pub enum ExecuteMsg {
    SetAdmin {
        address: String,
    },

    SendMessage {
        msg: Vec<u8>,
        timeout_height: u64,
    },

    #[cfg(not(feature = "native_ibc"))]
    IbcChannelOpen {
        msg: CwChannelOpenMsg,
    },

    #[cfg(not(feature = "native_ibc"))]
    IbcChannelConnect {
        msg: CwChannelConnectMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcChannelClose {
        msg: CwChannelCloseMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketReceive {
        msg: CwPacketReceiveMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketAck {
        msg: CwPacketAckMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketTimeout {
        msg: CwPacketTimeoutMsg,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
/// This is a Rust enum representing different types of queries that can be made to the contract. Each
/// variant of the enum corresponds to a specific query and has a return type specified using the
/// `#[returns]` attribute.
pub enum QueryMsg {
    #[returns(String)]
    GetAdmin {},
}
