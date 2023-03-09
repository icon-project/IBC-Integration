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

#[cw_serde]
pub enum IbcExecuteMsg {
    Event {
        sequence_no: i128,
        rollback: Vec<u8>,
        message: CallServiceMessageRequest,
    },
}
