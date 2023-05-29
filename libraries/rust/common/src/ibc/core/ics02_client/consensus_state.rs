use crate::ibc::prelude::*;

use core::marker::{Send, Sync};

use dyn_clone::DynClone;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf as ErasedProtobuf;

use crate::ibc::core::ics02_client::error::ClientError;
use crate::ibc::core::ics23_commitment::commitment::CommitmentRoot;
use crate::ibc::timestamp::Timestamp;

/// Abstract of consensus state information used by the validity predicate
/// to verify new commits & state roots.
///
/// Users are not expected to implement sealed::ErasedPartialEqConsensusState.
/// Effectively, that trait bound mandates implementors to derive PartialEq,
/// after which our blanket implementation will implement
/// `ErasedPartialEqConsensusState` for their type.
pub trait ConsensusState:
    DynClone + ErasedProtobuf<Any, Error = ClientError> + core::fmt::Debug + Send + Sync
{
    /// Commitment root of the consensus state, which is used for key-value pair verification.
    fn root(&self) -> &CommitmentRoot;

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp;

    /// Convert into a boxed trait object
    fn into_box(self) -> Box<dyn ConsensusState>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

// Implements `Clone` for `Box<dyn ConsensusState>`
dyn_clone::clone_trait_object!(ConsensusState);

// Implements `serde::Serialize` for all types that have ConsensusState as supertrait
#[cfg(feature = "serde")]
erased_serde::serialize_trait_object!(ConsensusState);
