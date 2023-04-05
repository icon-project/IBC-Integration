use cosmwasm_schema::cw_serde;

use super::*;

#[cw_serde]
pub enum LightClientConnectionMessage {
    OpenAck {
        verify_connection_state: VerifyConnectionState,
        verify_client_full_satate: VerifyClientFullState,
        vefiry_client_consensus_state: VerifyClientConsesnusState,
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
pub struct VerifyClientFullState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    client_state_proof: Vec<u8>,
    root: Vec<u8>,
    client_state_path: Vec<u8>,
    expected_client_state: Vec<u8>,
}
impl VerifyClientFullState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        client_state_proof: Vec<u8>,
        root: Vec<u8>,
        client_state_path: Vec<u8>,
        expected_client_state: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            client_state_proof,
            root,
            client_state_path,
            expected_client_state,
        }
    }
}

#[cw_serde]
pub struct VerifyClientConsesnusState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    consensus_state_proof: Vec<u8>,
    root: Vec<u8>,
    conesenus_state_path: Vec<u8>,
    expected_conesenus_state: Vec<u8>,
}

impl VerifyClientConsesnusState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        consensus_state_proof: Vec<u8>,
        root: Vec<u8>,
        conesenus_state_path: Vec<u8>,
        expected_conesenus_state: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            consensus_state_proof,
            root,
            conesenus_state_path,
            expected_conesenus_state,
        }
    }
}

#[cw_serde]
pub struct OpenAckResponse {
    pub conn_id: String,
    pub version: Vec<u8>,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
}