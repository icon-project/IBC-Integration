use super::*;

#[cw_serde]
pub enum LightClientConnectionMessage {
    OpenTry {
        verify_connection_state: VerifyConnectionState,
        verify_client_full_satate: VerifyClientFullState,
        vefiry_client_consensus_state: VerifyClientConsesnusState,
    },
}

#[cw_serde]
pub struct VerifyConnectionState {}

#[cw_serde]
pub struct VerifyClientFullState {}

#[cw_serde]
pub struct VerifyClientConsesnusState {}

// proof_height: String,
//         counterparty_prefix: Vec<u8>,
//         client_state_proof: Vec<u8>,
//         consensus_state_proof: Vec<u8>,
//         root: Vec<u8>,
//         counterparty_chan_end_path: Vec<u8>,
//         expected_counterparty_connection_end: Vec<u8>,
//         client_state_path: Vec<u8>,
//         expected_client_state: Vec<u8>,
//         conesenus_state_path: Vec<u8>,
//         expected_conesenus_state: Vec<u8>,
