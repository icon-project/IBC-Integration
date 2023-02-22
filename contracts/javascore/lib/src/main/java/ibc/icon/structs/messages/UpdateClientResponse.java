package ibc.icon.structs.messages;

public class UpdateClientResponse {
    public UpdateClientResponse(byte[] clientStateCommitment,
            ConsensusStateUpdate[] updates, boolean ok) {
        this.clientStateCommitment = clientStateCommitment;
        this.updates = updates;
        this.ok = ok;
    }

    public byte[] clientStateCommitment;
    public ConsensusStateUpdate[] updates;
    public boolean ok;
}
