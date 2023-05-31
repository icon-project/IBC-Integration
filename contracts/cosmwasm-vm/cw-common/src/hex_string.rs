use std::borrow::Cow;

use cosmwasm_schema::cw_serde;
use hex_buffer_serde::Hex;
use schemars::JsonSchema;

#[derive(JsonSchema)]
struct FromHexString(());

impl Hex<Vec<u8>> for FromHexString {
    type Error = hex::FromHexError;

    fn create_bytes(value: &Vec<u8>) -> Cow<'_, [u8]> {
        Cow::Borrowed(&value[..])
    }

    fn from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Self::Error> {
        Ok(bytes.to_vec())
    }
}

#[cw_serde]
pub struct HexString(String);

impl HexString {
    pub fn to_bytes(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let str = self.0.replace("0x", "");
        if str.is_empty() {
            return Ok(Vec::<u8>::new());
        }
        hex::decode(str)
    }

    pub fn from_bytes(bytes: &[u8]) -> HexString {
        HexString(hex::encode(bytes))
    }

    pub fn from_str(str: &str) -> Self {
        HexString(str.to_owned())
    }
}

impl From<&str> for HexString {
    fn from(value: &str) -> Self {
        HexString(value.to_owned())
    }
}

#[cw_serde]
pub struct TestHex {
    #[serde(with = "FromHexString")]
    pub bytes: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::TestHex;

    #[test]
    fn test_hex_serialize_deserialize() {
        let test = TestHex {
            bytes: hex::decode("deadbeef").unwrap(),
        };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!("{\"bytes\":\"deadbeef\"}", serialized.to_string());
        let deserialized = serde_json::from_str::<TestHex>("{\"bytes\":\"deadbeef\"}").unwrap();
        assert_eq!(test, deserialized);
    }
}
