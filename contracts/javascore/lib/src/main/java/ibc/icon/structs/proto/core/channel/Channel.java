package ibc.icon.structs.proto.core.channel;


import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

import java.math.BigInteger;
import java.util.List;


public class Channel {

    // State defines if a channel is in one of the following states:
    // CLOSED, INIT, TRYOPEN, OPEN or UNINITIALIZED.
    public static class State {
        // Default State
        public static int STATE_UNINITIALIZED_UNSPECIFIED = 0;
        // A channel has just started the opening handshake.
        public static int STATE_INIT = 1;
        // A channel has acknowledged the handshake step on the counterparty chain.
        public static int STATE_TRYOPEN = 2;
        // A channel has completed the handshake. Open channels are
        // ready to send and receive packets.
        public static int STATE_OPEN = 3;
        // A channel has been closed and can no longer be used to send or receive
        // packets.
        public static int STATE_CLOSED = 4;
    }

    // Order defines if a channel is ORDERED or UNORDERED
    public static class Order {
        // zero-value for channel ordering
        public static int ORDER_NONE_UNSPECIFIED = 0;
        // packets can be delivered in any order, which may differ from the order in
        // which they were sent.
        public static int ORDER_UNORDERED = 1;
        // packets are delivered exactly in the order which they were sent
        public static int ORDER_ORDERED = 2;
    }

    // current state of the channel end
    public int state;
    // whether the channel is ordered or unordered
    public int ordering;
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
        obj.state = reader.readInt();
        obj.ordering = reader.readInt();
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

    public byte[] encode() {
        byte[][] encodedConnectionHops = new byte[this.connectionHops.length][];
        for (int i = 0; i < this.connectionHops.length; i++) {
            encodedConnectionHops[i] = Proto.encode(4, this.connectionHops[i]);
        }

        return ByteUtil.join(
                Proto.encode(1, BigInteger.valueOf(state)),
                Proto.encode(2, BigInteger.valueOf(ordering)),
                Proto.encode(3, counterparty.encode()),
                ByteUtil.join(encodedConnectionHops),
                Proto.encode(5, version));
    }

    public void setState(int state) {
        this.state = state;
    }

    public int getState() {
        return state;
    }

    public void setOrdering(int ordering) {
        this.ordering = ordering;
    }

    public int getOrdering() {
        return ordering;
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
