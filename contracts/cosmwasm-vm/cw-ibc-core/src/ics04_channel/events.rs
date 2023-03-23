use super::*;

pub const CHANNEL_ID_ATTRIBUTE_KEY: &str = "channel_id";

// Makes OpenInitChannel IBC Event
pub fn make_open_init_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenInit) -> Event {
    Event::new(IbcEventType::OpenInitChannel.as_str())
        .add_attribute("port_id", msg.port_id_on_a.as_str())
        .add_attribute("channel_id", channel_id.as_str())
        .add_attribute("counterparty_port_id", msg.port_id_on_b.as_str())
        .add_attribute("connection_id", msg.connection_hops_on_a[0].as_str())
        .add_attribute("version", msg.version_proposal.as_str())
}

// Makes OpenInitChannel IBC Event
pub fn make_open_try_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenTry) -> Event {
    Event::new(IbcEventType::OpenTryChannel.as_str())
        .add_attribute("port_id", msg.port_id_on_b.as_str())
        .add_attribute("channel_id", channel_id.as_str())
        .add_attribute("counterparty_port_id", msg.port_id_on_a.as_str())
        .add_attribute("counterparty_channel_id", msg.chan_id_on_a.as_str())
        .add_attribute("connection_id", msg.connection_hops_on_b[0].as_str())
        .add_attribute("version", msg.version_supported_on_a.as_str())
}

// Makes OpenAckChannel  IBC Event
pub fn make_open_ack_channel_event(msg: &MsgChannelOpenAck) -> Event {
    Event::new(IbcEventType::OpenAckChannel.as_str())
        .add_attribute("port_id", msg.port_id_on_a.as_str())
        .add_attribute("channel_id", msg.chan_id_on_a.as_str())
        .add_attribute("counterparty_port_id", "")
        .add_attribute("counterparty_channel_id", msg.chan_id_on_b.as_str())
        .add_attribute("connection_id", "")
}

// Makes OpenConfirmChannel IBC Event
pub fn make_open_confirm_channel_event(msg: &MsgChannelOpenConfirm) -> Event {
    Event::new(IbcEventType::OpenAckChannel.as_str())
        .add_attribute("port_id", msg.port_id_on_b.as_str())
        .add_attribute("channel_id", msg.chan_id_on_b.as_str())
        .add_attribute("counterparty_port_id", "")
        .add_attribute("counterparty_channel_id", "")
        .add_attribute("connection_id", "")
}

// Event for created channel id
pub fn event_channel_id_generated(channel_id: ChannelId) -> Event {
    Event::new("channel_id_created")
        .add_attribute(CHANNEL_ID_ATTRIBUTE_KEY, channel_id.ibc_channel_id().as_str())
}
