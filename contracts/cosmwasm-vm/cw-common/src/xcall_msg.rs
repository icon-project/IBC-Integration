use super::*;

#[cw_serde]
#[non_exhaustive]
pub enum ExecuteMsg {
    SetAdmin {
        address: Address,
    },
    SetProtocol {
        value: u128,
    },
    SetProtocolFeeHandler {
        address: Address,
    },
    SendCallMessage {
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    },

    ExecuteCall {
        request_id: u128,
    },

    ExecuteRollback {
        sequence_no: u128,
    },
    UpdateAdmin {
        address: Address,
    },
    RemoveAdmin {},

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
