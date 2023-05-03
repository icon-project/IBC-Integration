use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

use crate::hex_string::HexString;

use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // Client Messages
    RegisterClient {
        client_type: String,
        client_address: Addr,
    },
    CreateClient {
        client_state: HexString,
        consensus_state: HexString,
        signer: HexString,
    },
    UpdateClient {
        client_id: String,
        header: HexString,
        signer: HexString,
    },
    // Not included in this version of ibc core
    UpgradeClient {},

    ClientMisbehaviour {},

    // Connection Messsages
    ConnectionOpenInit {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: HexString,
    },
    ConnectionOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry
        msg: HexString,
    },
    ConnectionOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck
        msg: HexString,
    },
    ConnectionOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm
        msg: HexString,
    },

    // Channel Messages
    ChannelOpenInit {
        // raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit
        msg: HexString,
    },
    ChannelOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry
        msg: HexString,
    },
    ChannelOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck
        msg: HexString,
    },
    ChannelOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm
        msg: HexString,
    },
    ChannelCloseInit {
        port_id_on_a: String,
        chan_id_on_a: String,
        signer: HexString,
    },
    ChannelCloseConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelCloseConfirm
        msg: HexString,
    },

    // Packet Messages
    SendPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::Packet
        packet: HexString,
    },
    ReceivePacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgRecvPacket
        msg: HexString,
    },
    AcknowledgementPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgAcknowledgement
        msg: HexString,
    },
    RequestTimeout {},
    Timeout {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeout
        msg: HexString,
    },
    TimeoutOnClose {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeoutOnClose
        msg: HexString,
    },

    // Storage Messages
    BindPort {
        port_id: String,
        address: String,
    },
    SetExpectedTimePerBlock {
        block_time: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    SequenceSend { port_id: String, channel_id: String },
}
