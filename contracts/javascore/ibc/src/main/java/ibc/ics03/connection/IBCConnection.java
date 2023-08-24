package ibc.ics03.connection;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Logger;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import ibc.ics02.client.IBCClient;
import ibc.ics24.host.IBCCommitment;
import icon.proto.core.client.Height;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.Version;
import score.Context;
import scorex.util.ArrayList;

public class IBCConnection extends IBCClient {
    public static final String v1Identifier = "1";
    public static final List<String> supportedV1Features = List.of("ORDER_ORDERED", "ORDER_UNORDERED");

    Logger logger = new Logger("ibc-core");

    public String _connectionOpenInit(MsgConnectionOpenInit msg) {
        String connectionId = generateConnectionIdentifier();
        Context.require(connections.get(connectionId) == null, "connectionId already exists");
        ILightClient client = getClient(msg.getClientId());
        Context.require(client.getClientState(msg.getClientId()) != null, "Client state not found");

        ConnectionEnd connection = new ConnectionEnd();
        connection.setClientId(msg.getClientId());
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_INIT);
        connection.setDelayPeriod(msg.getDelayPeriod());
        connection.setCounterparty(Counterparty.decode(msg.getCounterparty()));

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(connection.getClientId(), connectionId, encodedConnection);
        connections.set(connectionId, encodedConnection);

        return connectionId;
    }

    public String _connectionOpenTry(MsgConnectionOpenTry msg) {
        List<Version> counterpartyVersions = decodeCounterpartyVersions(msg.getCounterpartyVersions());
        // TODO: investigate need to self client validation
        Context.require(counterpartyVersions.size() > 0, "counterpartyVersions length must be greater than 0");

        String connectionId = generateConnectionIdentifier();
        Context.require(connections.get(connectionId) == null, "connectionId already exists");

        Counterparty counterparty = Counterparty.decode(msg.getCounterparty());
        ConnectionEnd connection = new ConnectionEnd();
        connection.setClientId(msg.getClientId());
        connection.setVersions(getSupportedVersions());
        connection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        connection.setDelayPeriod(msg.getDelayPeriod());
        connection.setCounterparty(counterparty);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(msg.getClientId());
        expectedCounterparty.setConnectionId("");

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(counterparty.getClientId());
        expectedConnection.setVersions(counterpartyVersions);
        expectedConnection.setState(ConnectionEnd.State.STATE_INIT);
        expectedConnection.setDelayPeriod(msg.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeight(), msg.getProofInit(), counterparty.getConnectionId(),
                expectedConnection);

        // TODO Investigate need for client verification if premissioned
        verifyClientState(
                connection,
                msg.getProofHeight(),
                IBCCommitment.clientStatePath(connection.getCounterparty().getClientId()),
                msg.getProofClient(),
                msg.getClientStateBytes());

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(connection.getClientId(), connectionId, encodedConnection);
        connections.set(connectionId, encodedConnection);

        return connectionId;
    }

    public byte[] _connectionOpenAck(MsgConnectionOpenAck msg) {
        byte[] connectionPb = connections.get(msg.getConnectionId());
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        int state = connection.getState();
        // TODO should we allow the state to be TRY_OPEN?
        Context.require(state == ConnectionEnd.State.STATE_INIT, "connection state is not INIT");
        Context.require(isSupportedVersion(Version.decode(msg.getVersion())),
                "connection state is in INIT but the provided version is not supported");


        // TODO: investigate need to self client validation
        // require(validateSelfClient(msg.clientStateBytes), "failed to validate self
        // client state");

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.getConnectionId());

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getCounterparty().getClientId());
        expectedConnection.setVersions(List.of(Version.decode(msg.getVersion())));
        expectedConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        expectedConnection.setDelayPeriod(connection.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeight(), msg.getProofTry(), msg.getCounterpartyConnectionID(),
                expectedConnection);

        verifyClientState(
                connection,
                msg.getProofHeight(),
                IBCCommitment.clientStatePath(connection.getCounterparty().getClientId()),
                msg.getProofClient(),
                msg.getClientStateBytes());

        // TODO: we should also verify a consensus state

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        connection.setVersions(expectedConnection.getVersions());
        connection.getCounterparty().setConnectionId(msg.getCounterpartyConnectionID());

        byte[] encodedConnection = connection.encode();
        updateConnectionCommitment(connection.getClientId(), msg.getConnectionId(), encodedConnection);
        connections.set(msg.getConnectionId(), encodedConnection);

        return encodedConnection;
    }

    public byte[] _connectionOpenConfirm(MsgConnectionOpenConfirm msg) {
        byte[] connectionPb = connections.get(msg.getConnectionId());
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        int state = connection.getState();
        Context.require(state == ConnectionEnd.State.STATE_TRYOPEN, "connection state is not TRYOPEN");

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(connection.getClientId());
        expectedCounterparty.setConnectionId(msg.getConnectionId());

        ConnectionEnd expectedConnection = new ConnectionEnd();
        expectedConnection.setClientId(connection.getCounterparty().getClientId());
        expectedConnection.setVersions(connection.getVersions());
        expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
        expectedConnection.setDelayPeriod(connection.getDelayPeriod());
        expectedConnection.setCounterparty(expectedCounterparty);

        verifyConnectionState(connection, msg.getProofHeight(), msg.getProofAck(),
                connection.getCounterparty().getConnectionId(), expectedConnection);

        connection.setState(ConnectionEnd.State.STATE_OPEN);
        byte[] encodedConnection = connection.encode();
        connections.set(msg.getConnectionId(), encodedConnection);

        return encodedConnection;
    }

    /* Verification functions */

    private void verifyClientState(ConnectionEnd connection, byte[] height, byte[] path, byte[] proof,
                                   byte[] clientStatebytes) {
        ILightClient client = getClient(connection.getClientId());
        client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                clientStatebytes);
    }

    private void verifyConnectionState(ConnectionEnd connection, byte[] height, byte[] proof, String connectionId,
                                       ConnectionEnd counterpartyConnection) {
        ILightClient client = getClient(connection.getClientId());

        client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                IBCCommitment.connectionPath(connectionId),
                counterpartyConnection.encode());
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

    public List<Version> decodeCounterpartyVersions(byte[][] counterpartyVersions) {
        List<Version> versions = new ArrayList<>();
        for (int i = 0; i < counterpartyVersions.length; i++) {
            versions.add(Version.decode(counterpartyVersions[i]));
        }
        return versions;
    }

    // TODO implement
    private boolean isSupportedVersion(Version version) {
        return true;
    }

    private void updateConnectionCommitment(String clientId, String connectionId, byte[] connectionBytes) {
        ILightClient client = getClient(clientId);
        byte[] height = client.getLatestHeight(clientId);
        byte[] clientStateBytes = client.getClientState(clientId);
        byte[] consensusStateBytes = client.getConsensusState(clientId, height);

        Height updateHeight = Height.decode(height);
        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                updateHeight.getRevisionNumber(),
                updateHeight.getRevisionHeight());

        sendBTPMessage(clientId, ByteUtil.join(clientKey, IBCCommitment.keccak256(clientStateBytes)));
        sendBTPMessage(clientId, ByteUtil.join(consensusKey, IBCCommitment.keccak256(consensusStateBytes)));
        sendBTPMessage(clientId, ByteUtil.join(IBCCommitment.connectionCommitmentKey(connectionId),
                IBCCommitment.keccak256(connectionBytes)));
    }

}
