package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class ConsensusStateUpdate {
    public ConsensusStateUpdate(byte[] consensusStateCommitment, Height height) {
        this.consensusStateCommitment = consensusStateCommitment;
        this.height = height;
    }

    // commitment for updated consensusState
    public byte[] consensusStateCommitment;
    // updated height
    public Height height;
}
