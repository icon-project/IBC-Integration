package icon.ibc.structs.messages;

import java.math.BigInteger;

public class MsgPacketTimeout {
    private byte[] packet;
    private byte[] proof;
    private byte[] proofHeight;
    private BigInteger nextSequenceRecv;

    public byte[] getPacket() {
        return packet;
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

    public byte[] getProofHeight() {
        return proofHeight;
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