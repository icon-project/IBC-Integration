use std::time::Duration;

use crate::ibc::{
    core::{ics02_client::error::ClientError, ics23_commitment::commitment::CommitmentRoot},
    timestamp::Timestamp,
};
use dyn_clone::DynClone;
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use prost::Message;

use crate::{
    constants::ICON_CONSENSUS_STATE_TYPE_URL, icon::icon::lightclient::v1::ConsensusState,
};

impl ConsensusState {
    pub fn new(message_root: Vec<u8>) -> Result<Self, ClientError> {
        Ok(Self { message_root })
    }
    pub fn message_root(&self) -> CommitmentRoot {
        CommitmentRoot::from(self.message_root.clone())
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.message_root.as_slice()
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use crate::ibc::core::ics02_client::error::ClientError as Error;
        use bytes::Buf;
        use std::ops::Deref;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            <ConsensusState as Message>::decode(buf).map_err(ClientError::Decode)
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
            value: <ConsensusState as Message>::encode_to_vec(&consensus_state),
        }
    }
}

pub trait IConsensusState: core::fmt::Debug + Send + Sync + DynClone + prost::Message {
    fn root(&self) -> CommitmentRoot;
    fn timestamp(&self) -> Timestamp;
    fn into_box(self) -> Box<dyn IConsensusState>;
    fn as_bytes(&self) -> Vec<u8>;
}

impl IConsensusState for ConsensusState {
    fn root(&self) -> CommitmentRoot {
        self.message_root()
    }

    fn timestamp(&self) -> Timestamp {
        // TODO: Update the timestamp logic

        let block_time = Duration::from_secs(3);
        Timestamp::from_nanoseconds(block_time.as_nanos() as u64).unwrap()
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }

    fn into_box(self) -> Box<dyn IConsensusState>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
dyn_clone::clone_trait_object!(IConsensusState);


