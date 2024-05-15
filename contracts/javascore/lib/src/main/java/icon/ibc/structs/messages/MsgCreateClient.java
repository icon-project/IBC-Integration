package icon.ibc.structs.messages;

public class MsgCreateClient {
    private String clientType;
    private byte[] clientState;
    private byte[] consensusState;
    private int btpNetworkId;

    public String getClientType() {
        return clientType;
    }

    public void setClientType(String clientType) {
        this.clientType = clientType;
    }

    public byte[] getClientState() {
        return clientState;
    }

    public void setClientState(byte[] clientState) {
        this.clientState = clientState;
    }

    public byte[] getConsensusState() {
        return consensusState;
    }

    public void setConsensusState(byte[] consensusState) {
        this.consensusState = consensusState;
    }

    public int getBtpNetworkId() {
        return btpNetworkId;
    }

    public void setBtpNetworkId(int btpNetworkId) {
        this.btpNetworkId = btpNetworkId;
    }
}
