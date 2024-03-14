package ibc.icon.structs.messages;

import icon.proto.core.channel.Channel.Counterparty;
public class MsgOnChanOpenInit {
    private String[] connectionHops;
    private String portId;
    private String channelId;
    private Counterparty counterParty;
    private String version;

    public String[] getConnectionHops() {
        return connectionHops;
    }

    public void setConnectionHops(String[] connectionHops) {
        this.connectionHops = connectionHops;
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

    public Counterparty getCounterParty() {
        return counterParty;
    }

    public void setCounterParty(Counterparty counterParty) {
        this.counterParty = counterParty;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(String version) {
        this.version = version;
    }
}
