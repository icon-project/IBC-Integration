package ibc.icon.structs.messages;

public class MsgChannelCloseConfirm {
    private String portId;
    private String channelId;
    private byte[] proofInit;
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

    public byte[] getProofInit() {
        return proofInit;
    }

    public void setProofInit(byte[] proofInit) {
        this.proofInit = proofInit;
    }

    public byte[] getProofHeight() {
        return proofHeight;
    }

    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }

}