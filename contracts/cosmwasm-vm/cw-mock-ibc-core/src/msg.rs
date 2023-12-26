use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, IbcPacket};
use cw_common::cw_types::CwChannelConnectMsg;
use cw_common::hex_string::HexString;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendPacket {
        packet: HexString,
    },
    ReceivePacket {
        message: String,
    },
    RegisterXcall {
        address: Addr,
    },
    IbcConfig {
        msg: CwChannelConnectMsg,
    },
    WriteAcknowledgement {
        packet: IbcPacket,
        acknowledgement: HexString,
    },
}
