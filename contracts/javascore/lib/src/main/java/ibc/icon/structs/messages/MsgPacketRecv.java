package ibc.icon.structs.messages;

import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;

public class MsgPacketRecv {
    private byte[] packet;
    private byte[] proof;
    private byte[] proofHeight;

    public byte[] getPacketRaw() {
        return packet;
    }

    public Packet getPacket() {
        return Packet.decode(packet);
    }

    public void setPacket(byte[] packet) {
        this.packet = packet;
    }

    public byte[] getProof() {
        return proof;
    }

    public void setProof(byte[] proof) {
        this.proof = proof;
    }

    public byte[] getProofHeightRaw() {
        return proofHeight;
    }

    public Height getProofHeight() {
        return Height.decode(proofHeight);
    }

    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }
}