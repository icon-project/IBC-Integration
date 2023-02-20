package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class ConsensusStateUpdate {
    // commitment for updated consensusState
    public byte[] consensusStateCommitment;
    // updated height
    public Height height;
}
