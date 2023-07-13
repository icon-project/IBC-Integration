use cw_xcall_lib::network_address::NetId;

use super::*;
#[cw_serde]
pub struct InstantiateMsg {
    pub network_id: String,
    pub denom: String,
}

/// The `#[cw_serde]` attribute is used to automatically generate serialization and deserialization code
/// for the struct or enum it is applied to.
#[cw_serde]
#[derive(QueryResponses)]
/// This is a Rust enum representing different types of queries that can be made to the contract. Each
/// variant of the enum corresponds to a specific query and has a return type specified using the
/// `#[returns]` attribute.
pub enum QueryMsg {
    #[returns(String)]
    GetAdmin {},
    #[returns(u128)]
    GetProtocolFee {},
    #[returns(String)]
    GetProtocolFeeHandler {},
    #[returns(String)]
    GetNetworkAddress {},
    #[returns(bool)]
    VerifySuccess { sn: u128 },
    #[returns(String)]
    GetDefaultConnection { nid: NetId },
}
