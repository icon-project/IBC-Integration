package ibc.ics03.connection;

import java.math.BigInteger;

import ibc.icon.interfaces.IIBCConnection;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.Logger;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Counterparty;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.ics24.host.IBCStore;
import score.Context;

public class IBCConnection implements IIBCConnection {
    private static final String v1Identifier = "1";
    private static final String[] supportedV1Features = new String[] { "ORDER_ORDERED", "ORDER_UNORDERED" };
    private static final String commitmentPrefix = "ibc";

    IBCStore store = new IBCStore();
    Logger logger = new Logger("ibc-core");

    public String connectionOpenInit(MsgConnectionOpenInit msg) {
        String connectionId = generateConnectionIdentifier();
        Context.require(IBCStore.connections.get(connectionId) == null, "connectionId already exists");
        ConnectionEnd connection = new ConnectionEnd();
        connection.setClientId(msg.clientId);
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_INIT);
        connection.setDelayPeriod(msg.delayPeriod);
        connection.setCounterparty(msg.counterparty);

        updateConnectionCommitment(connectionId);
        IBCStore.connections.set(connectionId, connection);

        return connectionId;
    }

    public String connectionOpenTry(MsgConnectionOpenTry msg) {
        // TODO: investigate need to self client validation
        Context.require(msg.counterpartyVersions.length > 0, "counterpartyVersions length must be greater than 0");

        String connectionId = generateConnectionIdentifier();
        Context.require(IBCStore.connections.get(connectionId) == null, "connectionId already exists");
        ConnectionEnd connection = new ConnectionEnd();

        connection.setClientId(msg.clientId);
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        connection.setDelayPeriod(msg.delayPeriod);
        connection.setCounterparty(msg.counterparty);

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(msg.clientId);
        expectedCounterparty.setConnectionId("");
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(msg.counterparty.getClientId());
        expectedConnection.setVersions(msg.counterpartyVersions);
        expectedConnection.setState(ConnectionEnd.State.STATE_INIT);
        expectedConnection.setDelayPeriod(msg.delayPeriod);
        expectedConnection.setCounterparty(expectedCounterparty);

        boolean connectionStateVerification = verifyConnectionState(connection, msg.proofHeight, msg.proofInit,
                msg.counterparty.getConnectionId(), expectedConnection);
        Context.require(connectionStateVerification, "failed to verify connection state");

        boolean clientStateVerification = verifyClientState(
                connection,
                msg.proofHeight,
                getClientStatePath(connection.counterparty.getClientId()),
                msg.proofClient,
                msg.clientStateBytes);
        Context.require(clientStateVerification, "failed to verify clientState");
        // TODO we should also verify a consensus state

        updateConnectionCommitment(connectionId);
        IBCStore.connections.set(connectionId, connection);

        return connectionId;
    }

    public void connectionOpenAck(MsgConnectionOpenAck msg) {
        ConnectionEnd connection = IBCStore.connections.get(msg.connectionId);
        Context.require(connection != null, "connection does not exist");
        ConnectionEnd.State state = connection.getState();
        Context.require(state.equals(ConnectionEnd.State.STATE_INIT) || state.equals(ConnectionEnd.State.STATE_TRYOPEN),
                "connection state is not INIT or TRYOPEN");
        if (state.equals(ConnectionEnd.State.STATE_INIT)) {
            Context.require(isSupportedVersion(msg.version),
                    "connection state is in INIT but the provided version is not supported");
        } else {
            Context.require(connection.versions.length == 1 && isEqualVersion(connection.versions[0], msg.version),
                    "connection state is in TRYOPEN but the provided version is not set in the previous connection versions");
        }

        // TODO: investigate need to self client validation
        // require(validateSelfClient(msg.clientStateBytes), "failed to validate self
        // client state");

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.connectionId);
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getClientId());
        expectedConnection.setVersions(new Version[] { msg.version });
        expectedConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        expectedConnection.setDelayPeriod(connection.delayPeriod);
        expectedConnection.setCounterparty(expectedCounterparty);

        boolean connectionStateVerification = verifyConnectionState(connection, msg.proofHeight, msg.proofTry,
                msg.counterpartyConnectionID, expectedConnection);
        Context.require(connectionStateVerification, "failed to verify connection state");

        boolean clientStateVerification = verifyClientState(
                connection,
                msg.proofHeight,
                getClientStatePath(connection.counterparty.getClientId()),
                msg.proofClient,
                msg.clientStateBytes);
        Context.require(clientStateVerification, "failed to verify clientState");

        // TODO we should also verify a consensus state

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        connection.setVersions(expectedConnection.versions);
        connection.counterparty.setConnectionId(msg.counterpartyConnectionID);
        updateConnectionCommitment(msg.connectionId);

        IBCStore.connections.set(msg.connectionId, connection);
    }

    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg) {
        ConnectionEnd connection = IBCStore.connections.get(msg.connectionId);
        Context.require(connection != null, "connection does not exist");
        ConnectionEnd.State state = connection.getState();
        Context.require(state.equals(ConnectionEnd.State.STATE_TRYOPEN), "connection state is not TRYOPEN");

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.connectionId);
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getCounterparty().getClientId());
        expectedConnection.setVersions(connection.getVersions());
        expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
        expectedConnection.setDelayPeriod(connection.delayPeriod);
        expectedConnection.setCounterparty(expectedCounterparty);

        boolean connectionStateVerification = verifyConnectionState(connection, msg.proofHeight, msg.proofAck,
                connection.getCounterparty().getConnectionId(), expectedConnection);
        Context.require(connectionStateVerification, "failed to verify connection state");

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        updateConnectionCommitment(msg.connectionId);

        IBCStore.connections.set(msg.connectionId, connection);
    }

    /* Verification functions */

    private boolean verifyClientState(ConnectionEnd connection, Height height, byte[] path, byte[] proof,
            byte[] clientStatebytes) {
        ILightClient client = new ILightClientScoreInterface(IBCStore.clientImpls.get(connection.getClientId()));
        return client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                clientStatebytes);
    }

    private boolean verifyClientConsensusState(ConnectionEnd connection, Height height, Height consensusHeight,
            byte[] proof, byte[] consensusStateBytes) {
        byte[] consensusPath = getConsensusStatePath(connection.getCounterparty().getClientId(),
                consensusHeight.getRevisionNumber(),
                consensusHeight.getRevisionHeight());

        ILightClient client = new ILightClientScoreInterface(IBCStore.clientImpls.get(connection.getClientId()));
        return client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                consensusPath,
                consensusStateBytes);
    }

    private boolean verifyConnectionState(ConnectionEnd connection, Height height, byte[] proof, String connectionId,
            ConnectionEnd counterpartyConnection) {
        ILightClient client = new ILightClientScoreInterface(IBCStore.clientImpls.get(connection.getClientId()));
        return client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                getConnectionPath(connectionId),
                counterpartyConnection.toBytes());
    }

    /* Internal functions */

    private String generateConnectionIdentifier() {
        BigInteger currConnectionSequence = IBCStore.nextConnectionSequence.getOrDefault(BigInteger.ZERO);
        String identifier = "connection-" + currConnectionSequence.toString();
        IBCStore.nextConnectionSequence.set(currConnectionSequence.add(BigInteger.ONE));

        return identifier;
    }

    // TODO: Implement
    private byte[] getConnectionPath(String connectionId) {
        return new byte[0];
        // IBCCommitment.consensusStatePath(
        // connection.getCounterparty().getClientId(), consensusHeight.revision_number,
        // consensusHeight.revision_height)
    }

    // TODO: Implement
    private byte[] getConsensusStatePath(String clientId, BigInteger revisionNumber, BigInteger revisionHeight) {
        return new byte[0];
        // IBCCommitment.consensusStatePath(
        // connection.getCounterparty().getClientId(), consensusHeight.revision_number,
        // consensusHeight.revision_height)
    }

    /**
     * @dev getSupportedVersions return the supported versions.
     *
     */
    private Version[] getSupportedVersions() {
        Version version = new Version();
        version.setFeatures(supportedV1Features);
        version.setIdentifier(v1Identifier);

        return new Version[] { version };
    }

    // TODO implement
    private boolean isSupportedVersion(Version version) {
        return true;
    }

    // TODO implement
    private boolean isEqualVersion(Version a, Version b) {
        return true;
    }

    private void updateConnectionCommitment(String connectionId) {
        // TODO IBC-Store: wait for implementation
    }

    private byte[] getClientStatePath(String clientId) {
        return new byte[0];
        // TODO IBC-Store: wait for implementation
        // IBCCommitment.clientStatePath(connection.counterparty.getClientId())
    }

}
