package ibc.ics03.connection;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.when;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Counterparty;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics24.host.IBCCommitment;
import score.Address;

public class ConnectionTest extends TestBase {
	private final ServiceManager sm = getServiceManager();
	private final Account owner = sm.createAccount();
	private Score connection;
	private MockContract<ILightClient> lightClient;

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

		lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);

		proofHeight.revisionHeight = BigInteger.valueOf(5);
		proofHeight.revisionNumber = BigInteger.valueOf(6);
		proofHeight.revisionHeight = BigInteger.valueOf(7);
		proofHeight.revisionNumber = BigInteger.valueOf(8);

		prefix.setKeyPrefix(IBCConnection.commitmentPrefix);

		counterparty.setClientId("counterpartyId");
		counterparty.setConnectionId("connectionId");
		counterparty.setPrefix(prefix);

		version.identifier = IBCConnection.v1Identifier;
		version.features = IBCConnection.supportedV1Features;

		baseConnection.setClientId(clientId);
		baseConnection.setVersions(new Version[] { version });
		baseConnection.setDelayPeriod(delayPeriod);
		baseConnection.setCounterparty(counterparty);

		connection.invoke(owner, "setClient", clientId, lightClient.getAddress());
	}

	@Test
	void connectionOpenInit_clientNotFound() {
		// Arrange
		MsgConnectionOpenInit msg = new MsgConnectionOpenInit();
		msg.clientId = "non existent";

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
		msg.clientId = clientId;

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
		msg.clientId = clientId;
		msg.counterparty = counterparty;
		msg.delayPeriod = delayPeriod;
		String expectedConnectionId = "connection-0";
		when(lightClient.mock.getClientState(msg.clientId)).thenReturn(new byte[0]);

		// Act
		connection.invoke(owner, "connectionOpenInit", msg);

		// Assert
		ConnectionEnd expectedConnection = baseConnection;
		expectedConnection.setState(ConnectionEnd.State.STATE_INIT);

		byte[] storedCommitment = (byte[]) connection.call("getCommitment",
				IBCCommitment.connectionCommitmentKey(expectedConnectionId));
		assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()), storedCommitment);
		assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
	}

	@Test
	void connectionOpenTry_MissingVersion() {
		// Arrange
		MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
		msg.counterpartyVersions = new Version[] {};

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
		msg.counterpartyVersions = new Version[] {};

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
		msg.clientId = clientId;
		msg.counterparty = counterparty;
		msg.delayPeriod = delayPeriod;
		msg.clientStateBytes = new byte[1];
		msg.counterpartyVersions = new Version[] { version };
		msg.proofInit = new byte[2];
		msg.proofClient = new byte[3];
		msg.proofConsensus = new byte[4];
		msg.proofHeight = proofHeight;
		msg.consensusHeight = consensusHeight;

		Counterparty expectedCounterparty = new Counterparty();
		expectedCounterparty.setClientId(msg.clientId);
		expectedCounterparty.setConnectionId("");
		expectedCounterparty.setPrefix(prefix);

		ConnectionEnd counterpartyConnection = new ConnectionEnd();
		counterpartyConnection.setClientId(counterparty.getClientId());
		counterpartyConnection.setVersions(new Version[] { version });
		counterpartyConnection.setState(ConnectionEnd.State.STATE_INIT);
		counterpartyConnection.setDelayPeriod(msg.delayPeriod);
		counterpartyConnection.setCounterparty(expectedCounterparty);

		// verifyConnectionState
		byte[] connectionPath = IBCCommitment.connectionPath(msg.counterparty.getConnectionId());
		when(lightClient.mock.verifyMembership(msg.clientId,
				msg.proofHeight, BigInteger.ZERO,
				BigInteger.ZERO, msg.proofInit, prefix.getKeyPrefix(), connectionPath,
				counterpartyConnection.toBytes()))
				.thenReturn(false).thenReturn(true);

		// verifyClientState
		byte[] clientStatePath = IBCCommitment.clientStatePath(msg.counterparty.getClientId());
		when(lightClient.mock.verifyMembership(msg.clientId, msg.proofHeight, BigInteger.ZERO,
				BigInteger.ZERO, msg.proofClient, prefix.getKeyPrefix(), clientStatePath,
				msg.clientStateBytes))
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
		msg.clientId = clientId;
		msg.counterparty = counterparty;
		msg.delayPeriod = delayPeriod;
		msg.clientStateBytes = new byte[1];
		msg.counterpartyVersions = new Version[] { version };
		msg.proofInit = new byte[2];
		msg.proofClient = new byte[3];
		msg.proofConsensus = new byte[4];
		msg.proofHeight = proofHeight;
		msg.consensusHeight = consensusHeight;

		Counterparty expectedCounterparty = new Counterparty();
		expectedCounterparty.setClientId(msg.clientId);
		expectedCounterparty.setConnectionId("");
		expectedCounterparty.setPrefix(prefix);

		ConnectionEnd counterpartyConnection = new ConnectionEnd();
		counterpartyConnection.setClientId(counterparty.getClientId());
		counterpartyConnection.setVersions(new Version[] { version });
		counterpartyConnection.setState(ConnectionEnd.State.STATE_INIT);
		counterpartyConnection.setDelayPeriod(msg.delayPeriod);
		counterpartyConnection.setCounterparty(expectedCounterparty);

		String expectedConnectionId = "connection-0";

		// verifyConnectionState
		byte[] connectionPath = IBCCommitment.connectionPath(msg.counterparty.getConnectionId());
		when(lightClient.mock.verifyMembership(msg.clientId,
				msg.proofHeight, BigInteger.ZERO,
				BigInteger.ZERO, msg.proofInit, prefix.getKeyPrefix(), connectionPath,
				counterpartyConnection.toBytes())).thenReturn(true);

		// verifyClientState
		byte[] clientStatePath = IBCCommitment.clientStatePath(msg.counterparty.getClientId());
		when(lightClient.mock.verifyMembership(msg.clientId, msg.proofHeight, BigInteger.ZERO,
				BigInteger.ZERO, msg.proofClient, prefix.getKeyPrefix(), clientStatePath,
				msg.clientStateBytes))
				.thenReturn(true);

		// Act
		connection.invoke(owner, "connectionOpenTry", msg);

		// Assert
		ConnectionEnd expectedConnection = baseConnection;
		expectedConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);

		byte[] storedCommitment = (byte[]) connection.call("getCommitment",
				IBCCommitment.connectionCommitmentKey(expectedConnectionId));
		assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()), storedCommitment);
		assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
	}

	@Test
	void connectionOpenAck_alreadyOpen() {
		// Arrange
		connectionOpenConfirm();
		MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
		msg.connectionId = "connection-0";

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
		msg.connectionId = "connection-0";
		Version wrongVersion = new Version();
		wrongVersion.identifier = "OtherVersion";
		wrongVersion.features = new String[] { "some features" };
		msg.version = wrongVersion;

		// Act & Assert
		String expectedErrorMessage = "connection state is in TRYOPEN but the provided version is not set in the previous connection versions";
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
		msg.connectionId = "connection-0";
		msg.clientStateBytes = new byte[1];
		msg.version = version;
		msg.counterpartyConnectionID = counterparty.clientId;
		msg.proofTry = new byte[2];
		msg.proofClient = new byte[3];
		msg.proofConsensus = new byte[4];
		msg.proofHeight = proofHeight;
		msg.consensusHeight = consensusHeight;

		Counterparty expectedCounterparty = new Counterparty();
		expectedCounterparty.setClientId(clientId);
		expectedCounterparty.setConnectionId(msg.connectionId);
		expectedCounterparty.setPrefix(prefix);

		ConnectionEnd counterpartyConnection = new ConnectionEnd();
		counterpartyConnection.setClientId(clientId);
		counterpartyConnection.setVersions(new Version[] { version });
		counterpartyConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
		counterpartyConnection.setDelayPeriod(delayPeriod);
		counterpartyConnection.setCounterparty(expectedCounterparty);

		// verifyConnectionState
		byte[] connectionPath = IBCCommitment.connectionPath(msg.counterpartyConnectionID);
		when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
				msg.proofTry, prefix.getKeyPrefix(), connectionPath, counterpartyConnection.toBytes()))
				.thenReturn(true);

		// verifyClientState
		byte[] clientStatePath = IBCCommitment.clientStatePath(counterparty.clientId);
		when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
				msg.proofClient, prefix.getKeyPrefix(), clientStatePath, msg.clientStateBytes)).thenReturn(true);

		// Act
		connection.invoke(owner, "connectionOpenAck", msg);

		// Assert
		ConnectionEnd expectedConnection = baseConnection;
		expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
		expectedConnection.setVersions(counterpartyConnection.versions);
		expectedConnection.counterparty.setConnectionId(msg.counterpartyConnectionID);
		byte[] storedCommitment = (byte[]) connection.call("getCommitment",
				IBCCommitment.connectionCommitmentKey(msg.connectionId));
		assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()), storedCommitment);
		assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));
	}

	@Test
	void connectionOpenConfirm_NotInTryOpen() {
		// Arrange
		connectionOpenInit();
		MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
		msg.connectionId = "connection-0";
		msg.proofAck = new byte[1];
		msg.proofHeight = proofHeight;

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
		msg.connectionId = "connection-0";
		msg.proofAck = new byte[1];
		msg.proofHeight = proofHeight;

		Counterparty expectedCounterparty = new Counterparty();
		expectedCounterparty.setClientId(clientId);
		expectedCounterparty.setConnectionId(msg.connectionId);
		expectedCounterparty.setPrefix(prefix);

		ConnectionEnd counterpartyConnection = new ConnectionEnd();
		counterpartyConnection.setClientId(counterparty.getClientId());
		counterpartyConnection.setVersions(new Version[] { version });
		counterpartyConnection.setState(ConnectionEnd.State.STATE_OPEN);
		counterpartyConnection.setDelayPeriod(delayPeriod);
		counterpartyConnection.setCounterparty(expectedCounterparty);

		// verifyConnectionState
		byte[] connectionPath = IBCCommitment.connectionPath(counterparty.connectionId);
		when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
				msg.proofAck, prefix.getKeyPrefix(), connectionPath, counterpartyConnection.toBytes()))
				.thenReturn(true);

		// Act
		connection.invoke(owner, "connectionOpenConfirm", msg);

		// Assert
		ConnectionEnd expectedConnection = baseConnection;
		expectedConnection.setState(ConnectionEnd.State.STATE_OPEN);
		byte[] storedCommitment = (byte[]) connection.call("getCommitment",
				IBCCommitment.connectionCommitmentKey(msg.connectionId));
		assertArrayEquals(IBCCommitment.keccak256(expectedConnection.toBytes()), storedCommitment);
		assertEquals(BigInteger.ONE, connection.call("getNextConnectionSequence"));

	}
}
