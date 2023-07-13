use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_xcall_lib::network_address::NetworkAddress;

#[cw_serde]
pub enum ExecuteMsg {
    SendCallMessage {
        to: NetworkAddress,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    },
    HandleCallMessage {
        from: NetworkAddress,
        data: Vec<u8>,
        protocols: Vec<String>,
    },
    XCallMessage {
        data: Vec<u8>,
    },
    SuccessCall {},
    FailureCall {},
    TestCall {
        success_addr: String,
        fail_addr: String,
    },
    AddConnection {
        src_endpoint: String,
        dest_endpoint: String,
        network_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
/// This is a Rust enum representing different types of queries that can be made to the contract. Each
/// variant of the enum corresponds to a specific query and has a return type specified using the
/// `#[returns]` attribute.
pub enum QueryMsg {
    #[returns(u64)]
    GetSequence {},
}
