use cw_common::hex_string::HexString;

use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {
    pub clear_store: bool,
}
