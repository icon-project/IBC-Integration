use crate::ibc::prelude::*;

use derive_more::Into;

use crate::ibc::core::ics04_channel::error::PacketError;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgAcknowledgement";

#[derive(Clone, Debug, PartialEq, Eq, Into, serde::Serialize, serde::Deserialize)]
pub struct Acknowledgement(Vec<u8>);

impl Acknowledgement {
    // Returns the data as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn from_bytes(b: &[u8]) -> Acknowledgement {
        Acknowledgement(b.to_vec())
    }
}

impl AsRef<[u8]> for Acknowledgement {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl TryFrom<Vec<u8>> for Acknowledgement {
    type Error = PacketError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            Err(PacketError::InvalidAcknowledgement)
        } else {
            Ok(Self(bytes))
        }
    }
}
