package ibc.icon.structs.proto.core.channel;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

import java.util.List;

public class Channel {

    // State defines if a channel is in one of the following states:
    // CLOSED, INIT, TRYOPEN, OPEN or UNINITIALIZED.
    public enum State {
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
    public enum Order {
        // zero-value for channel ordering
        ORDER_NONE_UNSPECIFIED,
        // packets can be delivered in any order, which may differ from the order in
        // which they were sent.
        ORDER_UNORDERED,
        // packets are delivered exactly in the order which they were sent
        ORDER_ORDERED,
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

    public static void writeObject(ObjectWriter writer, Channel obj) {
        obj.writeObject(writer);
    }

    public static Channel readObject(ObjectReader reader) {

        Channel obj = new Channel();
        reader.beginList();
        obj.state = reader.readString();
        obj.ordering = reader.readString();
        Counterparty counterparty = new Counterparty();
        counterparty.portId = reader.readString();
        counterparty.channelId = reader.readString();
        obj.counterparty = counterparty;

        reader.beginList();
        String[] connectionHops = null;
        List<String> connectionHopsList = new ArrayList<>();
        while (reader.hasNext()) {
            byte[] connectionHopElementBytes = reader.readNullable(byte[].class);
            if (connectionHopElementBytes != null) {
                ObjectReader connectionHopElementReader = Context.newByteArrayObjectReader("RLPn",
                        connectionHopElementBytes);
                connectionHopsList.add(connectionHopElementReader.read(String.class));
            }
        }

        connectionHops = new String[connectionHopsList.size()];
        for (int i = 0; i < connectionHopsList.size(); i++) {
            connectionHops[i] = (String) connectionHopsList.get(i);
        }
        obj.connectionHops = connectionHops;
        reader.end();

        obj.version = reader.readString();
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(6);
        writer.write(this.state);
        writer.write(this.ordering);
        writer.write(this.counterparty.portId);
        writer.write(this.counterparty.channelId);

        String[] connectionHops = this.connectionHops;
        if (connectionHops != null) {
            writer.beginNullableList(connectionHops.length);
            for (String v : connectionHops) {
                ByteArrayObjectWriter vWriter = Context.newByteArrayObjectWriter("RLPn");
                vWriter.write(v);
                writer.write(vWriter.toByteArray());
            }
            writer.end();
        } else {
            writer.writeNull();
        }

        writer.write(this.version);

        writer.end();
    }

    public static Channel fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return Channel.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        Channel.writeObject(writer, this);
        return writer.toByteArray();
    }

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
