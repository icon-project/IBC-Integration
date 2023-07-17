use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::network_address::NetId;

#[cw_serde]
pub enum ExecuteMsg {
    SendMessage { to: NetId, sn: i64, msg: Vec<u8> },
}

#[cw_serde]
#[derive(QueryResponses)]
/// This is a Rust enum representing different types of queries that can be made to the contract. Each
/// variant of the enum corresponds to a specific query and has a return type specified using the
/// `#[returns]` attribute.
pub enum QueryMsg {
    #[returns(u64)]
    GetFee { nid: NetId, response: bool },
}
