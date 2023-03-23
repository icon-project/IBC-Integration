package ibc.icon.structs.messages;

public class ConsensusStateUpdate {
    public ConsensusStateUpdate(byte[] consensusStateCommitment, byte[] height) {
        this.consensusStateCommitment = consensusStateCommitment;
        this.height = height;
    }

    // commitment for updated consensusState
    private byte[] consensusStateCommitment;
    // updated height
    private byte[] height;

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
