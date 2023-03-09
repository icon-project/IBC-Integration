package ibc.icon.structs.messages;

import icon.proto.core.client.Height;

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

    public byte[] getHeightRaw() {
        return height;
    }

    public Height getHeight() {
        return Height.decode(height);
    }

    public void setHeight(byte[] height) {
        this.height = height;
    }

}
