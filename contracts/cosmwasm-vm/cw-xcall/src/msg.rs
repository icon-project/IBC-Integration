use super::*;

/// This is a Rust struct representing a message to instantiate a contract with timeout height and IBC
/// host address.
///
/// Properties:
///
/// * `timeout_height`: `timeout_height` is a field of type `u64` (unsigned 64-bit integer) in the
/// `InstantiateMsg` struct. It represents the block height at which the transaction will timeout if it
/// has not been included in a block by that height. This is used to prevent transactions from being
/// * `ibc_host`: `ibc_host` is a field of type `Addr` in the `InstantiateMsg` struct. It likely
/// represents the address of the IBC host that the message is being sent to. However, without more
/// context it's difficult to say for sure.
#[cw_serde]
pub struct InstantiateMsg {
    pub timeout_height: u64,
    pub ibc_host: Addr,
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
    #[returns(u64)]
    GetTimeoutHeight {},
}
