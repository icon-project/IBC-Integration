use cosmwasm_schema::cw_serde;

use crate::xcall_types::network_address::NetworkAddress;

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
