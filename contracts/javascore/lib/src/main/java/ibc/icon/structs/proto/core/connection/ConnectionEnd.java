package ibc.icon.structs.proto.core.connection;

import java.math.BigInteger;

// ConnectionEnd defines a stateful object on a chain connected to another
// separate one.
// NOTE: there must only be 2 defined ConnectionEnds to establish
// a connection between two chains.
public class ConnectionEnd {
    // State defines if a connection is in one of the following states:
    // INIT, TRYOPEN, OPEN or UNINITIALIZED.
    enum State {
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
