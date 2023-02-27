package ibc.icon.structs.proto.core.channel;

// IdentifiedChannel defines a channel with additional port and channel
// identifier fields.
public class IdentifiedChannel {
    // current state of the channel end
    public Channel.State state;
    // whether the channel is ordered or unordered
    public Channel.Order ordering;
    // counterparty channel end
    public Counterparty counterparty;
    // list of connection identifiers, in order, along which packets sent on
    // this channel will travel
    public String[] connectionHops;
    // opaque channel version, which is agreed upon during the handshake
    public String version;
    // port identifier
    public String portId;
    // channel identifier
    public String channelId;
}
