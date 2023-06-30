use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub port_id: String,
    pub denom: String,
}
