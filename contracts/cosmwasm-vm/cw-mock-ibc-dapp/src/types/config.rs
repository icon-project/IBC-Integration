use cosmwasm_schema::cw_serde;
use cw_common::cw_types::CwOrder;

#[cw_serde]
pub struct Config {
    pub port_id: String,
    pub denom: String,
    pub order: CwOrder,
}
