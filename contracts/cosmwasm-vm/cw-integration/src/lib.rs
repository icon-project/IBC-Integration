use strum_macros::EnumString;

#[derive(Debug, Eq, PartialEq, EnumString, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TestSteps {
    CreateClient,
    UpdateClient,
    ConnOpenTry,
    ConnOpenInit,
    ConnOpenConfirm,
    ConnOpenAck,
    ChannelOpenInit,
    ChannelOpenTry,
    ChannelOpenConfirm,
    ChannelOpenAck,
    ChannelCloseInit,
    ChannelCloseConfirm,
    ReceivePacket,
}
