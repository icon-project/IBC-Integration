use cosmwasm_schema::cw_serde;
use cw_common::xcall_types::network_address::NetId;

#[cw_serde]
pub struct ChannelConfig {
    pub client_id: String,
    pub timeout_height: u64,
    pub counterparty_nid: NetId,
}
