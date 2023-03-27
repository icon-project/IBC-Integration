package ibc.icon.structs.messages;

public class UpdateClientResponse {
    // current client state
    private byte[] clientStateCommitment;
    // commitment for updated consensusState
    private byte[] consensusStateCommitment;
    // updated height
    private byte[] height;

    

    public UpdateClientResponse(byte[] clientStateCommitment, byte[] consensusStateCommitment, byte[] height) {
        this.clientStateCommitment = clientStateCommitment;
        this.consensusStateCommitment = consensusStateCommitment;
        this.height = height;
    }

    public byte[] getClientStateCommitment() {
        return clientStateCommitment;
    }

    public void setClientStateCommitment(byte[] clientStateCommitment) {
        this.clientStateCommitment = clientStateCommitment;
    }

    public byte[] getConsensusStateCommitment() {
        return consensusStateCommitment;
    }

    public void setConsensusStateCommitment(byte[] consensusStateCommitment) {
        this.consensusStateCommitment = consensusStateCommitment;
    }

    public byte[] getHeight() {
        return height;
    }

    public void setHeight(byte[] height) {
        this.height = height;
    }
}
