//! Message definitions for all ICS4 domain types: channel open & close handshake datagrams, as well
//! as packets.

// use crate::ibc::core::ics04_channel::msgs::acknowledgement::MsgAcknowledgement;
// use crate::ibc::core::ics04_channel::msgs::chan_close_confirm::MsgChannelCloseConfirm;
// use crate::ibc::core::ics04_channel::msgs::chan_close_init::MsgChannelCloseInit;
// use crate::ibc::core::ics04_channel::msgs::chan_open_ack::MsgChannelOpenAck;
// use crate::ibc::core::ics04_channel::msgs::chan_open_confirm::MsgChannelOpenConfirm;
// use crate::ibc::core::ics04_channel::msgs::chan_open_init::MsgChannelOpenInit;
// use crate::ibc::core::ics04_channel::msgs::chan_open_try::MsgChannelOpenTry;
// use crate::ibc::core::ics04_channel::msgs::recv_packet::MsgRecvPacket;
// use crate::ibc::core::ics04_channel::msgs::timeout::MsgTimeout;
// use crate::ibc::core::ics04_channel::msgs::timeout_on_close::MsgTimeoutOnClose;

// // Opening handshake messages.
// pub mod chan_open_ack;
// pub mod chan_open_confirm;
// pub mod chan_open_init;
// pub mod chan_open_try;

// // Closing handshake messages.
// pub mod chan_close_confirm;
// pub mod chan_close_init;

// // Packet specific messages.
pub mod acknowledgement;
// pub mod recv_packet;
// pub mod timeout;
// pub mod timeout_on_close;

// /// Enumeration of all possible messages that the ICS4 protocol processes.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum ChannelMsg {
//     OpenInit(MsgChannelOpenInit),
//     OpenTry(MsgChannelOpenTry),
//     OpenAck(MsgChannelOpenAck),
//     OpenConfirm(MsgChannelOpenConfirm),
//     CloseInit(MsgChannelCloseInit),
//     CloseConfirm(MsgChannelCloseConfirm),
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum PacketMsg {
//     Recv(MsgRecvPacket),
//     Ack(MsgAcknowledgement),
//     Timeout(MsgTimeout),
//     TimeoutOnClose(MsgTimeoutOnClose),
// }
