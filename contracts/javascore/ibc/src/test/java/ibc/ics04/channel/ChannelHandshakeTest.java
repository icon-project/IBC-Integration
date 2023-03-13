package ibc.ics04.channel;

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

import com.google.protobuf.ByteString;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import score.Address;
import test.proto.core.channel.ChannelOuterClass.Channel;
import test.proto.core.channel.ChannelOuterClass.Channel.Counterparty;
import test.proto.core.client.Client.Height;
import test.proto.core.connection.Connection;
import test.proto.core.connection.Connection.ConnectionEnd;
import test.proto.core.connection.Connection.MerklePrefix;
import test.proto.core.connection.Connection.Version;

public class ChannelHandshakeTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score channel;
    private MockContract<ILightClient> lightClient;
    private IBCChannelHandshake channelSpy;

    Height proofHeight;
    String clientId = "clientId";
    String connectionId = "connectionId";
    ConnectionEnd baseConnection;
    Channel baseChannel;
    MerklePrefix prefix;
    Version version;
    Connection.Counterparty connectionCounterparty;
    Counterparty baseCounterparty;
    String portId = "portId";
    String channelId = "channel-0";
    String channelVersion = IBCConnection.v1Identifier;

    public static class ChannelHandshakeMock extends IBCChannelHandshake {
        public ChannelHandshakeMock() {
        }

        public void setConnectionEnd(String connectionId, ConnectionEnd connectionEnd) {
            connections.set(connectionId, connectionEnd.toByteArray());
        }

        public void setClient(String clientId, Address client) {
            clientImplementations.set(clientId, client);
        }
    }

    @BeforeEach
    public void setup() throws Exception {
        channel = sm.deploy(owner, ChannelHandshakeMock.class);
        channelSpy = (IBCChannelHandshake) spy(channel.getInstance());
        channel.setInstance(channelSpy);
        doNothing().when(channelSpy).sendBTPMessage(any(byte[].class));

        lightClient = new MockContract<>(ILightClientScoreInterface.class,
                ILightClient.class, sm, owner);

        proofHeight = Height.newBuilder()
                .setRevisionHeight(5)
                .setRevisionNumber(6).build();

        prefix = MerklePrefix.newBuilder()
                .setKeyPrefix(ByteString.copyFrom(IBCConnection.commitmentPrefix)).build();

        connectionCounterparty = Connection.Counterparty.newBuilder()
                .setClientId(clientId)
                .setConnectionId("counterpartyId")
                .setPrefix(prefix).build();

        version = Version.newBuilder()
                .setIdentifier(IBCConnection.v1Identifier)
                .addAllFeatures(IBCConnection.supportedV1Features).build();

        baseConnection = ConnectionEnd.newBuilder()
                .setClientId(clientId)
                .setState(ConnectionEnd.State.STATE_OPEN)
                .setCounterparty(connectionCounterparty)
                .setDelayPeriod(1)
                .addAllVersions(List.of(version)).build();

        baseCounterparty = Counterparty.newBuilder()
                .setPortId(portId)
                .setChannelId(channelId).build();

        baseChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_INIT)
                .setOrdering(Channel.Order.ORDER_ORDERED)
                .setCounterparty(baseCounterparty)
                .addAllConnectionHops(List.of(connectionId))
                .setVersion("v1").build();
        channel.invoke(owner, "setClient", clientId, lightClient.getAddress());
    }

    @Test
    void channelOpenInit_multipleHops() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel = Channel.newBuilder(baseChannel)
                .clearConnectionHops()
                .addAllConnectionHops(List.of(connectionId, "otherId")).build();

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection_hops length must be 1";
        Executable multiHopChannel = () -> channel.invoke(owner,
                "channelOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                multiHopChannel);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenInit_noConnection() {
        // Arrange
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection does not exist";
        Executable withoutConnection = () -> channel.invoke(owner,
                "channelOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                withoutConnection);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenInit_inconsistentVersion() {
        // Arrange
        baseConnection = ConnectionEnd.newBuilder(baseConnection)
                .clearVersions()
                .addAllVersions(List.of(version, version)).build();
        addConnection(connectionId, baseConnection);
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "single version must be negotiated on connection before opening channel";
        Executable withoutNegotiatedVersion = () -> channel.invoke(owner, "channelOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                withoutNegotiatedVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenInit_wrongState() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_OPEN).build();
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "channel state must be STATE_INIT";
        Executable withoutNegotiatedVersion = () -> channel.invoke(owner,
                "channelOpenInit", msg);
        AssertionError e = assertThrows(AssertionError.class,
                withoutNegotiatedVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenInit() {
        // Arrange
        addConnection(connectionId, baseConnection);

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());

        // Act
        channel.invoke(owner, "channelOpenInit", msg);

        // Assert
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(msg.channel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(msg.getChannelRaw())));
        assertEquals(BigInteger.ONE, channel.call("getNextChannelSequence"));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceReceive", portId,
                channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceSend", portId,
                channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceAcknowledgement",
                portId, channelId));
    }

    @Test
    void channelOpenTry_multipleHops() {
        // Arrange
        baseChannel = Channel.newBuilder(baseChannel)
                .clearConnectionHops()
                .addAllConnectionHops(List.of(connectionId, "otherId")).build();

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection_hops length must be 1";
        Executable multiHop = () -> channel.invoke(owner, "channelOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                multiHop);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenTry_noConnection() {
        // Arrange
        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "connection does not exist";
        Executable noConnection = () -> channel.invoke(owner, "channelOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                noConnection);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenTry_inconsistentVersion() {
        // Arrange
        baseConnection = ConnectionEnd.newBuilder(baseConnection)
                .clearVersions()
                .addAllVersions(List.of(version, version)).build();
        addConnection(connectionId, baseConnection);

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "single version must be negotiated on connection before opening channel";
        Executable inconsistentVersion = () -> channel.invoke(owner, "channelOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                inconsistentVersion);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenTry_wrongState() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_INIT).build();
        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.toByteArray());

        // Act & Assert
        String expectedErrorMessage = "channel state must be STATE_TRYOPEN";
        Executable wrongState = () -> channel.invoke(owner, "channelOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                wrongState);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenTry_failedVerification() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_TRYOPEN).build();

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());
        msg.setCounterpartyVersion(channelVersion);
        msg.setProofHeight(proofHeight.toByteArray());
        msg.setProofInit(new byte[1]);

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setPortId(msg.getPortId())
                .setChannelId("").build();

        Channel expectedChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_INIT)
                .setOrderingValue(msg.getChannel().getOrdering())
                .setCounterparty(expectedCounterparty)
                .addAllConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()))
                .setVersion(msg.getCounterpartyVersion()).build();

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(),
                BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix().toByteArray(), IBCCommitment.channelPath(portId,
                        channelId),
                expectedChannel.toByteArray())).thenReturn(false);

        // Act & Assert
        String expectedErrorMessage = "failed to verify channel state";
        Executable wrongState = () -> channel.invoke(owner, "channelOpenTry", msg);
        AssertionError e = assertThrows(AssertionError.class,
                wrongState);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void channelOpenTry() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_TRYOPEN).build();

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.toByteArray());
        msg.setCounterpartyVersion(channelVersion);
        msg.setProofHeight(proofHeight.toByteArray());
        msg.setProofInit(new byte[1]);

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setPortId(msg.getPortId())
                .setChannelId("").build();

        Channel expectedChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_INIT)
                .setOrdering(baseChannel.getOrdering())
                .setCounterparty(expectedCounterparty)
                .addAllConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()))
                .setVersion(msg.getCounterpartyVersion()).build();

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(),
                BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix().toByteArray(),
                IBCCommitment.channelPath(portId, channelId), expectedChannel.toByteArray()))
                .thenReturn(true);
        // Act
        channel.invoke(owner, "channelOpenTry", msg);

        // Assert
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(msg.channel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(baseChannel.toByteArray())));

        assertEquals(BigInteger.ONE, channel.call("getNextChannelSequence"));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceReceive", portId,
                channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceSend", portId,
                channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceAcknowledgement",
                portId, channelId));
    }

    @Test
    void channelOpenAck() {
        channelOpenInit();
        MsgChannelOpenAck msg = new MsgChannelOpenAck();
        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setCounterpartyVersion("v1");
        msg.setCounterpartyChannelId(channelId);
        msg.setProofTry(new byte[0]);
        msg.setProofHeight(proofHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setPortId(msg.getPortId())
                .setChannelId(msg.getChannelId()).build();

        Channel counterpartyChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_TRYOPEN)
                .setOrdering(baseChannel.getOrdering())
                .setCounterparty(expectedCounterparty)
                .addAllConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()))
                .setVersion(msg.getCounterpartyVersion()).build();

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(),
                BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofTry(), prefix.getKeyPrefix().toByteArray(), IBCCommitment.channelPath(portId,
                        channelId),
                counterpartyChannel.toByteArray())).thenReturn(true);

        channel.invoke(owner, "channelOpenAck", msg);

        // Assert
        Counterparty counterparty = Counterparty.newBuilder(baseChannel.getCounterparty())
                .setChannelId(msg.getCounterpartyChannelId()).build();
        Channel expectedChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_OPEN)
                .setVersion(msg.getCounterpartyVersion())
                .setCounterparty(counterparty).build();
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(expectedChannel.toByteArray())));

    }

    @Test
    void channelOpenConfirm() {
        // Arrange
        channelOpenTry();
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();

        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(proofHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setPortId(msg.getPortId())
                .setChannelId(msg.getChannelId()).build();

        Channel counterpartyChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_OPEN)
                .setOrdering(baseChannel.getOrdering())
                .setCounterparty(expectedCounterparty)
                .addAllConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()))
                .setVersion(baseChannel.getVersion()).build();

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(),
                BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofAck(), prefix.getKeyPrefix().toByteArray(), IBCCommitment.channelPath(portId, channelId),
                counterpartyChannel.toByteArray())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelOpenConfirm", msg);

        // Assert
        Channel expectedChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_OPEN).build();

        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);
        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(expectedChannel.toByteArray())));
    }

    @Test
    void channelCloseInit() {
        // Arrange
        channelOpenConfirm();
        MsgChannelCloseInit msg = new MsgChannelCloseInit();

        msg.setPortId(portId);
        msg.setChannelId(channelId);

        // Act
        channel.invoke(owner, "channelCloseInit", msg);

        // Assert
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);
        Channel expectedChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_CLOSED).build();

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(expectedChannel.toByteArray())));
    }

    @Test
    void channelCloseConfirm() {
        // Arrange
        channelOpenConfirm();
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();

        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofInit(new byte[0]);
        msg.setProofHeight(proofHeight.toByteArray());

        Counterparty expectedCounterparty = Counterparty.newBuilder()
                .setPortId(msg.getPortId())
                .setChannelId(msg.getChannelId()).build();

        Channel counterpartyChannel = Channel.newBuilder()
                .setState(Channel.State.STATE_CLOSED)
                .setOrdering(baseChannel.getOrdering())
                .setCounterparty(expectedCounterparty)
                .addAllConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()))
                .setVersion(baseChannel.getVersion()).build();

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(),
                BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix().toByteArray(), IBCCommitment.channelPath(portId,
                        channelId),
                counterpartyChannel.toByteArray())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelCloseConfirm", msg);

        // Assert
        Channel expectedChannel = Channel.newBuilder(baseChannel)
                .setState(Channel.State.STATE_CLOSED).build();

        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);
        verify(channelSpy).sendBTPMessage(ByteUtil.join(key,
                IBCCommitment.keccak256(expectedChannel.toByteArray())));

    }

    private void addConnection(String connectionId, ConnectionEnd connectionEnd) {
        channel.invoke(owner, "setConnectionEnd", connectionId, connectionEnd);
    }

}
