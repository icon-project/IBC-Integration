package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;

public class MsgPacketAcknowledgement {
    public Packet packet;
    public byte[] acknowledgement;
    public byte[] proof;
    public Height proofHeight;
}