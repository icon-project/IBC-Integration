package ibc.icon.structs.proto.core.channel;

// Counterparty defines a channel end counterparty
public class Counterparty {
    // port on the counterparty chain which owns the other end of the channel.
    public String portId;
    // channel end on the counterparty chain
    public String channelId;

    public String getPortId() {
        return portId;
    }

    public void setPortId(String portId) {
        this.portId = portId;
    }

    public String getChannelId() {
        return channelId;
    }

    public void setChannelId(String channelId) {
        this.channelId = channelId;
    }
}
