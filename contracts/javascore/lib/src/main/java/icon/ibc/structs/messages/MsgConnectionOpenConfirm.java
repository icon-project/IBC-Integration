package icon.ibc.structs.messages;

public class MsgConnectionOpenConfirm {
    private String connectionId;
    private byte[] proofAck;
    private byte[] proofHeight;

    public String getConnectionId() {
        return connectionId;
    }

    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }

    public byte[] getProofAck() {
        return proofAck;
    }

    public void setProofAck(byte[] proofAck) {
        this.proofAck = proofAck;
    }

    public byte[] getProofHeight() {
        return proofHeight;
    }

    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }

}