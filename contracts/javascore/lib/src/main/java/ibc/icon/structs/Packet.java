package ibc.icon.structs;

import java.math.BigInteger;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class Packet {

    public BigInteger sequence;
    public String sourcePort;
    public String sourceChannel;
    public String destinationPort;
    public String destinationChannel;
    public byte[] data;
    public Height timeoutHeight;
    public BigInteger timestamp;

    public BigInteger getSequence() {
        return sequence;
    }

    public BigInteger getTimestamp() {
        return timestamp;
    }

    public byte[] getData() {
        return data;
    }

    public Height getTimeoutHeight() {
        return timeoutHeight;
    }

    public String getDestinationChannel() {
        return destinationChannel;
    }

    public String getDestinationPort() {
        return destinationPort;
    }

    public String getSourceChannel() {
        return sourceChannel;
    }

    public String getSourcePort() {
        return sourcePort;
    }

    public void setData(byte[] data) {
        this.data = data;
    }

    public void setDestinationChannel(String destinationChannel) {
        this.destinationChannel = destinationChannel;
    }

    public void setDestinationPort(String destinationPort) {
        this.destinationPort = destinationPort;
    }

    public void setSequence(BigInteger sequence) {
        this.sequence = sequence;
    }

    public void setSourceChannel(String sourceChannel) {
        this.sourceChannel = sourceChannel;
    }

    public void setSourcePort(String sourcePort) {
        this.sourcePort = sourcePort;
    }

    public void setTimeoutHeight(Height timeoutHeight) {
        this.timeoutHeight = timeoutHeight;
    }

    public void setTimestamp(BigInteger timestamp) {
        this.timestamp = timestamp;
    }

    public static Packet readObject(ObjectReader reader) {
        Packet obj = new Packet();
        reader.beginList();
        obj.setSequence(reader.readBigInteger());
        obj.setSourcePort(reader.readString());
        obj.setSourceChannel(reader.readString());
        obj.setDestinationPort(reader.readString());
        obj.setDestinationChannel(reader.readString());
        obj.setData(reader.readByteArray());
        obj.setTimeoutHeight(reader.read(Height.class));
        obj.setTimestamp(reader.readBigInteger());
        reader.end();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(8);
        writer.write(this.getSequence());
        writer.write(this.getSourcePort());
        writer.write(this.getSourceChannel());
        writer.write(this.getDestinationPort());
        writer.write(this.getDestinationChannel());
        writer.write(this.getData());
        writer.write(this.getTimeoutHeight());
        writer.write(this.getTimestamp());
        writer.end();
    }

    public static void writeObject(ObjectWriter writer, Packet obj) {
        obj.writeObject(writer);
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
}
