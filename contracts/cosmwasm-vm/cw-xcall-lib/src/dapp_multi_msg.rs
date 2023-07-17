use cosmwasm_schema::cw_serde;

use crate::network_address::NetworkAddress;

#[cw_serde]
pub enum ExecuteMsg {
    HandleCallMessage {
        from: NetworkAddress,
        data: Vec<u8>,
        protocols: Vec<String>,
    },
}
