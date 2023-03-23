use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
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
        rollback: Vec<u8>,
    },

    ExecuteCall {
        request_id: u128,
    },

    ExecuteRollback {
        sequence_no: u128,
    },

    #[cfg(feature = "nonibc")]
    IbcChannelOpen {
        msg: IbcChannelOpenMsg,
    },

    #[cfg(feature = "nonibc")]
    IbcChannelConnect {
        msg: IbcChannelConnectMsg,
    },
    #[cfg(feature = "nonibc")]
    IbcChannelClose {
        msg: IbcChannelCloseMsg,
    },
    #[cfg(feature = "nonibc")]
    IbcPacketReceive {
        msg: IbcPacketReceiveMsg,
    },
    #[cfg(feature = "nonibc")]
    IbcPacketAck {
        msg: IbcPacketAckMsg,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Address)]
    GetAdmin {},
    #[returns(u128)]
    GetProtocolFee {},
    #[returns(Address)]
    GetProtocolFeeHandler {},
}
