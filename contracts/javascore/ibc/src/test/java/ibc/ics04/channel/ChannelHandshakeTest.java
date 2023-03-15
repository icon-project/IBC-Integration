package ibc.ics04.channel;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.*;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Channel.Counterparty;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import score.Address;

import java.math.BigInteger;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.*;

public class ChannelHandshakeTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score channel;
    private MockContract<ILightClient> lightClient;
    private IBCChannelHandshake channelSpy;

    Height proofHeight = new Height();
    String clientId = "clientId";
    String connectionId = "connectionId";
    ConnectionEnd baseConnection = new ConnectionEnd();
    Channel baseChannel = new Channel();
    MerklePrefix prefix = new MerklePrefix();
    Version version = new Version();
    icon.proto.core.connection.Counterparty connectionCounterparty = new icon.proto.core.connection.Counterparty();
    Counterparty baseCounterparty = new Counterparty();
    String portId = "portId";
    String channelId = "channel-0";
    String channelVersion = IBCConnection.v1Identifier;

    public static class ChannelHandshakeMock extends IBCChannelHandshake {
        public ChannelHandshakeMock() {
        }

        public void setConnectionEnd(String connectionId, ConnectionEnd connectionEnd) {
            connections.set(connectionId, connectionEnd.encode());
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

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);

        prefix.setKeyPrefix(IBCConnection.commitmentPrefix);
        proofHeight.setRevisionHeight(BigInteger.valueOf(5));
        proofHeight.setRevisionNumber(BigInteger.valueOf(6));

        connectionCounterparty.setClientId(clientId);
        connectionCounterparty.setConnectionId("");
        connectionCounterparty.setPrefix(prefix);
        version.setIdentifier(IBCConnection.v1Identifier);
        version.setFeatures(IBCConnection.supportedV1Features);

        baseConnection.setClientId(clientId);
        baseConnection.setState(ConnectionEnd.State.STATE_OPEN);
        baseConnection.setCounterparty(connectionCounterparty);
        baseConnection.setDelayPeriod(BigInteger.ONE);
        baseConnection.setVersions(List.of(version));

        baseCounterparty.setPortId(portId);
        baseCounterparty.setChannelId(channelId);

        baseChannel.setState(Channel.State.STATE_INIT);
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        baseChannel.setCounterparty(baseCounterparty);
        baseChannel.setConnectionHops(List.of(connectionId));
        baseChannel.setVersion("v1");
        channel.invoke(owner, "setClient", clientId, lightClient.getAddress());
    }

    @Test
    void channelOpenInit_multipleHops() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel.setConnectionHops(List.of(connectionId, "otherId"));

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.encode());

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
        msg.setChannel(baseChannel.encode());

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
        baseConnection.setVersions(List.of(version, version));
        addConnection(connectionId, baseConnection);
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.encode());

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
        baseChannel.setState(Channel.State.STATE_OPEN);
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.encode());

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
        msg.setChannel(baseChannel.encode());

        // Act
        channel.invoke(owner, "channelOpenInit", msg);

        // Assert
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(msg.channel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(msg.getChannelRaw())));
        assertEquals(BigInteger.ONE, channel.call("getNextChannelSequence"));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceReceive", portId, channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceSend", portId, channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceAcknowledgement", portId, channelId));
    }

    @Test
    void channelOpenTry_multipleHops() {
        // Arrange
        baseChannel.setConnectionHops(List.of(connectionId, "otherId"));

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.encode());

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
        msg.setChannel(baseChannel.encode());

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
        baseConnection.setVersions(List.of(version, version));
        addConnection(connectionId, baseConnection);

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.encode());

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

        baseChannel.setState(Channel.State.STATE_INIT);
        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setChannel(baseChannel.encode());

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
        baseChannel.setState(Channel.State.STATE_TRYOPEN);

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.encode());
        msg.setCounterpartyVersion(channelVersion);
        msg.setProofHeight(proofHeight.encode());
        msg.setProofInit(new byte[1]);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_INIT);
        expectedChannel.setOrdering(msg.getChannel().getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(msg.getCounterpartyVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix(), IBCCommitment.channelPath(portId, channelId),
                expectedChannel.encode())).thenReturn(false);

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
        baseChannel.setState(Channel.State.STATE_TRYOPEN);

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setChannel(baseChannel.encode());
        msg.setCounterpartyVersion(channelVersion);
        msg.setProofHeight(proofHeight.encode());
        msg.setProofInit(new byte[1]);

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_INIT);
        expectedChannel.setOrdering(msg.getChannel().getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(msg.getCounterpartyVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix(), IBCCommitment.channelPath(portId, channelId),
                expectedChannel.encode())).thenReturn(true);
        // Act
        channel.invoke(owner, "channelOpenTry", msg);

        // Assert
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(msg.channel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(msg.getChannel().encode())));

        assertEquals(BigInteger.ONE, channel.call("getNextChannelSequence"));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceReceive", portId, channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceSend", portId, channelId));
        assertEquals(BigInteger.ONE, channel.call("getNextSequenceAcknowledgement", portId, channelId));
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
        msg.setProofHeight(proofHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel counterpartyChannel = new Channel();
        counterpartyChannel.setState(Channel.State.STATE_TRYOPEN);
        counterpartyChannel.setOrdering(baseChannel.getOrdering());
        counterpartyChannel.setCounterparty(expectedCounterparty);
        counterpartyChannel.setConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()));
        counterpartyChannel.setVersion(msg.getCounterpartyVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofTry(), prefix.getKeyPrefix(), IBCCommitment.channelPath(portId, channelId),
                counterpartyChannel.encode())).thenReturn(true);

        channel.invoke(owner, "channelOpenAck", msg);

        // Assert

        Channel expectedChannel = baseChannel;
        expectedChannel.setState(Channel.State.STATE_OPEN);
        expectedChannel.setVersion(msg.getCounterpartyVersion());
        expectedChannel.getCounterparty().setChannelId(msg.getCounterpartyChannelId());
        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(expectedChannel.encode())));

    }

    @Test
    void channelOpenConfirm() {
        // Arrange
        channelOpenTry();
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();

        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(proofHeight.encode());

        Channel.Counterparty expectedCounterparty = new Channel.Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel counterpartyChannel = new Channel();
        counterpartyChannel.setState(Channel.State.STATE_OPEN);
        counterpartyChannel.setOrdering(baseChannel.getOrdering());
        counterpartyChannel.setCounterparty(expectedCounterparty);
        counterpartyChannel.setConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()));
        counterpartyChannel.setVersion(baseChannel.getVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofAck(), prefix.getKeyPrefix(), IBCCommitment.channelPath(portId, channelId),
                counterpartyChannel.encode())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelOpenConfirm", msg);

        // Assert
        Channel expectedChannel = baseChannel;
        expectedChannel.setState(Channel.State.STATE_OPEN);

        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);
        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(expectedChannel.encode())));
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
        Channel expectedChannel = baseChannel;
        expectedChannel.setState(Channel.State.STATE_CLOSED);

        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(expectedChannel.encode())));
    }

    @Test
    void channelCloseConfirm() {
        // Arrange
        channelOpenConfirm();
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();

        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofInit(new byte[0]);
        msg.setProofHeight(proofHeight.encode());

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel counterpartyChannel = new Channel();
        counterpartyChannel.setState(Channel.State.STATE_CLOSED);
        counterpartyChannel.setOrdering(baseChannel.getOrdering());
        counterpartyChannel.setCounterparty(expectedCounterparty);
        counterpartyChannel.setConnectionHops(List.of(baseConnection.getCounterparty().getConnectionId()));
        counterpartyChannel.setVersion(baseChannel.getVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.getProofHeightRaw(), BigInteger.ZERO, BigInteger.ZERO,
                msg.getProofInit(), prefix.getKeyPrefix(), IBCCommitment.channelPath(portId, channelId),
                counterpartyChannel.encode())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelCloseConfirm", msg);

        // Assert
        Channel expectedChannel = baseChannel;
        expectedChannel.setState(Channel.State.STATE_CLOSED);

        byte[] key = IBCCommitment.channelCommitmentKey(portId, channelId);
        // byte[] storedCommitment = (byte[]) channel.call("getCommitment", key);
        // assertArrayEquals(IBCCommitment.keccak256(expectedChannel.toBytes()),
        // storedCommitment);
        verify(channelSpy).sendBTPMessage(ByteUtil.join(key, IBCCommitment.keccak256(expectedChannel.encode())));

    }

    private void addConnection(String connectionId, ConnectionEnd connectionEnd) {
        channel.invoke(owner, "setConnectionEnd", connectionId, connectionEnd);
    }

}
