use super::*;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub src_network_id: String,
    pub network_id: u64,
    pub network_type_id: u128,
}

impl Default for InstantiateMsg {
    fn default() -> Self {
        Self {
            src_network_id: "0x3.icon".to_string(),
            network_id: 1,
            network_type_id: 1,
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    GetLatestHeight { client_id: String },
    #[returns(Vec<u8>)]
    GetConsensusState { client_id: String, height: u64 },
    #[returns(Vec<u8>)]
    GetClientState { client_id: String },
}

#[cw_serde]
pub enum LightClientPacketMessage {
    VerifyPacketReceiptAbsence {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        receipt_path: Vec<u8>,
        packet_data: Vec<u8>,
    },

    VerifyNextSequenceRecv {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        seq_recv_path: Vec<u8>,
        sequence: u64,
        packet_data: Vec<u8>,
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
pub struct VerifyClientConsensusState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    consensus_state_proof: Vec<u8>,
    root: Vec<u8>,
    conesenus_state_path: Vec<u8>,
    expected_conesenus_state: Vec<u8>,
}

impl VerifyClientConsensusState {
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
pub enum ExecuteMsg {
    CreateClient {
        client_id: String,
        client_state: Vec<u8>,
        consensus_state: Vec<u8>,
    },
    UpdateClient {
        client_id: String,
        signed_header: Vec<u8>,
    },
    VerifyMembership {
        client_id: String,
        message_bytes: Vec<u8>,
        path: Vec<u8>,
        proofs: Vec<u8>,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
    },
    VerifyNonMembership {
        client_id: String,
        path: Vec<u8>,
        proofs: Vec<u8>,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
    },
    UpgradeClient {
        upgraded_client_state: Vec<u8>,
        upgraded_consensus_state: Vec<u8>,
        proof_upgrade_client: Vec<u8>,
        proof_upgrade_consensus_state: Vec<u8>,
    },
    VerifyChannel {
        verify_channel_state: VerifyChannelState,
    },
    Misbehaviour {
        client_id: String,
        misbehaviour: Vec<u8>,
    },
    VerifyConnection {
        verify_connection_state: VerifyConnectionState,
        verify_client_full_state: VerifyClientFullState,
        verify_client_consensus_state: VerifyClientConsensusState,
    },
    VerifyOpenConfirm {
        verify_connection_state: VerifyConnectionState,
    },
    TimeoutOnCLose {
        verify_channel_state: VerifyChannelState,
        next_seq_recv_verification_result: LightClientPacketMessage,
    },
    VerifyPacketData {
        verify_packet_data: VerifyPacketData,
        packet_data: Vec<u8>,
    },
    VerifyPacketAcknowledgement {
        verify_packet_acknowledge: VerifyPacketAcknowledgement,
        packet_data: Vec<u8>,
    },
}
