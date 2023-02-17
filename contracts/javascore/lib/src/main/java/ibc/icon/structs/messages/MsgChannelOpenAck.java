package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class MsgChannelOpenAck {
    public String portId;
    public String channelId;
    public String counterpartyVersion;
    public String counterpartyChannelId;
    public byte[] proofTry;
    public Height proofHeight;
}
