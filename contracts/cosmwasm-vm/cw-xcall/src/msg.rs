use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::request::CallServiceMessageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum IbcExecuteMsg {
    Event {
        sequence_no: i128,
        rollback: Vec<u8>,
        message: CallServiceMessageRequest,
    },
}
