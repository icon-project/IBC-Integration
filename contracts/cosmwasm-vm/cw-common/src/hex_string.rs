use std::{borrow::Cow, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;
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

impl FromStr for HexString {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexString(s.to_owned()))
    }
}

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
    use std::str::FromStr;

    use hex_buffer_serde::Hex;

    use crate::hex_string::{FromHexString, HexString};

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

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0x01, 0x02, 0x03];
        let _hex = FromHexString(());

        assert_eq!(FromHexString::from_bytes(&bytes).unwrap(), bytes);
    }

    #[test]
    fn test_from_str() {
        let s = "0x010203";
        let hex_string = HexString::from_str(s).unwrap();
        let expected = HexString(s.to_owned());

        assert_eq!(hex_string, expected);
    }

    #[test]
    fn test_to_bytes() {
        let hex_string = HexString("010203".to_owned());
        let bytes = hex_string.to_bytes().unwrap();
        let expected = vec![0x01, 0x02, 0x03];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_from_str_into_hex_string() {
        let s = "0x010203";
        let hex_string: HexString = s.into();
        let expected = HexString(s.to_owned());

        assert_eq!(hex_string, expected);
    }

    #[test]
    fn test_from_bytes_into_hex_string() {
        let bytes = vec![0x01, 0x02, 0x03];
        let hex_string = HexString::from_bytes(&bytes);
        let expected = HexString("010203".to_owned());

        assert_eq!(hex_string, expected);
    }
}
