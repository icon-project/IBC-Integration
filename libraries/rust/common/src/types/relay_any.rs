use cosmwasm_schema::cw_serde;
use ibc_proto::google::protobuf::Any;
use prost::DecodeError;

use super::hex_string::HexString;
use hex::FromHexError;
#[cw_serde]
pub struct RelayAny {
    pub type_url: String,
    pub value: HexString,
}

impl RelayAny {
    pub fn try_inner<T: prost::Message + std::default::Default>(&self) -> Result<T, DecodeError> {
        let bytes = self
            .value
            .to_bytes()
            .map_err(|e| DecodeError::new("error decoding"))?;
        return T::decode(bytes.as_slice());
    }

    pub fn to_any(&self) -> Result<Any, FromHexError> {
        let bytes = self.value.to_bytes()?;
        return Ok(Any {
            type_url: self.type_url.to_owned(),
            value: bytes,
        });
    }
}
