use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    SendCallMessage {
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    },
    HandleCallMessage {
        from: String,
        data: Vec<u8>,
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
}
