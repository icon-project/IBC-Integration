package ibc.icon.structs.proto.core.connection;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

import java.math.BigInteger;
import java.util.List;

// ConnectionEnd defines a stateful object on a chain connected to another
// separate one.
// NOTE: there must only be 2 defined ConnectionEnds to establish
// a connection between two chains.
public class ConnectionEnd {
    // State defines if a connection is in one of the following states:
    // INIT, TRYOPEN, OPEN or UNINITIALIZED.
    public static class State {
        public static int STATE_UNINITIALIZED_UNSPECIFIED = 0;
        // A connection end has just started the opening handshake.
        public static int STATE_INIT = 1;
        // A connection end has acknowledged the handshake step on the counterparty
        // chain.
        public static int STATE_TRYOPEN = 2;
        // A connection end has completed the handshake.
        public static int STATE_OPEN = 3;
    }

    // client associated with this connection.
    private String clientId;

    // IBC version which can be utilised to determine encodings or protocols for
    // channels or packets utilising this connection.
    private Version[] versions;

    // current state of the connection end.
    private int state;

    // counterparty chain associated with this connection.
    private Counterparty counterparty;

    // delay period that must pass before a consensus state can be used for
    // packet-verification NOTE: delay period logic is only implemented by some
    // clients.
    private BigInteger delayPeriod;

    public static void writeObject(ObjectWriter writer, ConnectionEnd obj) {
        obj.writeObject(writer);
    }

    public static ConnectionEnd readObject(ObjectReader reader) {
        ConnectionEnd obj = new ConnectionEnd();
        reader.beginList();
        obj.clientId = reader.readString();

        reader.beginList();
        Version[] versions = null;
        List<Version> versionsList = new ArrayList<>();
        while (reader.hasNext()) {
            byte[] versionElementBytes = reader.readNullable(byte[].class);
            if (versionElementBytes != null) {
                ObjectReader versionElementReader = Context.newByteArrayObjectReader("RLPn", versionElementBytes);
                versionsList.add(versionElementReader.read(Version.class));
            }
        }

        versions = new Version[versionsList.size()];
        for (int i = 0; i < versionsList.size(); i++) {
            versions[i] = (Version) versionsList.get(i);
        }
        obj.versions = versions;
        reader.end();

        obj.state = reader.readInt();
        obj.counterparty = reader.read(Counterparty.class);
        obj.delayPeriod = reader.readBigInteger();
        reader.end();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(5);
        writer.write(this.clientId);

        Version[] versions = this.getVersions();
        if (versions != null) {
            writer.beginNullableList(versions.length);
            for (Version v : versions) {
                ByteArrayObjectWriter vWriter = Context.newByteArrayObjectWriter("RLPn");
                vWriter.write(v);
                writer.write(vWriter.toByteArray());
            }
            writer.end();
        } else {
            writer.writeNull();
        }

        writer.write(this.state);
        writer.write(this.counterparty);
        writer.write(this.delayPeriod);

        writer.end();
    }

    public static ConnectionEnd fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return ConnectionEnd.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        ConnectionEnd.writeObject(writer, this);
        return writer.toByteArray();
    }

    public byte[] encode() {
        byte[][] encodedVersions = new byte[this.versions.length][];
        for (int i = 0; i < this.versions.length; i++) {
            encodedVersions[i] = Proto.encode(2, this.versions[i].encode());
        }

        return ByteUtil.join(
                Proto.encode(1, clientId),
                ByteUtil.join(encodedVersions),
                Proto.encode(3, BigInteger.valueOf(state)),
                Proto.encode(4, counterparty.encode()),
                Proto.encode(5, delayPeriod));
    }

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public Version[] getVersions() {
        return versions;
    }

    public void setVersions(Version[] versions) {
        this.versions = versions;
    }

    public void setState(int state) {
        this.state = state;
    }

    public int getState() {
        return state;
    }

    public Counterparty getCounterparty() {
        return counterparty;
    }

    public void setCounterparty(Counterparty counterparty) {
        this.counterparty = counterparty;
    }

    public BigInteger getDelayPeriod() {
        return delayPeriod;
    }

    public void setDelayPeriod(BigInteger delayPeriod) {
        this.delayPeriod = delayPeriod;
    }
}
