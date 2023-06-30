use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub network_id: String,
    pub denom: String,
}
