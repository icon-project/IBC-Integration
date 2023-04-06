package ibc.icon.structs.messages;

public class MsgChannelOpenInit {
    private String portId;
    private byte[] channel;

    public String getPortId() {
        return portId;
    }

    public void setPortId(String portId) {
        this.portId = portId;
    }

    public byte[] getChannel() {
        return channel;
    }

    public void setChannel(byte[] channel) {
        this.channel = channel;
    }
}