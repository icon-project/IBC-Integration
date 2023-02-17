package ibc.icon.structs.proto.core.connection;

import ibc.icon.structs.proto.core.commitment.MerklePrefix;

// Counterparty defines the counterparty chain associated with a connection end.
public class Counterparty {

    // identifies the client on the counterparty chain associated with a given
    // connection.
    String clientId;

    // identifies the connection end on the counterparty chain associated with a
    // given connection.
    String connectionId;

    // commitment merkle prefix of the counterparty chain.
    MerklePrefix prefix;

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
