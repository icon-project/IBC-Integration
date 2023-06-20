use common::ibc::core::ics04_channel::timeout::TimeoutHeight;

use debug_print::debug_println;

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

/// This function creates an event for an open init channel.
///
/// Arguments:
///
/// * `channel_id`: The unique identifier of the channel being opened.
/// * `port_id_on_a`: The identifier of the IBC port on one end of the channel being created.
/// * `port_id_on_b`: `port_id_on_b` is the identifier of the port on the counterparty chain that is
/// being connected to through the channel being opened.
/// * `connection_hops_on_a`: `connection_hops_on_a` is a reference to a vector of `IbcConnectionId`
/// which represents the connection identifiers of the connections that the channel initiator (on chain
/// A) traversed in order to reach the connection end that initiated the channel opening handshake.
/// * `version_proposal`: The proposed version for the channel.
///
/// Returns:
///
/// A new `Event` object with attributes related to an open init channel event in an inter-blockchain
/// communication (IBC) protocol.
pub fn create_open_init_channel_event(
    channel_id: &str,
    port_id_on_a: &str,
    port_id_on_b: &str,
    connection_hops_on_a: &str,
    version_proposal: &str,
) -> Event {
    Event::new(IbcEventType::OpenInitChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_a)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id)
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_hops_on_a)
        .add_attribute(VERSION_ATTRIBUTE_KEY, version_proposal)
}

/// This function creates an event for an open try channel operation.
///
/// Arguments:
///
/// * `channel_id`: The unique identifier of the channel being opened.
/// * `port_id_on_b`: The identifier of the port on chain B that the channel is associated with.
/// * `port_id_on_a`: `port_id_on_a` is a string representing the identifier of the port on the local
/// chain (chain A) that the channel is associated with.
/// * `chan_id_on_a`: `chan_id_on_a` is the channel identifier on the counterparty chain A.
/// * `connection_hops_on_b`: `connection_hops_on_b` is a string representing the list of connection
/// identifiers that the channel goes through on chain B, separated by a slash ("/").
/// * `version_supported_on_a`: `version_supported_on_a` is a string representing the version of the
/// protocol supported by the channel initiator on chain A.
///
/// Returns:
///
/// A new `Event` object with attributes added to it.
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

/// This function creates an event for an acknowledgement of an open channel..
///
/// Arguments:
///
/// * `port_id_on_a`: The identifier of the port on chain A that the channel was opened on.
/// * `chan_id_on_a`: The channel identifier on the first chain (chain A).
/// * `port_id_on_b`: `port_id_on_b` is a string parameter representing the identifier of the port on
/// the counterparty chain that is associated with the channel being acknowledged.
/// * `chan_id_on_b`: `chan_id_on_b` is a string variable representing the channel identifier on the
/// counterparty chain.
/// * `connection_id`: The ID of the connection associated with the channel being acknowledged.
///
/// Returns:
///
/// A new `Event` object is being returned, which represents an `OpenAckChannel` event in the IBC
/// protocol. The event contains attributes such as the port and channel IDs for both ends of the
/// channel, as well as the connection ID.
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

/// This function creates an event for confirming the opening of a channel.
///
/// Arguments:
///
/// * `port_id_on_b`: The identifier of the port on the local chain where the channel was opened.
/// * `chan_id_on_b`: The channel identifier on chain B.
/// * `port_id_on_a`: The identifier of the port on chain A that the channel is associated with.
/// * `chan_id_on_a`: `chan_id_on_a` is a string representing the channel identifier on the counterparty
/// chain A.
/// * `connection_id`: The unique identifier of the connection associated with the channel being
/// confirmed.
///
/// Returns:
///
/// A new `Event` object with attributes related to an `OpenConfirmChannel` event in the
/// Inter-Blockchain Communication (IBC) protocol.
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

/// This function creates an event for sending a packet in an IBC channel.
///
/// Arguments:
///
/// * `packet`: The packet parameter is of type Packet, which represents a message that is being sent
/// over an IBC channel.
/// * `channel_order`: The channel ordering specifies the order in which packets are sent and received
/// on a channel. It can be either "unordered" or "ordered".
/// * `dst_connection_id`: The destination connection ID is the unique identifier of the connection on
/// the destination chain to which the packet is being sent. It is used to ensure that the packet is
/// sent to the correct destination.
///
/// Returns:
///
/// a `Result` with either an `Event` or a `ContractError`.
pub fn create_send_packet_event(
    packet: Packet,
    channel_order: &Order,
    dst_connection_id: &IbcConnectionId,
) -> Result<Event, ContractError> {
    let hex_data = hex::encode(&packet.data);

    debug_println!("the hex data is {:?}", hex_data);
    Ok(Event::new(IbcEventType::SendPacket.as_str())
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex_data)
        .add_attribute(
            PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY,
            packet.timeout_height_on_b.to_event_attribute_value(),
        )
        .add_attribute(
            PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY,
            packet.timeout_timestamp_on_b.nanoseconds().to_string(),
        )
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.sequence.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.port_id_on_a.as_str())
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.chan_id_on_a.as_str())
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.chan_id_on_b.as_str())
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order.as_str())
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id.as_str()))
}

/// This function creates and returns an event object with attributes related to an acknowledgement of a
/// written IBC packet.
///
/// Arguments:
///
/// * `packet`: The `packet` parameter is an `IbcPacket` struct that represents an inter-blockchain
/// communication packet. It contains information such as the source and destination ports and channels,
/// the packet data, and the timeout height and timestamp.
/// * `channel_order`: The ordering of the channel, which can be either "unordered" or "ordered".
/// * `dst_connection_id`: `dst_connection_id` is a string representing the identifier of the
/// destination connection for the IBC packet.
///
/// Returns:
///
/// a `Result` with either an `Event` or a `ContractError`.
pub fn create_write_ack_event(
    packet: CwPacket,
    channel_order: &str,
    dst_connection_id: &str,
    acknowledgement: &[u8],
) -> Result<Event, ContractError> {
    let height = Height::new(
        packet.timeout.block().unwrap().revision,
        packet.timeout.block().unwrap().height,
    )
    .map_err(|error| ContractError::IbcClientError { error })?;

    let timestamp = match packet.timeout.timestamp() {
        Some(t) => t.to_string(),
        None => 0.to_string(),
    };

    let timeout_height = TimeoutHeight::At(height);

    Ok(Event::new(IbcEventType::WriteAck.as_str())
        .add_attribute(PKT_ACK_HEX_ATTRIBUTE_KEY, hex::encode(acknowledgement))
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex::encode(packet.data))
        .add_attribute(
            PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY,
            timeout_height.to_event_attribute_value(),
        )
        .add_attribute(PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY, timestamp)
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.sequence.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.src.port_id)
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.src.channel_id)
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.dest.port_id)
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.dest.channel_id)
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order)
        .add_attribute(PKT_CONNECTION_ID_ATTRIBUTE_KEY, dst_connection_id))
}

/// The function creates an AckPacket event with various attributes in Rust.
///
/// Arguments:
///
/// * `port_id`: The identifier of the source port of the packet.
/// * `chan_id`: The channel identifier of the source channel.
/// * `seq_on_a`: The seq_on_a number of the packet on the sending chain (chain A).
/// * `dst_port_id`: The identifier of the destination port for the packet.
/// * `dst_chan_id`: The `dst_chan_id` parameter is a string representing the identifier of the
/// destination channel for the acknowledgement packet.
/// * `timeout_height_on_b`: The timeout height on the receiving chain for the packet being
/// acknowledged.
/// * `timeout_timestamp_on_b`: `timeout_timestamp_on_b` is a string representing the timestamp at which
/// the packet will time out on the receiving chain. This is used in the creation of an acknowledgement
/// packet event in the IBC protocol.
/// * `channel_order`: The ordering of packets in the channel, which can be either "unordered" or
/// "ordered".
/// * `dst_connection_id`: The identifier of the destination connection associated with the packet being
/// acknowledged.
///
/// Returns:
///
/// an `Event` object.
pub fn create_ack_packet_event(
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

/// This function creates an event for a packet timeout with various attributes.
///
/// Arguments:
///
/// * `packet`: The packet parameter is an object of type Packet, which contains information about a
/// packet being sent between two channels in the IBC protocol.
/// * `channel_order`: The `channel_order` parameter is a reference to an `Order` enum that specifies
/// the ordering of packets in a channel. It can be either `Order::Unordered` or `Order::Ordered`.
///
/// Returns:
///
/// an `Event` object.
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
        .add_attribute(PKT_SEQ_ATTRIBUTE_KEY, packet.sequence.to_string())
        .add_attribute(PKT_SRC_PORT_ATTRIBUTE_KEY, packet.port_id_on_a.as_str())
        .add_attribute(PKT_SRC_CHANNEL_ATTRIBUTE_KEY, packet.chan_id_on_a.as_str())
        .add_attribute(PKT_DST_PORT_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_DST_CHANNEL_ATTRIBUTE_KEY, packet.port_id_on_b.as_str())
        .add_attribute(PKT_CHANNEL_ORDERING_ATTRIBUTE_KEY, channel_order.as_str())
}

/// This function creates a new event for closing an initialized channel in the Inter-Blockchain
/// Communication protocol, with the given port and channel IDs as attributes.
///
/// Arguments:
///
/// * `port_id`: The `port_id` parameter is a string that represents the identifier of the port
/// associated with the channel being closed. In the context of the Inter-Blockchain Communication (IBC)
/// protocol, a port is a module that provides access to a specific blockchain network.
/// * `channel_id`: The `channel_id` parameter is a string that represents the unique identifier of a
/// channel in the Inter-Blockchain Communication (IBC) protocol. It is used to identify a specific
/// channel between two connected blockchains.
///
/// Returns:
///
/// an instance of the `Event` struct, which represents an event that can be emitted by the IBC module.
/// The event being created is of type `CloseInitChannel`, which indicates that an initial channel
/// handshake has been closed. The event includes attributes for the `port_id` and `channel_id`
/// associated with the closed channel.
pub fn create_close_init_channel_event(
    port_id: &str,
    channel_id: &str,
    channel_end: ChannelEnd,
) -> Event {
    Event::new(IbcEventType::CloseInitChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id)
        .add_attribute(
            COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY,
            channel_end.counterparty().channel_id().unwrap().to_string(),
        )
        .add_attribute(
            COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY,
            channel_end.counterparty().port_id().to_string(),
        )
}

/// This function creates an event for confirming the closure of a channel.
///
/// Arguments:
///
/// * `port_id_on_b`: The `port_id_on_b` parameter is a string that represents the identifier of the
/// port on the counterparty chain that the channel being closed is associated with.
/// * `chan_id_on_b`: `chan_id_on_b` is a string variable that represents the channel identifier on the
/// counterparty chain. It is used as an input parameter to create a new `CloseConfirmChannel` event in
/// the `create_close_confirm_channel_event` function.
///
/// Returns:
///
/// A function is being defined that returns an `Event` object. The `Event` object is created with the
/// type `CloseConfirmChannel` from the `IbcEventType` enum. The `Event` object also has two attributes
/// added to it: `port_id_on_b` and `chan_id_on_b`, which are passed as arguments to the function.
pub fn create_close_confirm_channel_event(
    port_id_on_b: &str,
    chan_id_on_b: &str,
    channel_end: ChannelEnd,
) -> Event {
    Event::new(IbcEventType::CloseConfirmChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, port_id_on_b)
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, chan_id_on_b)
        .add_attribute(
            COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY,
            channel_end.counterparty().channel_id().unwrap().to_string(),
        )
        .add_attribute(
            COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY,
            channel_end.counterparty().port_id().to_string(),
        )
}

/// This function creates an event for receiving a packet in an inter-blockchain communication protocol.
///
/// Arguments:
///
/// * `port_id`: The identifier of the source port of the packet.
/// * `chan_id`: The identifier of the source channel where the packet was sent from.
/// * `seq_on_a`: The seq_on_a number of the packet on the sending chain.
/// * `dst_port_id`: The identifier of the destination port for the packet.
/// * `dst_chan_id`: The `dst_chan_id` parameter is the identifier of the channel on the destination
/// chain where the packet is being sent to.
/// * `timeout_height_on_b`: The timeout height on the receiving chain B, after which the packet will be
/// considered timed out and can be retransmitted.
/// * `timeout_timestamp_on_b`: `timeout_timestamp_on_b` is a string representing the timestamp at which
/// the packet will timeout on the receiving chain. This is used in the context of inter-blockchain
/// communication (IBC) to ensure that packets are not stuck in transit indefinitely.
/// * `channel_order`: The ordering of packets in the channel, which can be either "unordered" or
/// "ordered".
/// * `dst_connection_id`: The identifier of the connection on the destination chain where the packet is
/// being sent to.
///
/// Returns:
///
/// A new `Event` object with attributes related to a receive packet event in an IBC protocol
/// implementation.
pub fn create_recieve_packet_event(
    data: &[u8],
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
        .add_attribute(PKT_DATA_HEX_ATTRIBUTE_KEY, hex::encode(data))
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
