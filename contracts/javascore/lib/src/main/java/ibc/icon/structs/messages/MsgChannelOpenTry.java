package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.client.Height;

public class MsgChannelOpenTry {
    public String portId;
    public String previousChannelId;
    public Channel channel;
    public String counterpartyVersion;
    public byte[] proofInit;
    public Height proofHeight;
}