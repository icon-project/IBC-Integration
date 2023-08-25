use super::*;

#[cw_serde]
pub struct MigrateMsg {
    pub clear_store: bool,
}
