use super::*;

pub const CHANNEL_ID_ATTRIBUTE_KEY: &str = "channel_id";
pub const PORT_ID_ATTRIBUTE_KEY: &str = "port_id";
pub const COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY: &str = "counterparty_channel_id";
pub const COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY: &str = "counterparty_port_id";
pub const VERSION_ATTRIBUTE_KEY: &str = "version";

// Makes OpenInitChannel IBC Event
pub fn make_open_init_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenInit) -> Event {
    Event::new(IbcEventType::OpenInitChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_a.as_str())
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.as_str())
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_b.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, msg.connection_hops_on_a[0].as_str())
        .add_attribute(VERSION_ATTRIBUTE_KEY, msg.version_proposal.as_str())
}

// Makes OpenInitChannel IBC Event
pub fn make_open_try_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenTry) -> Event {
    Event::new(IbcEventType::OpenTryChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_b.as_str())
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.as_str())
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_a.as_str())
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, msg.chan_id_on_a.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, msg.connection_hops_on_b[0].as_str())
        .add_attribute(VERSION_ATTRIBUTE_KEY, msg.version_supported_on_a.as_str())
}

// Makes OpenAckChannel  IBC Event
pub fn make_open_ack_channel_event(msg: &MsgChannelOpenAck) -> Event {
    Event::new(IbcEventType::OpenAckChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_a.as_str())
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, msg.chan_id_on_a.as_str())
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, PortId::default().ibc_port_id().as_str())
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, msg.chan_id_on_b.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, ConnectionId::default().connection_id().as_str())
}

// Makes OpenConfirmChannel IBC Event
pub fn make_open_confirm_channel_event(msg: &MsgChannelOpenConfirm) -> Event {
    Event::new(IbcEventType::OpenConfirmChannel.as_str())
        .add_attribute(PORT_ID_ATTRIBUTE_KEY, msg.port_id_on_b.as_str())
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, msg.chan_id_on_b.as_str())
        .add_attribute(COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY, PortId::default().ibc_port_id().as_str())
        .add_attribute(COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY, ChannelId::default().ibc_channel_id().as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, ConnectionId::default().connection_id().as_str())
}

// Event for created channel id
pub fn event_channel_id_generated(channel_id: ChannelId) -> Event {
    Event::new("channel_id_created")
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.ibc_channel_id().as_str())
}
