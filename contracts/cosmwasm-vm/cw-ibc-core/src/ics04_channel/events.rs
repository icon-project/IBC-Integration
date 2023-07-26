use super::*;
use cw_common::raw_types::channel::RawPacket;

pub const CHANNEL_ID_ATTRIBUTE_KEY: &str = "channel_id";
pub const PORT_ID_ATTRIBUTE_KEY: &str = "port_id";
pub const COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY: &str = "counterparty_channel_id";
pub const COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY: &str = "counterparty_port_id";
pub const VERSION_ATTRIBUTE_KEY: &str = "version";

// Packet constants
pub const PKT_SEQ_ATTRIBUTE_KEY: &str = "packet_sequence";
pub const PKT_DATA_ATTRIBUTE_KEY: &str = "packet_data";
pub const PKT_DATA_HEX_ATTRIBUTE_KEY: &str = "packet_data_hex";
pub const PKT_SRC_PORT_ATTRIBUTE_KEY: &str = "packet_src_port";
pub const PKT_SRC_CHANNEL_ATTRIBUTE_KEY: &str = "packet_src_channel";
pub const PKT_DST_PORT_ATTRIBUTE_KEY: &str = "packet_dst_port";
pub const PKT_DST_CHANNEL_ATTRIBUTE_KEY: &str = "packet_dst_channel";
pub const PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY: &str = "packet_channel_ordering";
pub const PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY: &str = "packet_timeout_height";
pub const PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY: &str = "packet_timeout_timestamp";
pub const PKT_ACK_ATTRIBUTE_KEY: &str = "packet_ack";
pub const PKT_ACK_HEX_ATTRIBUTE_KEY: &str = "packet_ack_hex";
pub const PKT_CONNECTION_ID_ATTRIBUTE_KEY: &str = "packet_connection";

pub fn create_channel_event(
    event_type: IbcEventType,
    port_id: &str,
    channel_id: &str,
    channel: &ChannelEnd,
) -> Result<Event, ContractError> {
    let mut event = Event::new(event_type.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id)
        .add_attribute(
            COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY,
            channel.counterparty().port_id.as_str(),
        )
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, channel.connection_hops[0].as_str())
        .add_attribute(VERSION_ATTRIBUTE_KEY, channel.version().to_string());
    match event_type {
        IbcEventType::OpenInitChannel => Ok(event),
        IbcEventType::OpenTryChannel
        | IbcEventType::CloseInitChannel
        | IbcEventType::CloseConfirmChannel
        | IbcEventType::OpenAckChannel
        | IbcEventType::OpenConfirmChannel => {
            event = event.add_attribute(
                COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY,
                channel.counterparty().channel_id().unwrap().to_string(),
            );
            Ok(event)
        }

        _ => Err(ContractError::InvalidEventType {
            event: "Channel Event".to_string(),
            event_type: event_type.as_str().to_string(),
        }),
    }
}

/// This function creates an event with a "channel_id_created" tag and adds a channel ID attribute to
/// it.
///
/// Arguments:
///
/// * `channel_id`: The `channel_id` parameter is of type `ChannelId`, which is an identifier for an IBC
/// channel. It is used to uniquely identify a channel between two IBC-enabled blockchains.
///
/// Returns:
///
/// A new `Event` object with the name "channel_id_created" and an attribute with the key
/// `CHANNEL_ID_ATTRIBUTE_KEY` and the value of the `ibc_channel_id` of the `channel_id` parameter
/// converted to a string.
pub fn create_channel_id_generated_event(channel_id: ChannelId) -> Event {
    Event::new("channel_id_created").add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.as_str())
}

pub fn create_packet_event(
    event_type: IbcEventType,
    packet: RawPacket,
    channel_order: &Order,
    connection_id: &IbcConnectionId,
    ack: Option<Vec<u8>>,
) -> Result<Event, ContractError> {
    let timeout_height = packet
        .timeout_height
        .map(|h| format!("{}-{}", h.revision_number, h.revision_height))
        .unwrap_or("0-0".to_string());
    let hex_data = hex::encode(&packet.data);

    let mut event = Event::new(event_type.as_str())
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.sequence.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.source_port.as_str())
        .add_attribute(
            PKT_SRC_CHANNEL_ATTRIBUTE_KEY,
            packet.source_channel.as_str(),
        )
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.destination_port.as_str())
        .add_attribute(
            PKT_DST_CHANNEL_ATTRIBUTE_KEY,
            packet.destination_channel.as_str(),
        )
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order.as_str())
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY, timeout_height)
        .add_attribute(
            PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY,
            packet.timeout_timestamp.to_string(),
        )
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex_data);

    match event_type {
        IbcEventType::SendPacket
        | IbcEventType::ReceivePacket
        | IbcEventType::AckPacket
        | IbcEventType::Timeout => Ok(event),
        IbcEventType::WriteAck => {
            let hex_data = hex::encode(&packet.data);
            event = event
                .add_attribute(PKT_ACK_HEX_ATTRIBUTE_KEY, hex::encode(ack.unwrap()))
                .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex_data);
            Ok(event)
        }
        _ => Err(ContractError::InvalidEventType {
            event: "Packet Event".to_string(),
            event_type: event_type.as_str().to_string(),
        }),
    }
}
