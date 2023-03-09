package ibc.icon.structs.messages;

public class UpdateClientResponse {
    private byte[] clientStateCommitment;
    private ConsensusStateUpdate update;
    private boolean ok;

    public UpdateClientResponse(byte[] clientStateCommitment,
            ConsensusStateUpdate update, boolean ok) {
        this.clientStateCommitment = clientStateCommitment;
        this.update = update;
        this.ok = ok;
    }

    public byte[] getClientStateCommitment() {
        return clientStateCommitment;
    }

    public void setClientStateCommitment(byte[] clientStateCommitment) {
        this.clientStateCommitment = clientStateCommitment;
    }

    public ConsensusStateUpdate getUpdate() {
        return update;
    }

    public void setUpdate(ConsensusStateUpdate update) {
        this.update = update;
    }

    public boolean isOk() {
        return ok;
    }

    public void setOk(boolean ok) {
        this.ok = ok;
    }

}
