package icon.ibc.structs.messages;

public class MsgConnectionOpenAck {
    private String connectionId;
    private byte[] clientStateBytes; // client state for chainA on chainB
    private byte[] version; // version that ChainB chose in ConnOpenTry
    private String counterpartyConnectionID;
    private byte[] proofTry; // proof that connectionEnd was added to ChainB state in ConnOpenTry
    private byte[] proofClient; // proof of client state on chainB for chainA
    private byte[] proofConsensus; // proof that chainB has stored ConsensusState of chainA on its client
    private byte[] proofHeight; // height that relayer conpublic classed proofTry
    private byte[] consensusHeight; // latest height of chainA that chainB has stored on its chainA client

    public String getConnectionId() {
        return connectionId;
    }
    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }
    public byte[] getClientStateBytes() {
        return clientStateBytes;
    }
    public void setClientStateBytes(byte[] clientStateBytes) {
        this.clientStateBytes = clientStateBytes;
    }
    public byte[] getVersion() {
        return version;
    }
    public void setVersion(byte[] version) {
        this.version = version;
    }
    public String getCounterpartyConnectionID() {
        return counterpartyConnectionID;
    }
    public void setCounterpartyConnectionID(String counterpartyConnectionID) {
        this.counterpartyConnectionID = counterpartyConnectionID;
    }
    public byte[] getProofTry() {
        return proofTry;
    }
    public void setProofTry(byte[] proofTry) {
        this.proofTry = proofTry;
    }
    public byte[] getProofClient() {
        return proofClient;
    }
    public void setProofClient(byte[] proofClient) {
        this.proofClient = proofClient;
    }
    public byte[] getProofConsensus() {
        return proofConsensus;
    }
    public void setProofConsensus(byte[] proofConsensus) {
        this.proofConsensus = proofConsensus;
    }
    public byte[] getProofHeight() {
        return proofHeight;
    }
    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }
    public byte[] getConsensusHeight() {
        return consensusHeight;
    }
    public void setConsensusHeight(byte[] consensusHeight) {
        this.consensusHeight = consensusHeight;
    }



}