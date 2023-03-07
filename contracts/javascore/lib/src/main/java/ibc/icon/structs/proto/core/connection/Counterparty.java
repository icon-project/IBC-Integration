package ibc.icon.structs.proto.core.connection;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import score.ObjectReader;
import score.ObjectWriter;

// Counterparty defines the counterparty chain associated with a connection end.
public class Counterparty {

    // identifies the client on the counterparty chain associated with a given
    // connection.
    private String clientId;

    // identifies the connection end on the counterparty chain associated with a
    // given connection.
    private String connectionId;

    // commitment merkle prefix of the counterparty chain.
    private MerklePrefix prefix;

    public static void writeObject(ObjectWriter writer, Counterparty obj) {
        obj.writeObject(writer);
    }

    public static Counterparty readObject(ObjectReader reader) {
        Counterparty obj = new Counterparty();
        reader.beginList();
        obj.clientId = reader.readString();
        obj.connectionId = reader.readString();
        obj.prefix = new MerklePrefix();
        obj.prefix.setKeyPrefix(reader.readByteArray());
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(3);
        writer.write(this.clientId);
        writer.write(this.connectionId);
        writer.write(this.prefix.getKeyPrefix());
        writer.end();
    }

    public byte[] encode() {
        return ByteUtil.join(
                Proto.encode(1, clientId),
                Proto.encode(2, connectionId),
                Proto.encode(3, prefix.encode()));
    }

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public String getConnectionId() {
        return connectionId;
    }

    public void setConnectionId(String connectionId) {
        this.connectionId = connectionId;
    }

    public MerklePrefix getPrefix() {
        return prefix;
    }

    public void setPrefix(MerklePrefix prefix) {
        this.prefix = prefix;
    }
}
