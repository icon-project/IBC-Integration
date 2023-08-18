use super::*;
use cosmwasm_schema::QueryResponses;
// use cw_common::cw_types::CwOrder;

/// This is a Rust struct representing a message to instantiate a contract with timeout height and IBC
/// host address.
///
/// Properties:
///
/// * `ibc_host`: `ibc_host` is a field of type `Addr` in the `InstantiateMsg` struct. It likely
/// represents the address of the IBC host that the message is being sent to. However, without more
/// context it's difficult to say for sure.
#[cw_serde]
pub struct InstantiateMsg {
    pub ibc_host: Addr,
}

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
    #[cfg(not(feature = "native_ibc"))]
    IbcWriteAcknowledgement {
        seq: u64,
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
