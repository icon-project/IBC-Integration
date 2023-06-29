package ibc.icon.structs.messages;

public class MsgRequestTimeoutPacket {
    private byte[] packet;
    private byte[] proof;
    private byte[] proofHeight;

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

}