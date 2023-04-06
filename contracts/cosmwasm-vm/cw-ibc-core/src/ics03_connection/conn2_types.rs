use cosmwasm_schema::cw_serde;

use super::*;

#[cw_serde]
pub enum LightClientConnectionMessage {
    OpenAck {
        verify_connection_state: VerifyConnectionState,
    },
}

#[cw_serde]
pub struct VerifyConnectionState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    proof: Vec<u8>,
    root: Vec<u8>,
    counterparty_conn_end_path: Vec<u8>,
    expected_counterparty_connection_end: Vec<u8>,
}
impl VerifyConnectionState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        counterparty_conn_end_path: Vec<u8>,
        expected_counterparty_connection_end: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            proof,
            root,
            counterparty_conn_end_path,
            expected_counterparty_connection_end,
        }
    }
}

#[cw_serde]
pub struct OpenConfirmResponse {
    pub conn_id: String,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
}
