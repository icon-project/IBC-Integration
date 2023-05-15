use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Binary, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
};

#[cw_serde]
pub enum ExecuteMsg {
    SetAdmin {
        address: String,
    },
    SetProtocol {
        value: u128,
    },
    SetProtocolFeeHandler {
        address: String,
    },
    SendCallMessage {
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    },
    ReceiveCallMessage {
        data: Vec<u8>,
    },

    ExecuteCall {
        request_id: u128,
    },

    ExecuteRollback {
        sequence_no: u128,
    },
    UpdateAdmin {
        address: String,
    },
    RemoveAdmin {},
    SetTimeoutHeight {
        height: u64,
    },
}
