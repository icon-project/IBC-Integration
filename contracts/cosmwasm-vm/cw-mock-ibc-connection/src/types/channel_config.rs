use cosmwasm_schema::cw_serde;
use cw_xcall_lib::network_address::NetId;

#[cw_serde]
pub struct ChannelConfig {
    pub client_id: String,
    pub timeout_height: u64,
    pub counterparty_nid: NetId,
}
