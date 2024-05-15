use crate::ibc::prelude::*;
use tendermint::merkle::proof::ProofOps as TendermintProof;

use ibc_proto::ibc::core::commitment::v1::MerklePath;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleRoot;

use ics23::CommitmentProof;

use crate::ibc::core::ics23_commitment::commitment::{CommitmentPrefix, CommitmentRoot};
use crate::ibc::core::ics23_commitment::error::CommitmentError;
// use crate::ibc::core::ics23_commitment::specs::ProofSpecs;

pub fn apply_prefix(prefix: &CommitmentPrefix, mut path: Vec<String>) -> MerklePath {
    let mut key_path: Vec<String> = vec![format!("{prefix:?}")];
    key_path.append(&mut path);
    MerklePath { key_path }
}

impl From<CommitmentRoot> for MerkleRoot {
    fn from(root: CommitmentRoot) -> Self {
        Self {
            hash: root.into_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleProof {
    pub proofs: Vec<CommitmentProof>,
}

/// Convert to ics23::CommitmentProof
/// The encoding and decoding shouldn't fail since ics23::CommitmentProof and ibc_proto::ics23::CommitmentProof should be the same
/// Ref. <https://github.com/informalsystems/ibc-rs/issues/853>
impl From<RawMerkleProof> for MerkleProof {
    fn from(proof: RawMerkleProof) -> Self {
        let proofs: Vec<CommitmentProof> = proof
            .proofs
            .into_iter()
            .map(|p| {
                let mut encoded = Vec::new();
                prost::Message::encode(&p, &mut encoded).unwrap();
                prost::Message::decode(&*encoded).unwrap()
            })
            .collect();
        Self { proofs }
    }
}

impl From<MerkleProof> for RawMerkleProof {
    fn from(proof: MerkleProof) -> Self {
        Self {
            proofs: proof
                .proofs
                .into_iter()
                .map(|p| {
                    let mut encoded = Vec::new();
                    prost::Message::encode(&p, &mut encoded).unwrap();
                    prost::Message::decode(&*encoded).unwrap()
                })
                .collect(),
        }
    }
}

pub fn convert_tm_to_ics_merkle_proof(
    tm_proof: &TendermintProof,
) -> Result<MerkleProof, CommitmentError> {
    let mut proofs = Vec::new();

    for op in &tm_proof.ops {
        let mut parsed = ibc_proto::ics23::CommitmentProof { proof: None };
        prost::Message::merge(&mut parsed, op.data.as_slice())
            .map_err(CommitmentError::CommitmentProofDecodingFailed)?;

        proofs.push(parsed);
    }

    Ok(MerkleProof::from(RawMerkleProof { proofs }))
}
