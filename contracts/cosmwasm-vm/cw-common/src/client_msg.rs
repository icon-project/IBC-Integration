use crate::cw_types::CwEndpoint;
use crate::{
    client_response::{OpenAckResponse, OpenConfirmResponse, OpenTryResponse, PacketDataResponse},
    errors::CwErrors,
    types::{
        MessageInfo, PacketData, VerifyChannelState, VerifyPacketAcknowledgement, VerifyPacketData,
    },
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::from_slice;

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
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub counterparty_conn_end_path: Vec<u8>,
    pub expected_counterparty_connection_end: Vec<u8>,
    pub expected_counterparty_connection_end_hash: Vec<u8>,
}
impl VerifyConnectionState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        counterparty_conn_end_path: Vec<u8>,
        expected_counterparty_connection_end: Vec<u8>,
        expected_counterparty_connection_end_hash: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            proof,
            root,
            counterparty_conn_end_path,
            expected_counterparty_connection_end,
            expected_counterparty_connection_end_hash
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
    pub expected_client_state_hash: Vec<u8>,
}
impl VerifyClientFullState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        client_state_proof: Vec<u8>,
        root: Vec<u8>,
        client_state_path: Vec<u8>,
        expected_client_state: Vec<u8>,
        expected_client_state_hash:Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            client_state_proof,
            root,
            client_state_path,
            expected_client_state,
            expected_client_state_hash
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
        message_info: MessageInfo,
        endpoint: CwEndpoint,
        verify_channel_state: VerifyChannelState,
        // add all props that we need on response
    },
    Misbehaviour {
        client_id: String,
        misbehaviour: Vec<u8>,
    },
    VerifyOpenConfirm {
        expected_response: OpenConfirmResponse,
        client_id: String,
        verify_connection_state: VerifyConnectionState,
        // add all props that we need on response
    },
    TimeoutOnCLose {
        client_id: String,
        verify_channel_state: VerifyChannelState,
        next_seq_recv_verification_result: LightClientPacketMessage,
    },
    VerifyPacketData {
        client_id: String,
        verify_packet_data: VerifyPacketData,
        packet_data: Vec<u8>,
    },
    VerifyPacketAcknowledgement {
        client_id: String,
        verify_packet_acknowledge: VerifyPacketAcknowledgement,
        packet_data: Vec<u8>,
    },
    VerifyConnectionOpenTry(VerifyConnectionPayload<OpenTryResponse>),
    VerifyConnectionOpenAck(VerifyConnectionPayload<OpenAckResponse>),
}

#[cw_serde]
pub struct VerifyConnectionPayload<T> {
    pub client_id: String,
    pub verify_connection_state: VerifyConnectionState,
    pub verify_client_full_state: VerifyClientFullState,
    pub verify_client_consensus_state: VerifyClientConsensusState,
    pub expected_response: T,
}

impl TryFrom<LightClientPacketMessage> for PacketDataResponse {
    type Error = CwErrors;

    fn try_from(value: LightClientPacketMessage) -> Result<Self, Self::Error> {
        let res = match value {
            LightClientPacketMessage::VerifyNextSequenceRecv {
                height: _,
                prefix: _,
                proof: _,
                root: _,
                seq_recv_path: _,
                sequence: _,
                packet_data,
            } => {
                let packet_data: PacketData = from_slice(&packet_data)
                    .map_err(|e| CwErrors::FailedToConvertToPacketDataResponse(e))?;
                PacketDataResponse::from(packet_data)
            }
            LightClientPacketMessage::VerifyPacketReceiptAbsence {
                height: _,
                prefix: _,
                proof: _,
                root: _,
                receipt_path: _,
                packet_data,
            } => {
                let packet_data: PacketData = from_slice(&packet_data)
                    .map_err(|e| CwErrors::FailedToConvertToPacketDataResponse(e))?;
                PacketDataResponse::from(packet_data)
            }
        };
        Ok(res)
    }
}
