use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct HexString(String);

impl HexString {
    pub fn to_bytes(&self) -> Result<Vec<u8>, hex::FromHexError> {
        let str = self.0.replace("0x", "");
        if str.len() == 0 {
            return Ok(Vec::<u8>::new());
        }
        hex::decode(str)
    }

    pub fn from_bytes(bytes: &[u8]) -> HexString {
        HexString(hex::encode(bytes))
    }
}
