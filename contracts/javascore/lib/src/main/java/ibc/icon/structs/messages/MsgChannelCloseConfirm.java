package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class MsgChannelCloseConfirm {
    public String portId;
    public String channelId;
    public byte[] proofInit;
    public Height proofHeight;
}