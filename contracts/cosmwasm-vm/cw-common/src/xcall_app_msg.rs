use cosmwasm_schema::cw_serde;

use crate::xcall_types::network_address::{NetworkAddress, NetId};

#[cw_serde]
pub enum ExecuteMsg {
    SetAdmin {
        address: String,
    },
    SetProtocolFee {
        value: u128,
    },
    SetProtocolFeeHandler {
        address: String,
    },
    SendCallMessage {
        to: NetworkAddress,
        data: Vec<u8>,
        sources: Option<Vec<String>>,
        destinations: Option<Vec<String>>,
        rollback: Option<Vec<u8>>,
    },
    HandleCallMessage {
        from: NetId,
        sn: Option<i64>,
        msg: Vec<u8>,
    },

    HandleError {
        sn: i64,
        code: i64,
        msg: String,
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
}
