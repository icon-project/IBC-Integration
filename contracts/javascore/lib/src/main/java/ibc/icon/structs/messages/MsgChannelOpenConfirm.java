package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class MsgChannelOpenConfirm {
    public String portId;
    public String channelId;
    public byte[] proofAck;
    public Height proofHeight;
}