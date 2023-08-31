use crate::types::{VerifyChannelState, VerifyPacketAcknowledgement, VerifyPacketData};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub ibc_host: Addr,
}

impl Default for InstantiateMsg {
    fn default() -> Self {
        Self {
            ibc_host: Addr::unchecked("ibc_host"),
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
    GetLatestConsensusState { client_id: String },
    #[returns(Vec<u8>)]
    GetClientState { client_id: String },
    #[returns(bool)]
    VerifyMembership {
        client_id: String,
        message_bytes: Vec<u8>,
        path: Vec<u8>,
        proofs: Vec<u8>,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
    },
    #[returns(bool)]
    VerifyNonMembership {
        client_id: String,
        path: Vec<u8>,
        proofs: Vec<u8>,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
    },
    #[returns(bool)]
    VerifyChannel {
        verify_channel_state: VerifyChannelState,
    },
    #[returns(bool)]
    VerifyOpenConfirm {
        client_id: String,
        verify_connection_state: VerifyConnectionState,
    },
    #[returns(bool)]
    TimeoutOnCLose {
        client_id: String,
        verify_channel_state: VerifyChannelState,
        next_seq_recv_verification_result: LightClientPacketMessage,
    },
    #[returns(bool)]
    PacketTimeout {
        client_id: String,
        next_seq_recv_verification_result: LightClientPacketMessage,
    },
    #[returns(bool)]
    VerifyPacketData {
        client_id: String,
        verify_packet_data: VerifyPacketData,
    },
    #[returns(bool)]
    VerifyPacketAcknowledgement {
        client_id: String,
        verify_packet_acknowledge: VerifyPacketAcknowledgement,
    },
    #[returns(bool)]
    VerifyConnectionOpenTry(VerifyConnectionPayload),
    #[returns(bool)]
    VerifyConnectionOpenAck(VerifyConnectionPayload),
    #[returns(Vec<u64>)]
    GetPreviousConsensusState { client_id: String, height: u64 },
    #[returns(u64)]
    GetTimestampAtHeight { client_id: String, height: u64 },
}

#[cw_serde]
pub struct Response {
    value: u64,
}

#[cw_serde]
pub enum LightClientPacketMessage {
    VerifyPacketReceiptAbsence {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        receipt_path: Vec<u8>,
        // packet_data: Vec<u8>,
    },

    VerifyNextSequenceRecv {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        seq_recv_path: Vec<u8>,
        sequence: u64,
        //  packet_data: Vec<u8>,
    },
}

#[cw_serde]
pub struct VerifyConnectionState {
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub counterparty_conn_end_path: Vec<u8>,
    pub expected_counterparty_connection_end: Vec<u8>,
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
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub client_state_proof: Vec<u8>,
    pub root: Vec<u8>,
    pub client_state_path: Vec<u8>,
    pub expected_client_state: Vec<u8>,
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
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub consensus_state_proof: Vec<u8>,
    pub root: Vec<u8>,
    pub conesenus_state_path: Vec<u8>,
    pub expected_conesenus_state: Vec<u8>,
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

    UpgradeClient {
        upgraded_client_state: Vec<u8>,
        upgraded_consensus_state: Vec<u8>,
        proof_upgrade_client: Vec<u8>,
        proof_upgrade_consensus_state: Vec<u8>,
    },

    Misbehaviour {
        client_id: String,
        misbehaviour: Vec<u8>,
    },
}

#[cw_serde]
pub struct VerifyConnectionPayload {
    pub client_id: String,
    pub verify_connection_state: VerifyConnectionState,
    pub verify_client_full_state: VerifyClientFullState,
    pub verify_client_consensus_state: VerifyClientConsensusState,
}
