use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_common::cw_types::CwChannelConnectMsg;
use cw_common::hex_string::HexString;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendPacket { packet: HexString },
    ReceivePacket { message: String },
    RegisterXcall { address: Addr },
    IbcConfig { msg: CwChannelConnectMsg },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    SequenceSend { port_id: String, channel_id: String },
}
