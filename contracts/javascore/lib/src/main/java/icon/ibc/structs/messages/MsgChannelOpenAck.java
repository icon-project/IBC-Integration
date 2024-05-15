package icon.ibc.structs.messages;

public class MsgChannelOpenAck {
    private String portId;
    private String channelId;
    private String counterpartyVersion;
    private String counterpartyChannelId;
    private byte[] proofTry;
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

    public String getCounterpartyVersion() {
        return counterpartyVersion;
    }

    public void setCounterpartyVersion(String counterpartyVersion) {
        this.counterpartyVersion = counterpartyVersion;
    }

    public String getCounterpartyChannelId() {
        return counterpartyChannelId;
    }

    public void setCounterpartyChannelId(String counterpartyChannelId) {
        this.counterpartyChannelId = counterpartyChannelId;
    }

    public byte[] getProofTry() {
        return proofTry;
    }

    public void setProofTry(byte[] proofTry) {
        this.proofTry = proofTry;
    }

    public byte[] getProofHeight() {
        return proofHeight;
    }


    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }

}
