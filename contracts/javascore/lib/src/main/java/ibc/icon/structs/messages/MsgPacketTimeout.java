package ibc.icon.structs.messages;

import java.math.BigInteger;

import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;

public class MsgPacketTimeout {
    private byte[] packet;
    private byte[] proof;
    private byte[] proofHeight;
    private BigInteger nextSequenceRecv;
    
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

    public BigInteger getNextSequenceRecv() {
        return nextSequenceRecv;
    }

    public void setNextSequenceRecv(BigInteger nextSequenceRecv) {
        this.nextSequenceRecv = nextSequenceRecv;
    }
}