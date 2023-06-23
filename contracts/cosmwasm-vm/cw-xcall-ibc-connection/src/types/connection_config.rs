use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ConnectionConfig {
    pub lightclient_address: String,
    pub client_id: String,
    pub timeout_height: u64,
}
