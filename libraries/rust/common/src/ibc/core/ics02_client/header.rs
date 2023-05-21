use crate::ibc::prelude::*;

use crate::ibc::core::ics02_client::error::ClientError;
use crate::ibc::timestamp::Timestamp;
use crate::ibc::Height;
use dyn_clone::DynClone;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf as ErasedProtobuf;

/// Abstract of consensus state update information
///
/// Users are not expected to implement sealed::ErasedPartialEqHeader.
/// Effectively, that trait bound mandates implementors to derive PartialEq,
/// after which our blanket implementation will implement
/// `ErasedPartialEqHeader` for their type.
pub trait Header:
    DynClone + ErasedProtobuf<Any, Error = ClientError> + core::fmt::Debug + Send + Sync
{
    /// The height of the consensus state
    fn height(&self) -> Height;

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp;

    /// Convert into a boxed trait object
    fn into_box(self) -> Box<dyn Header>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

dyn_clone::clone_trait_object!(Header);
