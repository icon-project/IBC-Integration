use crate::ibc::core::ics23_commitment::error::CommitmentError;
use crate::ibc::prelude::*;

use core::{convert::TryFrom, fmt};
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use serde::{Deserialize, Serialize};

use super::merkle::MerkleProof;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentRoot {
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "crate::serializers::ser_hex_upper")
    )]
    bytes: Vec<u8>,
}

impl fmt::Debug for CommitmentRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = hex::encode(&self.bytes);
        f.debug_tuple("CommitmentRoot").field(&hex).finish()
    }
}

impl CommitmentRoot {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: Vec::from(bytes),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

impl From<Vec<u8>> for CommitmentRoot {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitmentPath;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentProofBytes {
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "crate::serializers::ser_hex_upper")
    )]
    bytes: Vec<u8>,
}

impl fmt::Debug for CommitmentProofBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = hex::encode(&self.bytes);
        f.debug_tuple("CommitmentProof").field(&hex).finish()
    }
}

impl TryFrom<Vec<u8>> for CommitmentProofBytes {
    type Error = CommitmentError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
       
            Ok(Self { bytes })
        
    }
}

impl From<CommitmentProofBytes> for Vec<u8> {
    fn from(p: CommitmentProofBytes) -> Vec<u8> {
        p.bytes
    }
}

impl TryFrom<RawMerkleProof> for CommitmentProofBytes {
    type Error = CommitmentError;

    fn try_from(proof: RawMerkleProof) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        prost::Message::encode(&proof, &mut buf).unwrap();
        buf.try_into()
    }
}

impl TryFrom<MerkleProof> for CommitmentProofBytes {
    type Error = CommitmentError;

    fn try_from(value: MerkleProof) -> Result<Self, Self::Error> {
        Self::try_from(RawMerkleProof::from(value))
    }
}

impl TryFrom<CommitmentProofBytes> for RawMerkleProof {
    type Error = CommitmentError;

    fn try_from(value: CommitmentProofBytes) -> Result<Self, Self::Error> {
        let value: Vec<u8> = value.into();
        let res: RawMerkleProof = prost::Message::decode(value.as_ref())
            .map_err(CommitmentError::InvalidRawMerkleProof)?;
        Ok(res)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct CommitmentPrefix {
    bytes: Vec<u8>,
}

impl CommitmentPrefix {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

impl TryFrom<Vec<u8>> for CommitmentPrefix {
    type Error = CommitmentError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            Err(Self::Error::EmptyCommitmentPrefix)
        } else {
            Ok(Self { bytes })
        }
    }
}

impl fmt::Debug for CommitmentPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let converted = core::str::from_utf8(self.as_bytes());
        match converted {
            Ok(s) => write!(f, "{s}"),
            Err(_e) => write!(f, "<not valid UTF8: {:?}>", self.as_bytes()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CommitmentPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{self:?}").serialize(serializer)
    }
}
