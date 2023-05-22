//! Definition of domain type msg `MsgUpgradeAnyClient`.

use crate::ibc::prelude::*;

use core::str::FromStr;

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpgradeClient as RawMsgUpgradeClient;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::protobuf::Protobuf;

use crate::ibc::core::ics02_client::error::ClientError;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::core::ics23_commitment::error::CommitmentError;
use crate::ibc::core::ics24_host::identifier::ClientId;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub(crate) const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpgradeClient";

/// A type of message that triggers the upgrade of an on-chain (IBC) client.
#[derive(Clone, Debug, PartialEq)]
pub struct MsgUpgradeClient {
    // client unique identifier
    pub client_id: ClientId,
    // Upgraded client state
    pub client_state: Any,
    // Upgraded consensus state, only contains enough information
    // to serve as a basis of trust in update logic
    pub consensus_state: Any,
    // proof that old chain committed to new client
    pub proof_upgrade_client: RawMerkleProof,
    // proof that old chain committed to new consensus state
    pub proof_upgrade_consensus_state: RawMerkleProof,
    // signer address
    pub signer: Signer,
}

impl Msg for MsgUpgradeClient {
    type Raw = RawMsgUpgradeClient;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgUpgradeClient> for MsgUpgradeClient {}

impl From<MsgUpgradeClient> for RawMsgUpgradeClient {
    fn from(dm_msg: MsgUpgradeClient) -> RawMsgUpgradeClient {
        let c_bytes = CommitmentProofBytes::try_from(dm_msg.proof_upgrade_client)
            .map_or(vec![], |c| c.into());
        let cs_bytes = CommitmentProofBytes::try_from(dm_msg.proof_upgrade_consensus_state)
            .map_or(vec![], |c| c.into());

        RawMsgUpgradeClient {
            client_id: dm_msg.client_id.to_string(),
            client_state: Some(dm_msg.client_state),
            consensus_state: Some(dm_msg.consensus_state),
            proof_upgrade_client: c_bytes,
            proof_upgrade_consensus_state: cs_bytes,
            signer: dm_msg.signer.to_string(),
        }
    }
}

impl TryFrom<RawMsgUpgradeClient> for MsgUpgradeClient {
    type Error = ClientError;

    fn try_from(proto_msg: RawMsgUpgradeClient) -> Result<Self, Self::Error> {
        let raw_client_state = proto_msg
            .client_state
            .ok_or(ClientError::MissingRawClientState)?;

        let raw_consensus_state = proto_msg
            .consensus_state
            .ok_or(ClientError::MissingRawConsensusState)?;

        let c_bytes =
            CommitmentProofBytes::try_from(proto_msg.proof_upgrade_client).map_err(|_| {
                ClientError::InvalidUpgradeClientProof(CommitmentError::EmptyMerkleProof)
            })?;
        let cs_bytes = CommitmentProofBytes::try_from(proto_msg.proof_upgrade_consensus_state)
            .map_err(|_| {
                ClientError::InvalidUpgradeConsensusStateProof(CommitmentError::EmptyMerkleProof)
            })?;

        Ok(MsgUpgradeClient {
            client_id: ClientId::from_str(&proto_msg.client_id)
                .map_err(ClientError::InvalidClientIdentifier)?,
            client_state: raw_client_state,
            consensus_state: raw_consensus_state,
            proof_upgrade_client: RawMerkleProof::try_from(c_bytes)
                .map_err(ClientError::InvalidUpgradeClientProof)?,
            proof_upgrade_consensus_state: RawMerkleProof::try_from(cs_bytes)
                .map_err(ClientError::InvalidUpgradeConsensusStateProof)?,
            signer: proto_msg.signer.parse().map_err(ClientError::Signer)?,
        })
    }
}
