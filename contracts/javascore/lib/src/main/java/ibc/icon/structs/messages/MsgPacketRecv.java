package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;

public class MsgPacketRecv {
    public Packet packet;
    public byte[] proof;
    public Height proofHeight;
}