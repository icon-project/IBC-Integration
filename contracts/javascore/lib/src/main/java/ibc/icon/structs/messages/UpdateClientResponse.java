package ibc.icon.structs.messages;

public class UpdateClientResponse {
    public UpdateClientResponse(byte[] clientStateCommitment,
            ConsensusStateUpdate update, boolean ok) {
        this.clientStateCommitment = clientStateCommitment;
        this.update = update;
        this.ok = ok;
    }

    public byte[] clientStateCommitment;
    public ConsensusStateUpdate update;
    public boolean ok;
}
