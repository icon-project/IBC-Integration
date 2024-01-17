use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_xcall_lib::network_address::NetId;

#[cw_serde]
pub enum ExecuteMsg {
    SetFee {
        network_id: NetId,
        message_fee: u128,
        response_fee: u128,
    },
    SendMessage {
        to: NetId,
        sn: i64,
        msg: Vec<u8>,
    },

    RecvMessage {
        src_network: NetId,
        conn_sn: u128,
        msg: String,
    },

    ClaimFees {},
    RevertMessage {
        sn: u128,
    },
    SetAdmin {
        address: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
/// This is a Rust enum representing different types of queries that can be made to the contract. Each
/// variant of the enum corresponds to a specific query and has a return type specified using the
/// `#[returns]` attribute.
pub enum QueryMsg {
    #[returns(u64)]
    GetFee { nid: NetId, response: bool },
    #[returns(bool)]
    GetReceipt { src_network: NetId, conn_sn: u128 },
    //return address of admin
    #[returns(Addr)]
    Admin {},
}

#[cw_serde]
pub struct MigrateMsg {}
