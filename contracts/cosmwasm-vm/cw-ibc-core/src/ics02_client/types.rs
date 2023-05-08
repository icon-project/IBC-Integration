use super::*;
use common::icon::icon::types::v1::BtpHeader as RawBtpHeader;
use common::icon::icon::types::v1::MerkleNode as RawMerkleNode;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use cw_common::constants::ICON_BTP_HEADER_TYPE_URL;
use cw_common::constants::ICON_CLIENT_STATE_TYPE_URL;
use cw_common::constants::ICON_CLIENT_TYPE;
use cw_common::constants::ICON_CONSENSUS_STATE_TYPE_URL;
use cw_common::constants::ICON_SIGNED_HEADER_TYPE_URL;

/// This struct representing the state of a client, with various fields such as trusting
/// period, frozen height, and validators.
/// 
/// Properties:
/// 
/// * `trusting_period`: The duration of time for which the client trusts the validity of a received
/// consensus state.
/// * `frozen_height`: `frozen_height` is an optional field in the `ClientState` struct that represents
/// the height at which the client state was frozen. If the value is `None`, it means that the client
/// state is not frozen and can be updated. If the value is `Some(height)`, it means
/// * `max_clock_drift`: `max_clock_drift` is a property of the `ClientState` struct that represents the
/// maximum allowed clock drift in nanoseconds between the client and the blockchain network. It is used
/// to ensure that the client's clock is synchronized with the network's clock to prevent attacks such
/// as replay attacks.
/// * `latest_height`: The latest height is the most recent block height that the client has verified
/// and considers as valid. This is an important property for a blockchain client as it helps to ensure
/// that the client is up-to-date with the latest state of the blockchain.
/// * `network_section_hash`: The `network_section_hash` property is a vector of bytes that represents
/// the hash of the network section that the client is associated with. The network section is a group
/// of validators that are responsible for maintaining the consensus of the blockchain network. The
/// client uses this hash to verify that it is connected to the
/// * `validators`: `validators` is a vector of byte arrays representing the public keys of the
/// validators that the client is configured to trust. These validators are expected to sign and
/// validate blocks on the network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientState {
    trusting_period: u64,
    frozen_height: Option<u64>,
    max_clock_drift: u64,
    latest_height: u64,
    network_section_hash: Vec<u8>,
    validators: Vec<Vec<u8>>,
}

impl TryFrom<&[u8]> for ClientState {
    type Error = ClientError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match serde_json_wasm::from_slice(value) {
            Ok(result) => Ok(result),
            Err(error) => Err(ClientError::Other {
                description: error.to_string(),
            }),
        }
    }
}

impl ClientState {
    pub fn new(
        trusting_period: u64,
        frozen_height: u64,
        max_clock_drift: u64,
        latest_height: u64,
        network_section_hash: Vec<u8>,
        validators: Vec<Vec<u8>>,
    ) -> Result<Self, ClientError> {
        let frozen_height = match frozen_height {
            0 => None,
            _ => Some(frozen_height),
        };

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

impl Protobuf<RawClientState> for ClientState {}
impl TryFrom<RawClientState> for ClientState {
    type Error = ClientError;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        let client_state = Self::new(
            raw.trusting_period,
            raw.frozen_height,
            raw.max_clock_drift,
            raw.latest_height,
            raw.network_section_hash,
            raw.validators,
        )?;

        Ok(client_state)
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        let frozen_height = value.frozen_height.unwrap_or(0);
        Self {
            trusting_period: value.trusting_period,
            frozen_height,
            max_clock_drift: value.max_clock_drift,
            latest_height: value.latest_height,
            network_section_hash: value.network_section_hash,
            validators: value.validators,
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;
        use ibc::core::ics02_client::error::ClientError as Error;
        use prost::Message;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            RawClientState::decode(buf)
                .map_err(ClientError::Decode)?
                .try_into()
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
            value: Protobuf::<RawClientState>::encode_vec(&client_state)
                .expect("encoding to `Any` from `IconConensusState`"),
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
        self.frozen_height
            .map(|height| IbcHeight::new(0, height).unwrap())
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ConsensusState {
    message_root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(message_root: Vec<u8>) -> Result<Self, ClientError> {
        let commitment_root = CommitmentRoot::from(message_root);
        Ok(Self {
            message_root: commitment_root,
        })
    }
    pub fn message_root(&self) -> &CommitmentRoot {
        &self.message_root
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.message_root.as_bytes()
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use ibc::core::ics02_client::error::ClientError as Error;
        use prost::Message;
        use std::ops::Deref;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            RawConsensusState::decode(buf)
                .map_err(ClientError::Decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            ICON_CONSENSUS_STATE_TYPE_URL => decode_consensus_state(raw.value.deref()),
            _ => Err(ClientError::UnknownConsensusStateType {
                consensus_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Any {
            type_url: ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawConsensusState>::encode_vec(&consensus_state)
                .expect("encoding to `Any` from `IconConensusState`"),
        }
    }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        let consensus_state = Self::new(raw.message_root)?;

        Ok(consensus_state)
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        Self {
            message_root: value.message_root().clone().into_vec(),
        }
    }
}

impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    fn root(&self) -> &CommitmentRoot {
        self.message_root()
    }

    fn timestamp(&self) -> IbcTimestamp {
        // TODO: Update the timestamp logic

        let block_time = Duration::from_secs(3);
        IbcTimestamp::from_nanoseconds(block_time.as_nanos() as u64).unwrap()
    }

    fn into_box(self) -> Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl TryFrom<Vec<u8>> for ConsensusState {
    type Error = ClientError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let result: ConsensusStateResponse =
            serde_json_wasm::from_slice(&value).map_err(|error| ClientError::Other {
                description: error.to_string(),
            })?;

        let commit = CommitmentRoot::from(hex::decode(result.message_root).map_err(|error| {
            ClientError::Other {
                description: error.to_string(),
            }
        })?);

        Ok(Self {
            message_root: commit,
        })
    }
}

impl TryFrom<ConsensusState> for Vec<u8> {
    type Error = ClientError;

    fn try_from(value: ConsensusState) -> Result<Self, Self::Error> {
        serde_json_wasm::to_vec(&value).map_err(|error| ClientError::Other {
            description: error.to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct ConsensusStateResponse {
    pub message_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedHeader {
    pub header: BtpHeader,
    pub signatures: Vec<Vec<u8>>,
}

impl TryFrom<Any> for SignedHeader {
    type Error = ContractError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use crate::error::ContractError as Error;
        use bytes::Buf;
        use core::ops::Deref;
        use prost::Message;

        fn decode_signed_header<B: Buf>(buf: B) -> Result<SignedHeader, Error> {
            RawSignedHeader::decode(buf)
                .map_err(|error| ContractError::IbcDecodeError {
                    error: error.to_string(),
                })?
                .try_into()
        }

        match raw.type_url.as_str() {
            ICON_BTP_HEADER_TYPE_URL => decode_signed_header(raw.value.deref()),
            _ => Err(ContractError::IbcDecodeError {
                error: "Invalid Type".to_string(),
            }),
        }
    }
}
impl Protobuf<Any> for SignedHeader {}

impl From<SignedHeader> for Any {
    fn from(value: SignedHeader) -> Self {
        Any {
            type_url: ICON_SIGNED_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawSignedHeader>::encode_vec(&value)
                .expect("encoding to `Any` from `BtpHeader`"),
        }
    }
}
impl From<SignedHeader> for RawSignedHeader {
    fn from(value: SignedHeader) -> Self {
        let network_section_to_root = value
            .header
            .network_section_to_root
            .into_iter()
            .map(|data| common::icon::icon::types::v1::MerkleNode {
                dir: data.dir,
                value: data.value,
            })
            .collect::<Vec<common::icon::icon::types::v1::MerkleNode>>();
        let btp_header = RawBtpHeader {
            main_height: value.header.main_height,
            round: value.header.round,
            next_proof_context_hash: value.header.next_proof_context_hash,
            network_section_to_root,
            network_id: value.header.network_id,
            update_number: value.header.update_number,
            prev_network_section_hash: value.header.prev_network_section_hash,
            message_count: value.header.message_count,
            message_root: value.header.message_root,
            next_validators: value.header.next_validators,
        };
        Self {
            header: Some(btp_header),
            signatures: value.signatures,
        }
    }
}
impl Protobuf<RawSignedHeader> for SignedHeader {}
impl TryFrom<RawSignedHeader> for SignedHeader {
    type Error = ContractError;

    fn try_from(value: RawSignedHeader) -> Result<Self, Self::Error> {
        let btp_header: BtpHeader = value.header.unwrap().try_into().map_err(|error| error)?;
        let signed_header = Self {
            header: btp_header,
            signatures: value.signatures,
        };

        Ok(signed_header)
    }
}

impl TryFrom<Vec<u8>> for SignedHeader {
    type Error = ContractError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match serde_json_wasm::from_slice(value.as_slice()) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: error.to_string(),
            }),
        }
    }
}
impl TryFrom<SignedHeader> for Vec<u8> {
    type Error = ContractError;

    fn try_from(value: SignedHeader) -> Result<Self, Self::Error> {
        serde_json_wasm::to_vec(&value).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BtpHeader {
    pub main_height: u64,
    pub round: u32,
    pub next_proof_context_hash: Vec<u8>,
    pub network_section_to_root: Vec<MerkleNode>,
    pub network_id: u64,
    pub update_number: u64,
    pub prev_network_section_hash: Vec<u8>,
    pub message_count: u64,
    pub message_root: Vec<u8>,
    pub next_validators: Vec<Vec<u8>>,
}

impl Protobuf<RawBtpHeader> for BtpHeader {}
impl TryFrom<RawBtpHeader> for BtpHeader {
    type Error = ContractError;

    fn try_from(value: RawBtpHeader) -> Result<Self, Self::Error> {
        let network_section_to_root = value
            .network_section_to_root
            .into_iter()
            .map(|data| MerkleNode {
                dir: data.dir,
                value: data.value,
            })
            .collect::<Vec<MerkleNode>>();
        let btp_header = BtpHeader {
            main_height: value.main_height,
            round: value.round,
            next_proof_context_hash: value.next_proof_context_hash,
            network_section_to_root,
            network_id: value.network_id,
            update_number: value.update_number,
            prev_network_section_hash: value.prev_network_section_hash,
            message_count: value.message_count,
            message_root: value.message_root,
            next_validators: value.next_validators,
        };

        Ok(btp_header)
    }
}

impl From<BtpHeader> for Any {
    fn from(value: BtpHeader) -> Self {
        Any {
            type_url: ICON_BTP_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawBtpHeader>::encode_vec(&value)
                .expect("encoding to `Any` from `BtpHeader`"),
        }
    }
}

impl From<BtpHeader> for RawBtpHeader {
    fn from(value: BtpHeader) -> Self {
        let network_section_to_root = value
            .network_section_to_root
            .into_iter()
            .map(|data| common::icon::icon::types::v1::MerkleNode {
                dir: data.dir,
                value: data.value,
            })
            .collect::<Vec<common::icon::icon::types::v1::MerkleNode>>();
        Self {
            main_height: value.main_height,
            round: value.round,
            next_proof_context_hash: value.next_proof_context_hash,
            network_section_to_root,
            network_id: value.network_id,
            update_number: value.update_number,
            prev_network_section_hash: value.prev_network_section_hash,
            message_count: value.message_count,
            message_root: value.message_root,
            next_validators: value.next_validators,
        }
    }
}

impl Protobuf<Any> for BtpHeader {}
impl TryFrom<Any> for BtpHeader {
    type Error = ContractError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use crate::error::ContractError as Error;
        use bytes::Buf;
        use core::ops::Deref;
        use prost::Message;

        fn decode_btp_header<B: Buf>(buf: B) -> Result<BtpHeader, Error> {
            RawBtpHeader::decode(buf)
                .map_err(|error| ContractError::IbcDecodeError {
                    error: error.to_string(),
                })?
                .try_into()
        }

        match raw.type_url.as_str() {
            ICON_BTP_HEADER_TYPE_URL => decode_btp_header(raw.value.deref()),
            _ => Err(ContractError::IbcDecodeError {
                error: "Invalid Type".to_string(),
            }),
        }
    }
}

impl TryFrom<Vec<u8>> for BtpHeader {
    type Error = ContractError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match serde_json_wasm::from_slice(value.as_slice()) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: error.to_string(),
            }),
        }
    }
}

impl TryFrom<BtpHeader> for Vec<u8> {
    type Error = ContractError;

    fn try_from(value: BtpHeader) -> Result<Self, Self::Error> {
        serde_json_wasm::to_vec(&value).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MerkleNode {
    pub dir: i32,
    pub value: Vec<u8>,
}

impl Protobuf<Any> for MerkleNode {}

impl TryFrom<Any> for MerkleNode {
    type Error = ContractError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use crate::error::ContractError as Error;
        use bytes::Buf;
        use core::ops::Deref;
        use prost::Message;

        fn decode_merkle_node<B: Buf>(buf: B) -> Result<MerkleNode, Error> {
            RawMerkleNode::decode(buf)
                .map_err(|error| ContractError::IbcDecodeError {
                    error: error.to_string(),
                })?
                .try_into()
        }

        match raw.type_url.as_str() {
            ICON_MERKLE_TYPE_URL => decode_merkle_node(raw.value.deref()),
            _ => Err(ContractError::IbcDecodeError {
                error: "Invalid Type".to_string(),
            }),
        }
    }
}

impl From<MerkleNode> for RawMerkleNode {
    fn from(value: MerkleNode) -> Self {
        Self {
            dir: value.dir,
            value: value.value,
        }
    }
}

impl From<MerkleNode> for Any {
    fn from(value: MerkleNode) -> Self {
        Any {
            type_url: ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawMerkleNode>::encode_vec(&value)
                .expect("encoding to `Any` from `MerkleNode`"),
        }
    }
}

impl Protobuf<RawMerkleNode> for MerkleNode {}
impl TryFrom<RawMerkleNode> for MerkleNode {
    type Error = ContractError;

    fn try_from(value: RawMerkleNode) -> Result<Self, Self::Error> {
        let merkle_node = MerkleNode {
            dir: value.dir,
            value: value.value,
        };

        Ok(merkle_node)
    }
}
impl TryFrom<Vec<u8>> for MerkleNode {
    type Error = ContractError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match serde_json_wasm::from_slice(value.as_slice()) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: error.to_string(),
            }),
        }
    }
}

impl TryFrom<MerkleNode> for Vec<u8> {
    type Error = ContractError;

    fn try_from(value: MerkleNode) -> Result<Self, Self::Error> {
        serde_json_wasm::to_vec(&value).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })
    }
}
