package ibc.icon.structs.proto.core.channel;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;

// Counterparty defines a channel end counterparty
public class Counterparty {
    // port on the counterparty chain which owns the other end of the channel.
    public String portId;
    // channel end on the counterparty chain
    public String channelId;

    public byte[] encode() {
        return ByteUtil.join(
                Proto.encode(1, portId),
                Proto.encode(2, channelId));
    }

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
