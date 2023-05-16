use std::time::Duration;

use crate::constants::ICON_CLIENT_TYPE;
use crate::{constants::ICON_CLIENT_STATE_TYPE_URL, icon::icon::lightclient::v1::ClientState};
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ContextError;
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// pub struct ClientState {
//     trusting_period: u64,
//     frozen_height: Option<u64>,
//     max_clock_drift: u64,
//     latest_height: u64,
//     network_section_hash: Vec<u8>,
//     validators: Vec<Vec<u8>>,
// }

// impl TryFrom<&[u8]> for ClientState {
//     type Error = ClientError;

//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         match serde_json_wasm::from_slice(value) {
//             Ok(result) => Ok(result),
//             Err(error) => Err(ClientError::Other {
//                 description: error.to_string(),
//             }),
//         }
//     }
// }

impl ClientState {
    pub fn new(
        trusting_period: u64,
        frozen_height: u64,
        max_clock_drift: u64,
        latest_height: u64,
        network_section_hash: Vec<u8>,
        validators: Vec<Vec<u8>>,
    ) -> Result<Self, ClientError> {
        if max_clock_drift == 0 {
            return Err(ClientError::Other {
                description: "ClientState max-clock-drift must be greater than zero".to_string(),
            });
        }

        Ok(Self {
            trusting_period,
            frozen_height,
            max_clock_drift,
            latest_height,
            network_section_hash,
            validators,
        })
    }
}

// impl Protobuf<RawClientState> for ClientState {}
// impl TryFrom<RawClientState> for ClientState {
//     type Error = ClientError;

//     fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
//         let client_state = Self::new(
//             raw.trusting_period,
//             raw.frozen_height,
//             raw.max_clock_drift,
//             raw.latest_height,
//             raw.network_section_hash,
//             raw.validators,
//         )?;

//         Ok(client_state)
//     }
// }

// impl From<ClientState> for RawClientState {
//     fn from(value: ClientState) -> Self {
//         let frozen_height = value.frozen_height.unwrap_or(0);
//         Self {
//             trusting_period: value.trusting_period,
//             frozen_height,
//             max_clock_drift: value.max_clock_drift,
//             latest_height: value.latest_height,
//             network_section_hash: value.network_section_hash,
//             validators: value.validators,
//         }
//     }
// }

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;
        use ibc::core::ics02_client::error::ClientError as Error;
        use prost::Message;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            <ClientState as Message>::decode(buf).map_err(ClientError::Decode)
        }

        match raw.type_url.as_str() {
            ICON_CLIENT_STATE_TYPE_URL => decode_client_state(raw.value.deref()),
            _ => Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: ICON_CLIENT_STATE_TYPE_URL.to_string(),
            value: <ClientState as Message>::encode_to_vec(&client_state),
        }
    }
}

impl TryFrom<ClientState> for Vec<u8> {
    type Error = ClientError;

    fn try_from(value: ClientState) -> Result<Self, Self::Error> {
        serde_json_wasm::to_vec(&value).map_err(|error| ClientError::Other {
            description: error.to_string(),
        })
    }
}

use ibc::core::ics02_client::client_state::ClientState as IbcClientState;
use ibc::core::ics02_client::client_type::ClientType as IbcClientType;
use ibc::core::ics24_host::identifier::ClientId as IbcClientId;
use ibc::Height as IbcHeight;
use prost::Message;

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl IbcClientState for ClientState {
    fn chain_id(&self) -> ibc::core::ics24_host::identifier::ChainId {
        todo!()
    }

    fn client_type(&self) -> IbcClientType {
        IbcClientType::new(ICON_CLIENT_TYPE.to_string())
    }

    fn latest_height(&self) -> ibc::Height {
        IbcHeight::new(0, self.latest_height).unwrap()
    }

    fn frozen_height(&self) -> Option<ibc::Height> {
        Some(IbcHeight::new(0, self.frozen_height).unwrap())
    }

    fn expired(&self, elapsed: std::time::Duration) -> bool {
        //TODO: Implement logic

        let trusting_period = Duration::from_secs(self.trusting_period);
        elapsed.as_secs() > trusting_period.as_secs()
    }

    fn zero_custom_fields(&mut self) {
        todo!()
    }

    fn initialise(
        &self,
        consensus_state: Any,
    ) -> Result<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>, ClientError>
    {
        todo!()
    }

    fn check_header_and_update_state(
        &self,
        ctx: &dyn ibc::core::ValidationContext,
        client_id: IbcClientId,
        header: Any,
    ) -> Result<ibc::core::ics02_client::client_state::UpdatedState, ClientError> {
        todo!()
    }

    fn check_misbehaviour_and_update_state(
        &self,
        ctx: &dyn ibc::core::ValidationContext,
        client_id: IbcClientId,
        misbehaviour: Any,
    ) -> Result<Box<dyn IbcClientState>, ContextError> {
        todo!()
    }

    fn verify_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: ibc_proto::ibc::core::commitment::v1::MerkleProof,
        proof_upgrade_consensus_state: ibc_proto::ibc::core::commitment::v1::MerkleProof,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn update_state_with_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
    ) -> Result<ibc::core::ics02_client::client_state::UpdatedState, ClientError> {
        todo!()
    }

    fn verify_client_consensus_state(
        &self,
        proof_height: ibc::Height,
        counterparty_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        client_cons_state_path: &ibc::core::ics24_host::path::ClientConsensusStatePath,
        expected_consensus_state: &dyn ibc::core::ics02_client::consensus_state::ConsensusState,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_connection_state(
        &self,
        proof_height: ibc::Height,
        counterparty_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        counterparty_conn_path: &ibc::core::ics24_host::path::ConnectionPath,
        expected_counterparty_connection_end: &ConnectionEnd,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_channel_state(
        &self,
        proof_height: ibc::Height,
        counterparty_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        counterparty_chan_end_path: &ibc::core::ics24_host::path::ChannelEndPath,
        expected_counterparty_channel_end: &ChannelEnd,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_client_full_state(
        &self,
        proof_height: ibc::Height,
        counterparty_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        client_state_path: &ibc::core::ics24_host::path::ClientStatePath,
        expected_client_state: Any,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_packet_data(
        &self,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        commitment_path: &ibc::core::ics24_host::path::CommitmentPath,
        commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_packet_acknowledgement(
        &self,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        ack_path: &ibc::core::ics24_host::path::AckPath,
        ack: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_next_sequence_recv(
        &self,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        seq_recv_path: &ibc::core::ics24_host::path::SeqRecvPath,
        sequence: Sequence,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_packet_receipt_absence(
        &self,
        height: ibc::Height,
        prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
        root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
        receipt_path: &ibc::core::ics24_host::path::ReceiptPath,
    ) -> Result<(), ClientError> {
        todo!()
    }
}
