package icon.ibc.structs.messages;

public class MsgPacketAcknowledgement {
    private byte[] packet;
    private byte[] acknowledgement;
    private byte[] proof;
    private byte[] proofHeight;

    public byte[] getPacket() {
        return packet;
    }

    public void setPacket(byte[] packet) {
        this.packet = packet;
    }

    public byte[] getAcknowledgement() {
        return acknowledgement;
    }

    public void setAcknowledgement(byte[] acknowledgement) {
        this.acknowledgement = acknowledgement;
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