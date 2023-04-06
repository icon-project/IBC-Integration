package ibc.icon.structs.messages;

public class MsgChannelOpenConfirm {
    private String portId;
    private String channelId;
    private byte[] proofAck;
    private byte[] proofHeight;

    public String getPortId() {
        return portId;
    }

    public void setPortId(String portId) {
        this.portId = portId;
    }

    public String getChannelId() {
        return channelId;
    }

    public void setChannelId(String channelId) {
        this.channelId = channelId;
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