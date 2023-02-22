package ibc.ics04.channel;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.when;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import org.mockito.stubbing.OngoingStubbing;

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
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Counterparty;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import score.Address;

public class PacketTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score packet;
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

    Height timeOutHeight = new Height();
    Packet basePacket = new Packet();

    public static class PacketMock extends IBCPacket {
        public PacketMock() {
        }

        public void setConnection(String connectionId, ConnectionEnd connectionEnd) {
            store.connections.set(connectionId, connectionEnd);
        }

        public void setClient(String clientId, Address client) {
            store.clientImpls.set(clientId, client);
        }

        public void setChannel(String portId, String channelId, Channel channel) {
            store.channels.at(portId).set(channelId, channel);
            store.nextSequenceSends.at(portId).set(channelId, BigInteger.ONE);
            store.nextSequenceRecvs.at(portId).set(channelId, BigInteger.ONE);
            store.nextSequenceAcks.at(portId).set(channelId, BigInteger.ONE);

        }
    }

    @BeforeEach
    public void setup() throws Exception {
        packet = sm.deploy(owner, PacketMock.class);

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

        baseChannel.setState(Channel.State.STATE_OPEN);
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        baseChannel.setCounterparty(baseCounterparty);
        baseChannel.setConnectionHops(new String[] { connectionId });
        baseChannel.setVersion("v1");

        packet.invoke(owner, "setClient", clientId, lightClient.getAddress());
        packet.invoke(owner, "setConnection", connectionId, baseConnection);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);

        basePacket.setData("data");
        basePacket.setSourcePort(portId);
        basePacket.setSourceChannel(channelId);
        basePacket.setDestinationPort(baseChannel.getCounterparty().getPortId());
        basePacket.setDestinationChannel(baseChannel.getCounterparty().getChannelId());
        basePacket.setSequence(BigInteger.ONE);
        timeOutHeight.setRevisionHeight(BigInteger.TEN.pow(30));
        timeOutHeight.setRevisionNumber(BigInteger.TEN.pow(30));
        basePacket.setTimeoutHeight(timeOutHeight);
        basePacket.setTimeoutTimestamp(BigInteger.TEN.pow(30));
    }

    @Test
    void sendPacket_nonOpenChannel() {
        // Arrange
        baseChannel.setState(Channel.State.STATE_CLOSED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);

        // Act & Assert
        String expectedErrorMessage = "channel state must be OPEN";
        Executable closedChannel = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                closedChannel);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_wrongPort() {
        // Arrange
        basePacket.setDestinationPort("other port");

        // Act & Assert
        String expectedErrorMessage = "packet destination port doesn't match the counterparty's port";
        Executable wrongPort = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                wrongPort);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_wrongChannelId() {
        // Arrange
        basePacket.setDestinationChannel("other channel id");

        // Act & Assert
        String expectedErrorMessage = "packet destination channel doesn't match the counterparty's channel";
        Executable wrongChannelId = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                wrongChannelId);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_lowTimeoutHeight() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.TEN);
        timeOutHeight.setRevisionNumber(BigInteger.ONE);
        basePacket.setTimeoutHeight(timeOutHeight);

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight);

        // Act & Assert
        String expectedErrorMessage = "receiving chain block height >= packet timeout height";
        Executable lowTimeoutHeight = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                lowTimeoutHeight);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_noConsensusState() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.ZERO);

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight);

        // Act & Assert
        String expectedErrorMessage = "consensusState not found";
        Executable noConsensusState = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                noConsensusState);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_toLowBlockTimestamp() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.ZERO);
        BigInteger destinationChainBlockTimestamp = BigInteger.TEN;
        basePacket.setTimeoutTimestamp(destinationChainBlockTimestamp.subtract(BigInteger.ONE));
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight);
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight)).thenReturn(destinationChainBlockTimestamp);

        // Act & Assert
        String expectedErrorMessage = "receiving chain block timestamp >= packet timeout timestamp";
        Executable toLowBlockTimestamp = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                toLowBlockTimestamp);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket_wrongSequence() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.ZERO);
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight);
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight)).thenReturn(BigInteger.ZERO);
        basePacket.setSequence(BigInteger.TEN);

        // Act & Assert
        String expectedErrorMessage = "packet sequence != next send sequence";
        Executable wrongSequence = () -> packet.invoke(owner, "sendPacket", basePacket);
        AssertionError e = assertThrows(AssertionError.class,
                wrongSequence);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void sendPacket() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.ZERO);
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight);
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight)).thenReturn(BigInteger.ZERO);

        // Act
        packet.invoke(owner, "sendPacket", basePacket);
        basePacket.setSequence(basePacket.getSequence().add(BigInteger.ONE));
        packet.invoke(owner, "sendPacket", basePacket);

        // Assert
        // TODO assert commitments
    }

    @Test
    void recvPacket_nonOpenChannel() {
        // Arrange
        baseChannel.setState(Channel.State.STATE_CLOSED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "channel state must be OPEN";
        Executable closedChannel = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                closedChannel);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_wrongPort() {
        // Arrange
        basePacket.setDestinationPort("other port");
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "packet destination port doesn't match the counterparty's port";
        Executable wrongPort = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                wrongPort);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_wrongChannelId() {
        // Arrange
        basePacket.setDestinationChannel("other channel id");
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "packet destination channel doesn't match the counterparty's channel";
        Executable wrongChannelId = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                wrongChannelId);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_nonOpenConnection() {
        // Arrange
        baseConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        packet.invoke(owner, "setConnection", connectionId, baseConnection);

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "connection state is not OPEN";
        Executable nonOpenConnection = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                nonOpenConnection);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_lowTimeoutHeight() {
        // Arrange
        timeOutHeight.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight()));
        basePacket.setTimeoutHeight(timeOutHeight);
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "block height >= packet timeout height";
        Executable lowTimeoutHeight = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                lowTimeoutHeight);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_lowTimeoutTimestamp() {
        // Arrange
        basePacket.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp()));
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;

        // Act & Assert
        String expectedErrorMessage = "block timestamp >= packet timeout timestamp";
        Executable lowTimeoutTimestamp = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                lowTimeoutTimestamp);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_doubleReceive_UnOrdered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_UNORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;
        msg.proof = new byte[1];
        msg.proofHeight = proofHeight;

        when(lightClient.mock.verifyMembership(clientId, proofHeight, baseConnection.getDelayPeriod(), BigInteger.ZERO,
                msg.proof, prefix.getKeyPrefix(), new byte[0], new byte[0])).thenReturn(true);

        packet.invoke(owner, "recvPacket", msg);

        // Act & Assert
        String expectedErrorMessage = "packet sequence already has been received";
        Executable alreadyReceived = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                alreadyReceived);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_outOfOrder_UnOrdered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_UNORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;
        msg.proof = new byte[1];
        msg.proofHeight = proofHeight;

        when(lightClient.mock.verifyMembership(clientId, proofHeight, baseConnection.getDelayPeriod(), BigInteger.ZERO,
                msg.proof, prefix.getKeyPrefix(), new byte[0], new byte[0])).thenReturn(true);

        // Act
        msg.packet.setSequence(BigInteger.TWO);
        packet.invoke(owner, "recvPacket", msg);
        // Assert
        msg.packet.setSequence(BigInteger.ONE);
        packet.invoke(owner, "recvPacket", msg);
    }

    @Test
    void recvPacket_futureReceive_Ordered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;
        msg.packet.setSequence(BigInteger.TWO);
        msg.proof = new byte[1];
        msg.proofHeight = proofHeight;

        when(lightClient.mock.verifyMembership(clientId, proofHeight, baseConnection.getDelayPeriod(), BigInteger.ZERO,
                msg.proof, prefix.getKeyPrefix(), new byte[0], new byte[0])).thenReturn(true);

        // Act & Assert
        String expectedErrorMessage = "packet sequence != next receive sequence";
        Executable notNext = () -> packet.invoke(owner, "recvPacket", msg);
        AssertionError e = assertThrows(AssertionError.class,
                notNext);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket() {
        // Arrange
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = basePacket;
        msg.proof = new byte[1];
        msg.proofHeight = proofHeight;

        when(lightClient.mock.verifyMembership(clientId, proofHeight, baseConnection.getDelayPeriod(), BigInteger.ZERO,
                msg.proof, prefix.getKeyPrefix(), new byte[0], new byte[0])).thenReturn(true);
        // Act
        packet.invoke(owner, "recvPacket", msg);

        // Assert
        // TODO assert storage
    }

    @Test
    void writeAcknowledgement() {
        // TODO: Wait for commitment work
    }

    @Test
    void acknowledgePacket() {
        // TODO: Wait for commitment work
    }
}
