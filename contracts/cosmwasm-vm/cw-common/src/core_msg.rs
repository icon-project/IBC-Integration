use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::{hex_string::HexString, types::RelayAny};

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
        // client_state any
        client_state: RelayAny,
        // consensus state any
        consensus_state: RelayAny,
        signer: HexString,
    },
    UpdateClient {
        client_id: String,
        // signed header any
        header: RelayAny,
        signer: HexString,
    },
    // Not included in this version of ibc core
    UpgradeClient {},

    ClientMisbehaviour {},

    // Connection Messsages
    ConnectionOpenInit {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: RelayAny,
    },
    ConnectionOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry
        msg: RelayAny,
    },
    ConnectionOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck
        msg: RelayAny,
    },
    ConnectionOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm
        msg: RelayAny,
    },

    // Channel Messages
    ChannelOpenInit {
        // raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit
        msg: RelayAny,
    },
    ChannelOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry
        msg: RelayAny,
    },
    ChannelOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck
        msg: RelayAny,
    },
    ChannelOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm
        msg: RelayAny,
    },
    ChannelCloseInit {
        port_id_on_a: String,
        chan_id_on_a: String,
        signer: HexString,
    },
    ChannelCloseConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelCloseConfirm
        msg: RelayAny,
    },

    // Packet Messages
    SendPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::Packet
        packet: RelayAny,
    },
    SendPacketCommon {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::Packet
        packet: HexString,
    },
    ReceivePacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgRecvPacket
        msg: RelayAny,
    },
    AcknowledgementPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgAcknowledgement
        msg: RelayAny,
    },
    RequestTimeout {},
    Timeout {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeout
        msg: RelayAny,
    },
    TimeoutOnClose {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeoutOnClose
        msg: RelayAny,
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
