use ibc::core::ics04_channel::Version;

use super::*;

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

// Creates OpenInitChannel IBC Event
pub fn create_open_init_channel_event(
    channel_id: &ChannelId,
    port_id_on_a: &IbcPortId,
    port_id_on_b: &IbcPortId,
    connection_hops_on_a: &IbcConnectionId,
    version_proposal: &Version,
) -> Event {
    Event::new(IbcEventType::OpenInitChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_a.as_str())
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.as_str())
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, port_id_on_b.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_hops_on_a.as_str())
        .add_attribute(VERSION_ATTRIBUTE_KEY, version_proposal.as_str())
}

// Creates OpenInitChannel IBC Event
pub fn create_open_try_channel_event(
    channel_id: &str,
    port_id_on_b: &str,
    port_id_on_a: &str,
    chan_id_on_a: &str,
    connection_hops_on_b: &str,
    version_supported_on_a: &str,
) -> Event {
    Event::new(IbcEventType::OpenTryChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id)
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, port_id_on_a)
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_a)
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_hops_on_b)
        .add_attribute(VERSION_ATTRIBUTE_KEY, version_supported_on_a)
}

// Creates OpenAckChannel  IBC Event
pub fn create_open_ack_channel_event(
    port_id_on_a: &str,
    chan_id_on_a: &str,
    port_id_on_b: &str,
    chan_id_on_b: &str,
    connection_id: &str,
) -> Event {
    Event::new(IbcEventType::OpenAckChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_a)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_a)
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_b)
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id)
}

// Creates OpenConfirmChannel IBC Event
pub fn create_open_confirm_channel_event(
    port_id_on_b: &str,
    chan_id_on_b: &str,
    port_id_on_a: &str,
    chan_id_on_a: &str,
    connection_id: &str,
) -> Event {
    Event::new(IbcEventType::OpenConfirmChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_b)
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, port_id_on_a)
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_a)
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id)
}

// Event for created channel id
pub fn create_channel_id_generated_event(channel_id: ChannelId) -> Event {
    Event::new("channel_id_created").add_attribute(
        CHANNEL_ID_ATTRIBUTE_KEY,
        channel_id.ibc_channel_id().as_str(),
    )
}

// Creates SendPacket event
pub fn create_send_packet_event(
    packet: Packet,
    channel_order: &Order,
    dst_connection_id: &IbcConnectionId,
) -> Result<Event, ContractError> {
    let data = std::str::from_utf8(&packet.data).map_err(|_| ContractError::IbcChannelError {
        error: ChannelError::NonUtf8PacketData,
    })?;
    let hex_data = hex::encode(&packet.data);

    Ok(Event::new(IbcEventType::SendPacket.as_str())
        .add_attribute(PKT_DATA_ATTRIBUTE_KEY, data)
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex_data)
        .add_attribute(
            PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY,
            packet.timeout_height_on_b.to_event_attribute_value(),
        )
        .add_attribute(
            PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY,
            packet.timeout_timestamp_on_b.nanoseconds().to_string(),
        )
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.seq_on_a.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.port_id_on_a.as_str())
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.chan_id_on_a.as_str())
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order.as_str())
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id.as_str()))
}

// Creates WriteAcknowledgement event
pub fn create_write_ack_event(
    packet: CwPacket,
    channel_order: &str,
    dst_connection_id: &str,
) -> Result<Event, ContractError> {
    let data = std::str::from_utf8(&packet.data).map_err(|_| ContractError::IbcChannelError {
        error: ChannelError::NonUtf8PacketData,
    })?;
    let hex_data = hex::encode(&packet.data);

    let timeout_height = Height::new(
        packet.timeout.block().unwrap().revision,
        packet.timeout.block().unwrap().height,
    )
    .map_err(|error| ContractError::IbcClientError { error })?;

    Ok(Event::new(IbcEventType::WriteAck.as_str())
        .add_attribute(PKT_DATA_ATTRIBUTE_KEY, data)
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex_data)
        .add_attribute(PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY, timeout_height)
        .add_attribute(
            PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY,
            packet.timeout.timestamp().unwrap().nanos().to_string(),
        )
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.sequence.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.src.port_id)
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.src.channel_id)
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.dest.port_id)
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.dest.channel_id)
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order)
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id))
}

// Creates AcknowledgePacket event
pub fn create_ack_packet_event(
    // packet: Packet,
    port_id: &str,
    chan_id: &str,
    seq_on_a: &str,
    dst_port_id: &str,
    dst_chan_id: &str,
    timeout_height_on_b: &str,
    timeout_timestamp_on_b: &str,
    channel_order: &str,
    dst_connection_id: &str,
) -> Event {
    Event::new(IbcEventType::AckPacket.as_str())
        .add_attribute(PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY, timeout_height_on_b)
        .add_attribute(PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY, timeout_timestamp_on_b)
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, seq_on_a)
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, port_id)
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, chan_id)
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, dst_port_id)
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, dst_chan_id)
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order)
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id)
}

// Creates TimeoutPacket event
pub fn create_packet_timeout_event(packet: Packet, channel_order: &Order) -> Event {
    Event::new(IbcEventType::Timeout.as_str())
        .add_attribute(
            PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY,
            packet.timeout_height_on_b.to_event_attribute_value(),
        )
        .add_attribute(
            PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY,
            packet.timeout_timestamp_on_b.nanoseconds().to_string(),
        )
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.seq_on_a.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.port_id_on_a.as_str())
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.chan_id_on_a.as_str())
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order.as_str())
}

// Creates CloseInitChannel event
pub fn create_close_init_channel_event(port_id: &str, channel_id: &str) -> Event {
    Event::new(IbcEventType::CloseInitChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id)
}

// Creates CloseConfirmChannel event
pub fn create_close_confirm_channel_event(port_id_on_b: &str, chan_id_on_b: &str) -> Event {
    Event::new(IbcEventType::CloseConfirmChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_b)
}

pub fn create_recieve_packet_event(
    // packet: Packet,
    port_id: &str,
    chan_id: &str,
    seq_on_a: &str,
    dst_port_id: &str,
    dst_chan_id: &str,
    timeout_height_on_b: &str,
    timeout_timestamp_on_b: &str,
    channel_order: &str,
    dst_connection_id: &str,
) -> Event {
    Event::new(IbcEventType::ReceivePacket.as_str())
        .add_attribute(PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY, timeout_height_on_b)
        .add_attribute(PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY, timeout_timestamp_on_b)
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, seq_on_a)
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, port_id)
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, chan_id)
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, dst_port_id)
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, dst_chan_id)
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order)
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id)
}
