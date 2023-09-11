use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_xcall_lib::network_address::NetId;

#[cw_serde]
pub struct InstantiateMsg {
    pub ibc_host: Addr,
    pub port_id: String,
    pub xcall_address: Addr,
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SendMessage {
        to: NetId,
        sn: i64,
        msg: Vec<u8>,
    },
    SetFees {
        nid: NetId,
        packet_fee: u128,
        ack_fee: u128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    GetFee { nid: NetId, response: bool },
}
