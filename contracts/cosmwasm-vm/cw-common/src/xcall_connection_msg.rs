use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
};

#[cw_serde]
pub enum ExecuteMsg {
    SetAdmin {
        address: String,
    },
    SetXCallHost {
        address: String,
    },
    MessageFromXCall {
        data: Vec<u8>,
    },
    SetIbcConfig {
        ibc_config: Vec<u8>,
    },
    UpdateAdmin {
        address: String,
    },
    RemoveAdmin {},
    SetTimeoutHeight {
        height: u64,
    },

    #[cfg(not(feature = "native_ibc"))]
    IbcChannelOpen {
        msg: IbcChannelOpenMsg,
    },

    #[cfg(not(feature = "native_ibc"))]
    IbcChannelConnect {
        msg: IbcChannelConnectMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcChannelClose {
        msg: IbcChannelCloseMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketReceive {
        msg: IbcPacketReceiveMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketAck {
        msg: IbcPacketAckMsg,
    },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketTimeout {
        msg: IbcPacketTimeoutMsg,
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
    #[returns(u64)]
    GetTimeoutHeight {},
    #[returns(u128)]
    GetProtocolFee {},
    #[returns(String)]
    GetProtocolFeeHandler {},
}

