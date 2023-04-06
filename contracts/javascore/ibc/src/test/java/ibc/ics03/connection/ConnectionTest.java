package ibc.ics03.connection;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doNothing;
import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import com.google.protobuf.ByteString;
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
import ibc.icon.test.MockContract;
import ibc.ics24.host.IBCCommitment;
import score.Address;
import test.proto.core.client.Client.Height;
import test.proto.core.connection.Connection.ConnectionEnd;
import test.proto.core.connection.Connection.Counterparty;
import test.proto.core.connection.Connection.MerklePrefix;
import test.proto.core.connection.Connection.Version;

public class ConnectionTest extends TestBase {
    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score connection;
    private MockContract<ILightClient> lightClient;
    private IBCConnection connectionSpy;

    Height proofHeight;
    Height consensusHeight;

    Counterparty counterparty;
    MerklePrefix prefix;
    Version version;
    BigInteger delayPeriod = BigInteger.TEN;
    String clientId = "type-0";

    ConnectionEnd baseConnection;

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
        doNothing().when(connectionSpy).sendBTPMessage(any(String.class), any(byte[].class));

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);
        proofHeight = Height.newBuilder()
                .setRevisionHeight(5)
                .setRevisionNumber(6).build();
        consensusHeight = Height.newBuilder()
                .setRevisionHeight(7)
                .setRevisionNumber(8).build();
        prefix = MerklePrefix.newBuilder()
                .setKeyPrefix(ByteString.copyFrom(IBCConnection.commitmentPrefix)).build();

        counterparty = Counterparty.newBuilder()
                .setClientId("counterpartyId")
                .setConnectionId("connectionId")
                .setPrefix(prefix).build();
        version = Version.newBuilder()
                .setIdentifier(IBCConnection.v1Identifier)
                .addAllFeatures(IBCConnection.supportedV1Features).build();
        baseConnection = ConnectionEnd.newBuilder()
                .setClientId(clientId)
                .addVersions(0, version)
                .setDelayPeriod(delayPeriod.longValue())
                .setCounterparty(counterparty).build();

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
                "_connectionOpenInit", msg);
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
                "_connectionOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutState);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenInit() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.toByteArray());
        msg.setDelayPeriod(delayPeriod);
        String expectedConnectionId = "connection-0";
        when(lightClient.mock.getClientState(msg.getClientId())).thenReturn(new byte[0]);

        // Act
        connection.invoke(owner, "_connectionOpenInit", msg);

        // Assert
        ConnectionEnd expectedConnection = ConnectionEnd.newBuilder(baseConnection)
                .setState(ConnectionEnd.State.STATE_INIT).build();

        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(expectedConnectionId);
        verify(connectionSpy)
                .sendBTPMessage(
                        clientId,
                        ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.toByteArray())));
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
                "_connectionOpenTry", msg);
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
                "_connectionOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                openConnectionWithoutVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void connectionOpenTry() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.toByteArray());
        msg.setDelayPeriod(delayPeriod);
        msg.setClientStateBytes(new byte[1]);
        msg.setCounterpartyVersions(new byte[][] { version.toByteArray() });
        msg.setProofInit(new byte[2]);
        msg.setProofClient(new byte[3]);
        msg.setProofConsensus(new byte[4]);
        msg.setProofHeight(proofHeight.toByteArray());
        msg.setConsensusHeight(consensusHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setClientId(msg.getClientId())
                .setConnectionId("")
                .setPrefix(prefix).build();

        ConnectionEnd counterpartyConnection = ConnectionEnd.newBuilder()
                .setClientId(counterparty.getClientId())
                .addVersions(0, version)
                .setState(ConnectionEnd.State.STATE_INIT)
                .setDelayPeriod(msg.getDelayPeriod().longValue())
                .setCounterparty(expectedCounterparty).build();

        String expectedConnectionId = "connection-0";

        // Act
        connection.invoke(owner, "_connectionOpenTry", msg);

        // Assert
        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(counterparty.getConnectionId());
        verify(lightClient.mock).verifyMembership(msg.getClientId(),
                msg.getProofHeight(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofInit(), prefix.getKeyPrefix().toByteArray(), connectionPath,
                counterpartyConnection.toByteArray());

        // verifyClientState
        byte[] clientStatePath = IBCCommitment.clientStatePath(counterparty.getClientId());
        verify(lightClient.mock).verifyMembership(msg.getClientId(), msg.getProofHeight(), BigInteger.ZERO,
                BigInteger.ZERO, msg.getProofClient(), prefix.getKeyPrefix().toByteArray(), clientStatePath,
                msg.getClientStateBytes());

        ConnectionEnd expectedConnection = ConnectionEnd.newBuilder(baseConnection)
                .setState(ConnectionEnd.State.STATE_TRYOPEN).build();

        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(expectedConnectionId);
        verify(connectionSpy)
                .sendBTPMessage(
                        clientId,
                        ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.toByteArray())));

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
                "_connectionOpenAck", msg);
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
        Version wrongVersion = Version.newBuilder()
                .setIdentifier("OtherVersion")
                .addFeatures("some features").build();
        msg.setVersion(wrongVersion.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection state is in TRYOPEN but the provided version is not set in the " +
                "previous connection versions";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "_connectionOpenAck", msg);
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
        msg.setVersion(version.toByteArray());
        msg.setCounterpartyConnectionID(counterparty.getClientId());
        msg.setProofTry(new byte[2]);
        msg.setProofClient(new byte[3]);
        msg.setProofConsensus(new byte[4]);
        msg.setProofHeight(proofHeight.toByteArray());
        msg.setConsensusHeight(consensusHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setClientId(clientId)
                .setConnectionId(msg.getConnectionId())
                .setPrefix(prefix).build();

        ConnectionEnd counterpartyConnection = ConnectionEnd.newBuilder()
                .setClientId(clientId)
                .addVersions(0, version)
                .setState(ConnectionEnd.State.STATE_TRYOPEN)
                .setDelayPeriod(delayPeriod.longValue())
                .setCounterparty(expectedCounterparty).build();

        // Act
        connection.invoke(owner, "_connectionOpenAck", msg);

        // Assert
        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(msg.getCounterpartyConnectionID());
        verify(lightClient.mock).verifyMembership(clientId, msg.getProofHeight(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofTry(), prefix.getKeyPrefix().toByteArray(), connectionPath,
                counterpartyConnection.toByteArray());

        // verifyClientState
        byte[] clientStatePath = IBCCommitment.clientStatePath(counterparty.getClientId());
        verify(lightClient.mock).verifyMembership(clientId, msg.getProofHeight(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofClient(), prefix.getKeyPrefix().toByteArray(), clientStatePath, msg.getClientStateBytes());

        Counterparty counterparty = Counterparty.newBuilder(baseConnection.getCounterparty())
                .setConnectionId(msg.getCounterpartyConnectionID()).build();
        ConnectionEnd expectedConnection = ConnectionEnd.newBuilder(baseConnection)
                .setState(ConnectionEnd.State.STATE_OPEN)
                .clearVersions()
                .addAllVersions(counterpartyConnection.getVersionsList())
                .setCounterparty(counterparty).build();

        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(msg.getConnectionId());
        verify(connectionSpy)
                .sendBTPMessage(
                        clientId,
                        ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.toByteArray())));

        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
    }

    @Test
    void connectionOpenConfirm_NotInTryOpen() {
        // Arrange
        connectionOpenInit();
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.setConnectionId("connection-0");
        msg.setProofAck(new byte[1]);
        msg.setProofHeight(proofHeight.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection state is not TRYOPEN";
        Executable clientVerificationFailed = () -> connection.invoke(owner,
                "_connectionOpenConfirm", msg);
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
        msg.setProofHeight(proofHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setClientId(clientId)
                .setConnectionId(msg.getConnectionId())
                .setPrefix(prefix).build();

        ConnectionEnd counterpartyConnection = ConnectionEnd.newBuilder()
                .setClientId(counterparty.getClientId())
                .addVersions(0, version)
                .setState(ConnectionEnd.State.STATE_OPEN)
                .setDelayPeriod(delayPeriod.longValue())
                .setCounterparty(expectedCounterparty).build();

        // Act
        connection.invoke(owner, "_connectionOpenConfirm", msg);

        // Assert
        // verifyConnectionState
        byte[] connectionPath = IBCCommitment.connectionPath(counterparty.getConnectionId());
        verify(lightClient.mock).verifyMembership(clientId, msg.getProofHeight(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofAck(), prefix.getKeyPrefix().toByteArray(), connectionPath,
                counterpartyConnection.toByteArray());

        ConnectionEnd expectedConnection = ConnectionEnd.newBuilder(baseConnection)
                .setState(ConnectionEnd.State.STATE_OPEN).build();

        byte[] connectionKey = IBCCommitment.connectionCommitmentKey(msg.getConnectionId());
        verify(connectionSpy)
                .sendBTPMessage(
                        clientId,
                        ByteUtil.join(connectionKey, IBCCommitment.keccak256(expectedConnection.toByteArray())));

        assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));

    }
}
