use std::time::Duration;

use crate::ibc_types::IbcTimestamp;
use common::icon::icon::lightclient::v1::ConsensusState as RawConsensusState;
use ibc::core::{ics02_client::error::ClientError, ics23_commitment::commitment::CommitmentRoot};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use serde::{Deserialize, Serialize};

use crate::constants::ICON_CONSENSUS_STATE_TYPE_URL;

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
// pub struct ConsensusState {
//     message_root: CommitmentRoot,
// }

// impl ConsensusState {
//     pub fn new(message_root: Vec<u8>) -> Result<Self, ClientError> {
//         let commitment_root = CommitmentRoot::from(message_root);
//         Ok(Self {
//             message_root: commitment_root,
//         })
//     }
//     pub fn message_root(&self) -> &CommitmentRoot {
//         &self.message_root
//     }
//     pub fn as_bytes(&self) -> &[u8] {
//         self.message_root.as_bytes()
//     }
// }

// impl Protobuf<Any> for ConsensusState {}

// impl TryFrom<Any> for ConsensusState {
//     type Error = ClientError;

//     fn try_from(raw: Any) -> Result<Self, Self::Error> {
//         use bytes::Buf;
//         use ibc::core::ics02_client::error::ClientError as Error;
//         use prost::Message;
//         use std::ops::Deref;

//         fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
//             RawConsensusState::decode(buf)
//                 .map_err(ClientError::Decode)?
//                 .try_into()
//         }

//         match raw.type_url.as_str() {
//             ICON_CONSENSUS_STATE_TYPE_URL => decode_consensus_state(raw.value.deref()),
//             _ => Err(ClientError::UnknownConsensusStateType {
//                 consensus_state_type: raw.type_url,
//             }),
//         }
//     }
// }

// impl From<ConsensusState> for Any {
//     fn from(consensus_state: ConsensusState) -> Self {
//         Any {
//             type_url: ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
//             value: Protobuf::<RawConsensusState>::encode_vec(&consensus_state)
//                 .expect("encoding to `Any` from `IconConensusState`"),
//         }
//     }
// }

// impl Protobuf<RawConsensusState> for ConsensusState {}

// impl TryFrom<RawConsensusState> for ConsensusState {
//     type Error = ClientError;

//     fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
//         let consensus_state = Self::new(raw.message_root)?;

//         Ok(consensus_state)
//     }
// }

// impl From<ConsensusState> for RawConsensusState {
//     fn from(value: ConsensusState) -> Self {
//         Self {
//             message_root: value.message_root().clone().into_vec(),
//         }
//     }
// }

// impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
//     fn root(&self) -> &CommitmentRoot {
//         self.message_root()
//     }

//     fn timestamp(&self) -> IbcTimestamp {
//         // TODO: Update the timestamp logic

//         let block_time = Duration::from_secs(3);
//         IbcTimestamp::from_nanoseconds(block_time.as_nanos() as u64).unwrap()
//     }

//     fn into_box(self) -> Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>
//     where
//         Self: Sized,
//     {
//         Box::new(self)
//     }
// }

// impl TryFrom<Vec<u8>> for ConsensusState {
//     type Error = ClientError;

//     fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
//         let result: ConsensusStateResponse =
//             serde_json_wasm::from_slice(&value).map_err(|error| ClientError::Other {
//                 description: error.to_string(),
//             })?;

//         let commit = CommitmentRoot::from(hex::decode(result.message_root).map_err(|error| {
//             ClientError::Other {
//                 description: error.to_string(),
//             }
//         })?);

//         Ok(Self {
//             message_root: commit,
//         })
//     }
// }

// impl TryFrom<ConsensusState> for Vec<u8> {
//     type Error = ClientError;

//     fn try_from(value: ConsensusState) -> Result<Self, Self::Error> {
//         serde_json_wasm::to_vec(&value).map_err(|error| ClientError::Other {
//             description: error.to_string(),
//         })
//     }
// }

#[derive(Debug, Deserialize)]
struct ConsensusStateResponse {
    pub message_root: String,
}
