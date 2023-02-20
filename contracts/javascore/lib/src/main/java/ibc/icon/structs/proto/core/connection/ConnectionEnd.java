package ibc.icon.structs.proto.core.connection;

import java.math.BigInteger;
import java.util.List;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

// ConnectionEnd defines a stateful object on a chain connected to another
// separate one.
// NOTE: there must only be 2 defined ConnectionEnds to establish
// a connection between two chains.
public class ConnectionEnd {
    // State defines if a connection is in one of the following states:
    // INIT, TRYOPEN, OPEN or UNINITIALIZED.
    public enum State {
        STATE_UNINITIALIZED_UNSPECIFIED,
        // A connection end has just started the opening handshake.
        STATE_INIT,
        // A connection end has acknowledged the handshake step on the counterparty
        // chain.
        STATE_TRYOPEN,
        // A connection end has completed the handshake.
        STATE_OPEN
    }

    // client associated with this connection.
    public String clientId;

    // IBC version which can be utilised to determine encodings or protocols for
    // channels or packets utilising this connection.
    public Version[] versions;

    // current state of the connection end.
    public String state;

    // counterparty chain associated with this connection.
    public Counterparty counterparty;

    // delay period that must pass before a consensus state can be used for
    // packet-verification NOTE: delay period logic is only implemented by some
    // clients.
    public BigInteger delayPeriod;

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

        obj.state = reader.readString();
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

    public State getState() {
        return State.valueOf(state);
    }

    public void setState(State state) {
        this.state = state.toString();
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
