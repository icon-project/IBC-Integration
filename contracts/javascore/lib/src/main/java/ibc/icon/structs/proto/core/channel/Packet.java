package ibc.icon.structs.proto.core.channel;

import ibc.icon.structs.proto.core.client.Height;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

// Packet defines a type that carries data across different chains through IBC
public class Packet {
    // number corresponds to the order of sends and receives, where a Packet
    // with an earlier sequence number must be sent and received before a Packet
    // with a later sequence number.
    public BigInteger sequence;
    // identifies the port on the sending chain.
    public String sourcePort;
    // identifies the channel end on the sending chain.
    public String sourceChannel;
    // identifies the port on the receiving chain.
    public String destinationPort;
    // identifies the channel end on the receiving chain.
    public String destinationChannel;
    // actual opaque bytes transferred directly to the application module
    public byte[] data;
    // block height after which the packet times out
    public Height timeoutHeight;
    // block timestamp (in nanoseconds) after which the packet times out
    public BigInteger timeoutTimestamp;

    public static void writeObject(ObjectWriter writer, Packet obj) {
        obj.writeObject(writer);
    }

    public static Packet readObject(ObjectReader reader) {
        Packet obj = new Packet();
        reader.beginList();
        obj.sequence = reader.readBigInteger();
        obj.sourcePort = reader.readString();
        obj.sourceChannel = reader.readString();
        obj.destinationPort = reader.readString();
        obj.destinationChannel = reader.readString();
        obj.data = reader.readByteArray();

        Height timeoutHeight = new Height();
        timeoutHeight.setRevisionNumber(reader.readBigInteger());
        timeoutHeight.setRevisionHeight(reader.readBigInteger());

        obj.timeoutHeight = timeoutHeight;
        obj.timeoutTimestamp = reader.readBigInteger();
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(9);
        writer.write(this.sequence);
        writer.write(this.sourcePort);
        writer.write(this.sourceChannel);
        writer.write(this.destinationPort);
        writer.write(this.destinationChannel);
        writer.write(this.data);
        writer.write(this.timeoutHeight.getRevisionNumber());
        writer.write(this.timeoutHeight.getRevisionHeight());
        writer.write(this.timeoutTimestamp);

        writer.end();
    }

    public static Packet fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return Packet.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        Packet.writeObject(writer, this);
        return writer.toByteArray();
    }

    public BigInteger getSequence() {
        return sequence;
    }

    public void setSequence(BigInteger sequence) {
        this.sequence = sequence;
    }

    public String getSourcePort() {
        return sourcePort;
    }

    public void setSourcePort(String sourcePort) {
        this.sourcePort = sourcePort;
    }

    public String getSourceChannel() {
        return sourceChannel;
    }

    public void setSourceChannel(String sourceChannel) {
        this.sourceChannel = sourceChannel;
    }

    public String getDestinationPort() {
        return destinationPort;
    }

    public void setDestinationPort(String destinationPort) {
        this.destinationPort = destinationPort;
    }

    public String getDestinationChannel() {
        return destinationChannel;
    }

    public void setDestinationChannel(String destinationChannel) {
        this.destinationChannel = destinationChannel;
    }

    public byte[] getData() {
        return data;
    }

    public void setData(byte[] data) {
        this.data = data;
    }

    public Height getTimeoutHeight() {
        return timeoutHeight;
    }

    public void setTimeoutHeight(Height timeoutHeight) {
        this.timeoutHeight = timeoutHeight;
    }

    public BigInteger getTimeoutTimestamp() {
        return timeoutTimestamp;
    }

    public void setTimeoutTimestamp(BigInteger timeoutTimestamp) {
        this.timeoutTimestamp = timeoutTimestamp;
    }
}
