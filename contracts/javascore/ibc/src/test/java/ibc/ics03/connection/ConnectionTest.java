package ibc.ics03.connection;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.doNothing;
import static org.mockito.Mockito.when;
import static org.mockito.Mockito.any;
import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.verify;

import java.math.BigInteger;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics24.host.IBCCommitment;
import score.Address;

public class ConnectionTest extends TestBase {
    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score connection;
    private MockContract<ILightClient> lightClient;
    private IBCConnection connectionSpy;

    Height proofHeight = new Height();
    Height consensusHeight = new Height();

    Counterparty counterparty = new Counterparty();
    MerklePrefix prefix = new MerklePrefix();
    Version version = new Version();

    BigInteger delayPeriod = BigInteger.TEN;
    String clientId = "type-0";

    ConnectionEnd baseConnection = new ConnectionEnd();

    public static class ConnectionMock extends IBCConnection {
        public ConnectionMock() {
        }

        public void setClient(String clientId, Address client) {
            clientImplementations.set(clientId, client);
        }
    }

    @BeforeEach
    public void setup() throws Exception {
        connection = sm.deploy(owner, ConnectionMock.class);
        connectionSpy = (IBCConnection) spy(connection.getInstance());
        connection.setInstance(connectionSpy);
        doNothing().when(connectionSpy).sendBTPMessage(any(byte[].class));

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);

        proofHeight.setRevisionHeight(BigInteger.valueOf(5));
        proofHeight.setRevisionNumber(BigInteger.valueOf(6));
        proofHeight.setRevisionHeight(BigInteger.valueOf(7));
        proofHeight.setRevisionNumber(BigInteger.valueOf(8));

        prefix.setKeyPrefix(IBCConnection.commitmentPrefix);

        counterparty.setClientId("counterpartyId");
        counterparty.setConnectionId("connectionId");
        counterparty.setPrefix(prefix);

        version.setIdentifier(IBCConnection.v1Identifier);
        version.setFeatures(IBCConnection.supportedV1Features);

        baseConnection.setClientId(clientId);
        baseConnection.setVersions(List.of(version));
        baseConnection.setDelayPeriod(delayPeriod);
        baseConnection.setCounterparty(counterparty);

        connection.invoke(owner, "setClient", clientId, lightClient.getAddress());
    }

    @Test
    void connectionOpenInit_clientNotFound() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();
        msg.setClientId("non existent");

        // Act & Assert
        String expectedErrorMessage = "Client does not exist";
        Executable openConnectionWithoutClient = () -> connection.invoke(owner,
                "connectionOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutClient);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenInit_clientStateNotFound() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();
        msg.setClientId(clientId);

        // Act & Assert
        String expectedErrorMessage = "Client state not found";
        Executable openConnectionWithoutState = () -> connection.invoke(owner,
                "connectionOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutState);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenInit() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.encode());
        msg.setDelayPeriod(delayPeriod);
        String expectedConnectionId = "connection-0";
        when(lightClient.mock.getClientState(msg.getClientId())).thenReturn(new byte[0]);

        // Act
        connection.invoke(owner, "connectionOpenInit", msg);

        // Assert
        ConnectionEnd expectedConnection = baseConnection;
        expectedConnection.setState(ConnectionEnd.State.STATE_INIT);

        // byte[] storedCommitment = (byte[]) connection.call("getCommitment",
        // IBCCommitment.connectionCommitmentKey(expectedConnectionId));
        // assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()),
        // storedCommitment);
        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(expectedConnectionId);
        verify(connectionSpy)
                .sendBTPMessage(ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.encode())));
        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
    }

    @Test
    void connectionOpenTry_MissingVersion() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.setCounterpartyVersions(new byte[0][]);

        // Act & Assert
        String expectedErrorMessage = "counterpartyVersions length must be greater than 0";
        Executable openConnectionWithoutVersion = () -> connection.invoke(owner,
                "connectionOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenTry_failedConnectionStateVerification() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.setCounterpartyVersions(new byte[0][]);

        // Act & Assert
        String expectedErrorMessage = "counterpartyVersions length must be greater than 0";
        Executable openConnectionWithoutVersion = () -> connection.invoke(owner,
                "connectionOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenTry_invalidStates() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.encode());
        msg.setDelayPeriod(delayPeriod);
        msg.setClientStateBytes(new byte[1]);
        msg.setCounterpartyVersions(new byte[][] { version.encode() });
        msg.setProofInit(new byte[2]);
        msg.setProofClient(new byte[3]);
        msg.setProofConsensus(new byte[4]);
        msg.setProofHeight(proofHeight.encode());
        msg.setConsensusHeight(consensusHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(msg.getClientId());
        expectedCounterparty.setConnectionId("");
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd counterpartyConnection = new ConnectionEnd();
        counterpartyConnection.setClientId(counterparty.getClientId());
        counterpartyConnection.setVersions(List.of(version));
        counterpartyConnection.setState(ConnectionEnd.State.STATE_INIT);
        counterpartyConnection.setDelayPeriod(msg.getDelayPeriod());
        counterpartyConnection.setCounterparty(expectedCounterparty);

        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(msg.getCounterparty().getConnectionId());
        when(lightClient.mock.verifyMembership(msg.getClientId(),
                msg.getProofHeightRaw(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofInit(), prefix.getKeyPrefix(), connectionPath,
                counterpartyConnection.encode()))
                .thenReturn(false).thenReturn(true);

        // verifyClientState
        byte[] clientStatePath = IBCCommitment.clientStatePath(msg.getCounterparty().getClientId());
        when(lightClient.mock.verifyMembership(msg.getClientId(), msg.getProofHeightRaw(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofClient(), prefix.getKeyPrefix(), clientStatePath,
                msg.getClientStateBytes()))
                .thenReturn(false);

        // Act & Assert
        String expectedErrorMessage = "failed to verify connection state";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "connectionOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                clientVerificationFailed);
        assertTrue(e.getMessage().contains(expectedErrorMessage));

        expectedErrorMessage = "failed to verify clientState";
        Executable stateVerificationFailed = () -> connection.invoke(owner,
                "connectionOpenTry", msg);
        e = assertThrows(AssertionError.class,
                stateVerificationFailed);
        assertTrue(e.getMessage().contains(expectedErrorMessage));

    }

    @Test
    void connectionOpenTry() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.encode());
        msg.setDelayPeriod(delayPeriod);
        msg.setClientStateBytes(new byte[1]);
        msg.setCounterpartyVersions(new byte[][] { version.encode() });
        msg.setProofInit(new byte[2]);
        msg.setProofClient(new byte[3]);
        msg.setProofConsensus(new byte[4]);
        msg.setProofHeight(proofHeight.encode());
        msg.setConsensusHeight(consensusHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(msg.getClientId());
        expectedCounterparty.setConnectionId("");
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd counterpartyConnection = new ConnectionEnd();
        counterpartyConnection.setClientId(counterparty.getClientId());
        counterpartyConnection.setVersions(List.of(version));
        counterpartyConnection.setState(ConnectionEnd.State.STATE_INIT);
        counterpartyConnection.setDelayPeriod(msg.getDelayPeriod());
        counterpartyConnection.setCounterparty(expectedCounterparty);

        String expectedConnectionId = "connection-0";

        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(msg.getCounterparty().getConnectionId());
        when(lightClient.mock.verifyMembership(msg.getClientId(),
                msg.getProofHeightRaw(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofInit(), prefix.getKeyPrefix(), connectionPath,
                counterpartyConnection.encode())).thenReturn(true);

        // verifyClientState
        byte[] clientStatePath = IBCCommitment.clientStatePath(msg.getCounterparty().getClientId());
        when(lightClient.mock.verifyMembership(msg.getClientId(), msg.getProofHeightRaw(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofClient(), prefix.getKeyPrefix(), clientStatePath,
                msg.getClientStateBytes()))
                .thenReturn(true);

        // Act
        connection.invoke(owner, "connectionOpenTry", msg);

        // Assert
        ConnectionEnd expectedConnection = baseConnection;
        expectedConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);

        // byte[] storedCommitment = (byte[]) connection.call("getCommitment",
        // IBCCommitment.connectionCommitmentKey(expectedConnectionId));
        // assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()),
        // storedCommitment);
        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(expectedConnectionId);
        verify(connectionSpy)
                .sendBTPMessage(ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.encode())));

        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
    }

    @Test
    void connectionOpenAck_alreadyOpen() {
        // Arrange
        connectionOpenConfirm();
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.setConnectionId("connection-0");

        // Act & Assert
        String expectedErrorMessage = "connection state is not INIT or TRYOPEN";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "connectionOpenAck", msg);
        AssertionError e = assertThrows(AssertionError.class,
                clientVerificationFailed);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenAck_wrongVersion() {
        // Arrange
        connectionOpenTry();
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.setConnectionId("connection-0");
        Version wrongVersion = new Version();
        wrongVersion.setIdentifier("OtherVersion");
        wrongVersion.setFeatures(List.of("some features"));
        msg.setVersion(wrongVersion.encode());

        // Act & Assert
        String expectedErrorMessage = "connection state is in TRYOPEN but the provided version is not set in the " +
                "previous connection versions";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "connectionOpenAck", msg);
        AssertionError e = assertThrows(AssertionError.class,
                clientVerificationFailed);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenAck() {
        // Arrange
        connectionOpenInit();
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.setConnectionId("connection-0");
        msg.setClientStateBytes(new byte[1]);
        msg.setVersion(version.encode());
        msg.setCounterpartyConnectionID(counterparty.getClientId());
        msg.setProofTry(new byte[2]);
        msg.setProofClient(new byte[3]);
        msg.setProofConsensus(new byte[4]);
        msg.setProofHeight(proofHeight.encode());
        msg.setConsensusHeight(consensusHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(clientId);
        expectedCounterparty.setConnectionId(msg.getConnectionId());
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd counterpartyConnection = new ConnectionEnd();
        counterpartyConnection.setClientId(clientId);
        counterpartyConnection.setVersions(List.of(version));
        counterpartyConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        counterpartyConnection.setDelayPeriod(delayPeriod);
        counterpartyConnection.setCounterparty(expectedCounterparty);

        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(msg.getCounterpartyConnectionID());
        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofTry(), prefix.getKeyPrefix(), connectionPath, counterpartyConnection.encode()))
                .thenReturn(true);

        // verifyClientState
        byte[] clientStatePath = IBCCommitment.clientStatePath(counterparty.getClientId());
        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofClient(), prefix.getKeyPrefix(), clientStatePath, msg.getClientStateBytes()))
                .thenReturn(true);

        // Act
        connection.invoke(owner, "connectionOpenAck", msg);

        // Assert
        ConnectionEnd expectedConnection = baseConnection;
        expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
        expectedConnection.setVersions(counterpartyConnection.getVersions());
        expectedConnection.getCounterparty().setConnectionId(msg.getCounterpartyConnectionID());
        // byte[] storedCommitment = (byte[]) connection.call("getCommitment",
        // IBCCommitment.connectionCommitmentKey(msg.connectionId));
        // assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()),
        // storedCommitment);
        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(msg.getConnectionId());
        verify(connectionSpy)
                .sendBTPMessage(ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.encode())));

        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
    }

    @Test
    void connectionOpenConfirm_NotInTryOpen() {
        // Arrange
        connectionOpenInit();
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.setConnectionId("connection-0");
        msg.setProofAck(new byte[1]);
        msg.setProofHeight(proofHeight.encode());

        // Act & Assert
        String expectedErrorMessage = "connection state is not TRYOPEN";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "connectionOpenConfirm", msg);
        AssertionError e = assertThrows(AssertionError.class,
                clientVerificationFailed);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenConfirm() {
        // Arrange
        connectionOpenTry();
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.setConnectionId("connection-0");
        msg.setProofAck(new byte[1]);
        msg.setProofHeight(proofHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setClientId(clientId);
        expectedCounterparty.setConnectionId(msg.getConnectionId());
        expectedCounterparty.setPrefix(prefix);

        ConnectionEnd counterpartyConnection = new ConnectionEnd();
        counterpartyConnection.setClientId(counterparty.getClientId());
        counterpartyConnection.setVersions(List.of(version));
        counterpartyConnection.setState(ConnectionEnd.State.STATE_OPEN);
        counterpartyConnection.setDelayPeriod(delayPeriod);
        counterpartyConnection.setCounterparty(expectedCounterparty);

        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(counterparty.getConnectionId());
        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofAck(), prefix.getKeyPrefix(), connectionPath, counterpartyConnection.encode()))
                .thenReturn(true);

        // Act
        connection.invoke(owner, "connectionOpenConfirm", msg);

        // Assert
        ConnectionEnd expectedConnection = baseConnection;
        expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
        // byte[] storedCommitment = (byte[]) connection.call("getCommitment",
        // IBCCommitment.connectionCommitmentKey(msg.connectionId));
        // assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()),
        // storedCommitment);
        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(msg.getConnectionId());
        verify(connectionSpy)
                .sendBTPMessage(ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.encode())));

        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));

    }
}
