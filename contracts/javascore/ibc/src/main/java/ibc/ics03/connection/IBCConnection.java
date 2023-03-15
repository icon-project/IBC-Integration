package ibc.ics03.connection;

import ibc.icon.interfaces.IIBCConnection;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Logger;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.Version;
import ibc.ics02.client.IBCClient;
import ibc.ics24.host.IBCCommitment;
import score.Context;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

public class IBCConnection extends IBCClient implements IIBCConnection {
    public static final String v1Identifier = "1";
    public static final List<String> supportedV1Features = List.of("ORDER_ORDERED", "ORDER_UNORDERED");
    public static final byte[] commitmentPrefix = "ibc".getBytes();

    Logger logger = new Logger("ibc-core");

    public String connectionOpenInit(MsgConnectionOpenInit msg) {
        String connectionId = generateConnectionIdentifier();
        Context.require(connections.get(connectionId) == null, "connectionId already exists");
        ILightClient client = getClient(msg.getClientId());
        Context.require(client.getClientState(msg.getClientId()) != null, "Client state not found");

        ConnectionEnd connection = new ConnectionEnd();
        connection.setClientId(msg.getClientId());
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_INIT);
        connection.setDelayPeriod(msg.getDelayPeriod());
        connection.setCounterparty(msg.getCounterparty());

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(connectionId, encodedConnection);
        connections.set(connectionId, encodedConnection);

        return connectionId;
    }

    public String connectionOpenTry(MsgConnectionOpenTry msg) {
        List<Version> counterpartyVersions = msg.getCounterpartyVersions();
        // TODO: investigate need to self client validation
        Context.require(counterpartyVersions.size() > 0, "counterpartyVersions length must be greater than 0");

        String connectionId = generateConnectionIdentifier();
        Context.require(connections.get(connectionId) == null, "connectionId already exists");

        Counterparty counterparty = msg.getCounterparty();
        ConnectionEnd connection = new ConnectionEnd();
        connection.setClientId(msg.getClientId());
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        connection.setDelayPeriod(msg.getDelayPeriod());
        connection.setCounterparty(counterparty);

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(msg.getClientId());
        expectedCounterparty.setConnectionId("");
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(counterparty.getClientId());
        expectedConnection.setVersions(counterpartyVersions);
        expectedConnection.setState(ConnectionEnd.State.STATE_INIT);
        expectedConnection.setDelayPeriod(msg.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeightRaw(), msg.getProofInit(), counterparty.getConnectionId(),
                expectedConnection);

        verifyClientState(
                connection,
                msg.getProofHeightRaw(),
                IBCCommitment.clientStatePath(connection.getCounterparty().getClientId()),
                msg.getProofClient(),
                msg.getClientStateBytes());
        // TODO we should also verify a consensus state

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(connectionId, encodedConnection);
        connections.set(connectionId, encodedConnection);

        return connectionId;
    }

    public void connectionOpenAck(MsgConnectionOpenAck msg) {
        ConnectionEnd connection = ConnectionEnd.decode(connections.get(msg.getConnectionId()));
        Context.require(connection != null, "connection does not exist");
        int state = connection.getState();
        // TODO should we allow the state to be TRY_OPEN?
        Context.require(state == ConnectionEnd.State.STATE_INIT || state == ConnectionEnd.State.STATE_TRYOPEN,
                "connection state is not INIT or TRYOPEN");
        if (state == ConnectionEnd.State.STATE_INIT) {
            Context.require(isSupportedVersion(msg.getVersion()),
                    "connection state is in INIT but the provided version is not supported");
        } else {
            Context.require(connection.getVersions().size() == 1
                            && Arrays.equals(connection.getVersions().get(0).encode(), msg.getVersionRaw()),
                    "connection state is in TRYOPEN but the provided version is not set in the previous connection " +
                            "versions");
        }

        // TODO: investigate need to self client validation
        // require(validateSelfClient(msg.clientStateBytes), "failed to validate self
        // client state");

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.getConnectionId());
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getClientId());
        expectedConnection.setVersions(List.of(msg.getVersion()));
        expectedConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        expectedConnection.setDelayPeriod(connection.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeightRaw(), msg.getProofTry(), msg.getCounterpartyConnectionID(),
                expectedConnection);

        verifyClientState(
                connection,
                msg.getProofHeightRaw(),
                IBCCommitment.clientStatePath(connection.getCounterparty().getClientId()),
                msg.getProofClient(),
                msg.getClientStateBytes());

        // TODO: we should also verify a consensus state

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        connection.setVersions(expectedConnection.getVersions());
        connection.getCounterparty().setConnectionId(msg.getCounterpartyConnectionID());

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(msg.getConnectionId(), encodedConnection);
        connections.set(msg.getConnectionId(), encodedConnection);

    }

    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg) {
        ConnectionEnd connection = ConnectionEnd.decode(connections.get(msg.getConnectionId()));
        Context.require(connection != null, "connection does not exist");
        int state = connection.getState();
        Context.require(state == ConnectionEnd.State.STATE_TRYOPEN, "connection state is not TRYOPEN");

        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix(commitmentPrefix);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.getConnectionId());
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getCounterparty().getClientId());
        expectedConnection.setVersions(connection.getVersions());
        expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
        expectedConnection.setDelayPeriod(connection.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeightRaw(), msg.getProofAck(),
                connection.getCounterparty().getConnectionId(), expectedConnection);

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(msg.getConnectionId(), encodedConnection);
        connections.set(msg.getConnectionId(), encodedConnection);
    }

    /* Verification functions */

    private void verifyClientState(ConnectionEnd connection, byte[] height, byte[] path, byte[] proof,
                                   byte[] clientStatebytes) {
        ILightClient client = getClient(connection.getClientId());
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                clientStatebytes);
        Context.require(ok, "failed to verify clientState");
    }

    private void verifyClientConsensusState(ConnectionEnd connection, byte[] height, Height consensusHeight,
                                            byte[] proof, byte[] consensusStateBytes) {
        byte[] consensusPath = IBCCommitment.consensusStatePath(connection.getCounterparty().getClientId(),
                consensusHeight.getRevisionNumber(),
                consensusHeight.getRevisionHeight());

        ILightClient client = getClient(connection.getClientId());
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                consensusPath,
                consensusStateBytes);
        Context.require(ok, "failed to verify consensus state");

    }

    private void verifyConnectionState(ConnectionEnd connection, byte[] height, byte[] proof, String connectionId,
                                       ConnectionEnd counterpartyConnection) {
        ILightClient client = getClient(connection.getClientId());
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                IBCCommitment.connectionPath(connectionId),
                counterpartyConnection.encode());
        Context.require(ok, "failed to verify connection state");
    }

    /* Internal functions */

    private String generateConnectionIdentifier() {
        BigInteger currConnectionSequence = nextConnectionSequence.getOrDefault(BigInteger.ZERO);
        String identifier = "connection-" + currConnectionSequence.toString();
        nextConnectionSequence.set(currConnectionSequence.add(BigInteger.ONE));

        return identifier;
    }

    /**
     * {@code @dev} getSupportedVersions return the supported versions.
     */
    private List<Version> getSupportedVersions() {
        Version version = new Version();
        version.setFeatures(supportedV1Features);
        version.setIdentifier(v1Identifier);

        return List.of(version);
    }

    // TODO implement
    private boolean isSupportedVersion(Version version) {
        return true;
    }

    private void updateConnectionCommitment(String connectionId, byte[] connectionBytes) {
        sendBTPMessage(ByteUtil.join(IBCCommitment.connectionCommitmentKey(connectionId),
                IBCCommitment.keccak256(connectionBytes)));
        // commitments.set(IBCCommitment.connectionCommitmentKey(connectionId),
        // IBCCommitment.keccak256(connection.toBytes()));>
    }

}
