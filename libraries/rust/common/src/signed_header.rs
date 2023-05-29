pub use crate::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use ibc_proto::google::protobuf::Any;
use prost::DecodeError;

use crate::constants::ICON_SIGNED_HEADER_TYPE_URL;
use prost::Message;

impl RawSignedHeader {}

impl TryFrom<Any> for RawSignedHeader {
    type Error = DecodeError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use core::ops::Deref;

        match raw.type_url.as_str() {
            ICON_SIGNED_HEADER_TYPE_URL => <RawSignedHeader as Message>::decode(raw.value.deref()),
            _ => Err(DecodeError::new("Invalid type url")),
        }
    }
}
// impl Protobuf<Any> for RawSignedHeader {}

impl From<RawSignedHeader> for Any {
    fn from(value: RawSignedHeader) -> Self {
        Any {
            type_url: ICON_SIGNED_HEADER_TYPE_URL.to_string(),
            value: RawSignedHeader::encode_to_vec(&value),
        }
    }
}
