package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.channel.Channel;

public class MsgChannelOpenInit {
    public String portId;
    public Channel channel;
}