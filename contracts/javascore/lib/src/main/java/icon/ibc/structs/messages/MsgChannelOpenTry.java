package icon.ibc.structs.messages;

public class MsgChannelOpenTry {
    private String portId;
    private byte[] channel;
    private String counterpartyVersion;
    private byte[] proofInit;
    private byte[] proofHeight;

    public String getPortId() {
        return portId;
    }

    public void setPortId(String portId) {
        this.portId = portId;
    }

    public byte[] getChannel() {
        return channel;
    }

    public void setChannel(byte[] channel) {
        this.channel = channel;
    }

    public String getCounterpartyVersion() {
        return counterpartyVersion;
    }

    public void setCounterpartyVersion(String counterpartyVersion) {
        this.counterpartyVersion = counterpartyVersion;
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