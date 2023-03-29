package ibc.icon.structs.messages;

public class MsgUpdateClient {
    private String clientId;
    private byte[] clientMessage;

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public byte[] getClientMessage() {
        return clientMessage;
    }

    public void setClientMessage(byte[] clientMessage) {
        this.clientMessage = clientMessage;
    }

}
