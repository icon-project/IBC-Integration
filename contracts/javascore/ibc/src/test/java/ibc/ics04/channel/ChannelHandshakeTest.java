package ibc.ics04.channel;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.when;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import org.mockito.stubbing.OngoingStubbing;
import org.w3c.dom.css.Counter;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Counterparty;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import score.Address;

public class ChannelHandshakeTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score channel;
    private MockContract<ILightClient> lightClient;

    Height proofHeight = new Height();
    String clientId = "clientId";
    String connectionId = "connectionId";
    ConnectionEnd baseConnection = new ConnectionEnd();
    Channel baseChannel = new Channel();
    MerklePrefix prefix = new MerklePrefix();
    Version version = new Version();
    ibc.icon.structs.proto.core.connection.Counterparty connectionCounterparty = new ibc.icon.structs.proto.core.connection.Counterparty();
    Counterparty baseCounterparty = new Counterparty();
    String portId = "portId";
    String channelId = "channel-0";
    String channelVersion = IBCConnection.v1Identifier;

    public static class ChannelHandshakeMock extends IBCChannelHandshake {
        public ChannelHandshakeMock() {
        }

        public void setConnectionEnd(String connectionId, ConnectionEnd connectionEnd) {
            store.connections.set(connectionId, connectionEnd);
        }

        public void setClient(String clientId, Address client) {
            store.clientImpls.set(clientId, client);
        }
    }

    @BeforeEach
    public void setup() throws Exception {
        channel = sm.deploy(owner, ChannelHandshakeMock.class);

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);

        prefix.setKeyPrefix(IBCConnection.commitmentPrefix);
        proofHeight.revisionHeight = BigInteger.valueOf(5);
        proofHeight.revisionNumber = BigInteger.valueOf(6);

        connectionCounterparty.setClientId(clientId);
        connectionCounterparty.setConnectionId("");
        connectionCounterparty.setPrefix(prefix);
        version.identifier = IBCConnection.v1Identifier;
        version.features = IBCConnection.supportedV1Features;

        baseConnection.setClientId(clientId);
        baseConnection.setState(ConnectionEnd.State.STATE_OPEN);
        baseConnection.setCounterparty(connectionCounterparty);
        baseConnection.setDelayPeriod(BigInteger.ONE);
        baseConnection.setVersions(new Version[] { version });

        baseCounterparty.setPortId(portId);
        baseCounterparty.setChannelId(channelId);

        baseChannel.setState(Channel.State.STATE_INIT);
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        baseChannel.setCounterparty(baseCounterparty);
        baseChannel.setConnectionHops(new String[] { connectionId });
        baseChannel.setVersion("v1");
        channel.invoke(owner, "setClient", clientId, lightClient.getAddress());
    }

    @Test
    void channelOpenInit_multipleHops() {
        // Arrange
        addConnection(connectionId, baseConnection);
        baseChannel.setConnectionHops(new String[] { connectionId, "otherId" });

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.portId = portId;
        msg.channel = baseChannel;

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
        msg.portId = portId;
        msg.channel = baseChannel;

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
        baseConnection.setVersions(new Version[] { version, version });
        addConnection(connectionId, baseConnection);
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.portId = portId;
        msg.channel = baseChannel;

        // Act & Assert
        String expectedErrorMessage = "single version must be negotiated on connection before opening channel";
        Executable withoutNegotiatedVersion = () -> channel.invoke(owner,
                "channelOpenInit", msg);
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
        msg.portId = portId;
        msg.channel = baseChannel;

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
        msg.portId = portId;
        msg.channel = baseChannel;

        // Act
        channel.invoke(owner, "channelOpenInit", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    @Test
    void channelOpenTry_multipleHops() {
        // Arrange
        baseChannel.setConnectionHops(new String[] { connectionId, "otherId" });

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.channel = baseChannel;

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
        msg.channel = baseChannel;

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
        baseConnection.setVersions(new Version[] { version, version });
        addConnection(connectionId, baseConnection);

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.channel = baseChannel;

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
        msg.channel = baseChannel;

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
        msg.portId = portId;
        msg.channel = baseChannel;
        msg.counterpartyVersion = channelVersion;
        msg.proofHeight = proofHeight;
        msg.proofInit = new byte[1];

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_INIT);
        expectedChannel.setOrdering(msg.channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(new String[] { baseConnection.getCounterparty().getConnectionId() });
        expectedChannel.setVersion(msg.counterpartyVersion);

        when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
                msg.proofInit, prefix.getKeyPrefix(), new byte[0], expectedChannel.toBytes())).thenReturn(false);

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
        msg.portId = portId;
        msg.channel = baseChannel;
        msg.counterpartyVersion = channelVersion;
        msg.proofHeight = proofHeight;
        msg.proofInit = new byte[1];

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_INIT);
        expectedChannel.setOrdering(msg.channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(new String[] { baseConnection.getCounterparty().getConnectionId() });
        expectedChannel.setVersion(msg.counterpartyVersion);

        when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
                msg.proofInit, prefix.getKeyPrefix(), new byte[0], expectedChannel.toBytes())).thenReturn(true);
        // Act
        channel.invoke(owner, "channelOpenTry", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    @Test
    void channelOpenAck() {
        channelOpenInit();
        MsgChannelOpenAck msg = new MsgChannelOpenAck();
        msg.portId = portId;
        msg.channelId = channelId;
        msg.counterpartyVersion = "v1";
        msg.counterpartyChannelId = channelId;
        msg.proofTry = new byte[0];
        msg.proofHeight = proofHeight;

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_TRYOPEN);
        expectedChannel.setOrdering(baseChannel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(new String[] { baseConnection.getCounterparty().getConnectionId() });
        expectedChannel.setVersion(msg.counterpartyVersion);

        when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
                msg.proofTry, prefix.getKeyPrefix(), new byte[0], expectedChannel.toBytes())).thenReturn(true);

        channel.invoke(owner, "channelOpenAck", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    @Test
    void channelOpenConfirm() {
        // Arrange
        channelOpenTry();
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();

        msg.portId = portId;
        msg.channelId = channelId;
        msg.proofAck = new byte[0];
        msg.proofHeight = proofHeight;

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_OPEN);
        expectedChannel.setOrdering(baseChannel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(new String[] { baseConnection.getCounterparty().getConnectionId() });
        expectedChannel.setVersion(baseChannel.getVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
                msg.proofAck, prefix.getKeyPrefix(), new byte[0], expectedChannel.toBytes())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelOpenConfirm", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    @Test
    void channelCloseInit() {
        // Arrange
        channelOpenConfirm();
        MsgChannelCloseInit msg = new MsgChannelCloseInit();

        msg.portId = portId;
        msg.channelId = channelId;

        // Act
        channel.invoke(owner, "channelCloseInit", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    @Test
    void channelCloseConfirm() {
        // Arrange
        channelOpenConfirm();
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();

        msg.portId = portId;
        msg.channelId = channelId;
        msg.proofInit = new byte[0];
        msg.proofHeight = proofHeight;

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_CLOSED);
        expectedChannel.setOrdering(baseChannel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(new String[] { baseConnection.getCounterparty().getConnectionId() });
        expectedChannel.setVersion(baseChannel.getVersion());

        when(lightClient.mock.verifyMembership(clientId, msg.proofHeight, BigInteger.ZERO, BigInteger.ZERO,
                msg.proofInit, prefix.getKeyPrefix(), new byte[0], expectedChannel.toBytes())).thenReturn(true);

        // Act
        channel.invoke(owner, "channelCloseConfirm", msg);

        // Assert
        // TODO: assert storage
        // TODO: assert updated commitment
    }

    private void addConnection(String connectionId, ConnectionEnd connectionEnd) {
        channel.invoke(owner, "setConnectionEnd", connectionId, connectionEnd);
    }

}
