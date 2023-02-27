package ibc.icon.structs.proto.core.channel;

import java.math.BigInteger;

import ibc.icon.structs.proto.core.client.Height;

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
    public String data;
    // block height after which the packet times out
    public Height timeoutHeight;
    // block timestamp (in nanoseconds) after which the packet times out
    public BigInteger timeoutTimestamp;

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

    public String getData() {
        return data;
    }

    public void setData(String data) {
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
