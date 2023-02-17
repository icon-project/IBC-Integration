package ibc.icon.structs.proto.core.channel;

public class Channel {

    // State defines if a channel is in one of the following states:
    // CLOSED, INIT, TRYOPEN, OPEN or UNINITIALIZED.
    enum State {
        // Default State
        STATE_UNINITIALIZED_UNSPECIFIED,
        // A channel has just started the opening handshake.
        STATE_INIT,
        // A channel has acknowledged the handshake step on the counterparty chain.
        STATE_TRYOPEN,
        // A channel has completed the handshake. Open channels are
        // ready to send and receive packets.
        STATE_OPEN,
        // A channel has been closed and can no longer be used to send or receive
        // packets.
        STATE_CLOSED
    }

    // Order defines if a channel is ORDERED or UNORDERED
    enum Order {
        // zero-value for channel ordering
        ORDER_NONE_UNSPECIFIED,
        // packets can be delivered in any order, which may differ from the order in
        // which they were sent.
        ORDER_UNORDERED,
        // packets are delivered exactly in the order which they were sent
        ORDER_ORDERED,
    }

    // Counterparty defines a channel end counterparty
    class Counterparty {
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

    // IdentifiedChannel defines a channel with additional port and channel
    // identifier fields.
    class IdentifiedChannel {
        // current state of the channel end
        public State state;
        // whether the channel is ordered or unordered
        public Order ordering;
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

    // current state of the channel end
    public String state;
    // whether the channel is ordered or unordered
    public String ordering;
    // counterparty channel end
    public Counterparty counterparty;
    // lis t of connection identifiers, in order, along which packets sent on
    // this channel will travel
    public String[] connectionHops;
    // opaque channel version, which is agreed upon during the handshake
    public String version;

    public State getState() {
        return State.valueOf(state);
    }

    public void setState(State state) {
        this.state = state.toString();
    }

    public Order getOrdering() {
        return Order.valueOf(ordering);
    }

    public void setOrdering(Order ordering) {
        this.ordering = ordering.toString();
    }

    public Counterparty getCounterparty() {
        return counterparty;
    }

    public void setCounterparty(Counterparty counterparty) {
        this.counterparty = counterparty;
    }

    public String[] getConnectionHops() {
        return connectionHops;
    }

    public void setConnectionHops(String[] connectionHops) {
        this.connectionHops = connectionHops;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(String version) {
        this.version = version;
    }
}
