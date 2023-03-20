package ibc.ics04.channel;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.Version;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import score.Address;

import java.math.BigInteger;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.*;

public class PacketTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score packet;
    private MockContract<ILightClient> lightClient;
    private IBCPacket packetSpy;

    Height proofHeight = new Height();
    String clientId = "clientId";
    String connectionId = "connectionId";
    ConnectionEnd baseConnection = new ConnectionEnd();
    Channel baseChannel = new Channel();
    MerklePrefix prefix = new MerklePrefix();
    Version version = new Version();
    Counterparty connectionCounterparty = new Counterparty();
    Channel.Counterparty baseCounterparty = new Channel.Counterparty();
    String portId = "portId";
    String channelId = "channel-0";
    String channelVersion = IBCConnection.v1Identifier;

    Height timeOutHeight = new Height();
    Packet basePacket = new Packet();

    public static class PacketMock extends IBCPacket {
        public PacketMock() {
        }

        public void setConnection(String connectionId, ConnectionEnd connectionEnd) {
            connections.set(connectionId, connectionEnd.encode());
        }

        public void setClient(String clientId, Address client) {
            clientImplementations.set(clientId, client);
        }

        public void setChannel(String portId, String channelId, Channel channel) {
            channels.at(portId).set(channelId, channel.encode());
            nextSequenceSends.at(portId).set(channelId, BigInteger.ONE);
            nextSequenceReceives.at(portId).set(channelId, BigInteger.ONE);
            nextSequenceAcknowledgements.at(portId).set(channelId, BigInteger.ONE);

        }
    }

    @BeforeEach
    public void setup() throws Exception {
        packet = sm.deploy(owner, PacketMock.class);
        packetSpy = (IBCPacket) spy(packet.getInstance());
        packet.setInstance(packetSpy);
        doNothing().when(packetSpy).sendBTPMessage(any(byte[].class));

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

        baseChannel.setState(Channel.State.STATE_OPEN);
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        baseChannel.setCounterparty(baseCounterparty);
        baseChannel.setConnectionHops(List.of(connectionId));
        baseChannel.setVersion("v1");

        packet.invoke(owner, "setClient", clientId, lightClient.getAddress());
        packet.invoke(owner, "setConnection", connectionId, baseConnection);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);

        basePacket.setData(new byte[10]);
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

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());

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

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());

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
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight.encode()))
                .thenReturn(destinationChainBlockTimestamp);

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
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight.encode())).thenReturn(BigInteger.ZERO);
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
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight.encode())).thenReturn(BigInteger.ZERO);

        // Act
        packet.invoke(owner, "sendPacket", basePacket);
        basePacket.setSequence(BigInteger.TWO);
        packet.invoke(owner, "sendPacket", basePacket);

        // Assert
        byte[] key1 = IBCCommitment.packetCommitmentKey(basePacket.getSourcePort(),
                basePacket.getSourceChannel(),
                BigInteger.ONE);
        byte[] key2 = IBCCommitment.packetCommitmentKey(basePacket.getSourcePort(),
                basePacket.getSourceChannel(),
                basePacket.getSequence());

        byte[] storedCommitment1 = (byte[]) packet.call("getCommitment", key1);
        byte[] storedCommitment2 = (byte[]) packet.call("getCommitment", key2);

        byte[] expectedCommitment = createPacketCommitment(basePacket);
        assertArrayEquals(expectedCommitment, storedCommitment1);
        assertArrayEquals(expectedCommitment, storedCommitment2);
        verify(packetSpy).sendBTPMessage(ByteUtil.join(key1, expectedCommitment));
        verify(packetSpy).sendBTPMessage(ByteUtil.join(key2, expectedCommitment));
        assertEquals(BigInteger.valueOf(3),
                packet.call("getNextSequenceSend", basePacket.getSourcePort(), basePacket.getSourceChannel()));
    }

    @Test
    void recvPacket_nonOpenChannel() {
        // Arrange
        baseChannel.setState(Channel.State.STATE_CLOSED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);

        // Act & Assert
        String expectedErrorMessage = "channel state must be OPEN";
        Executable closedChannel = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                closedChannel);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_wrongPort() {
        // Arrange
        basePacket.setSourcePort("other port");

        // Act & Assert
        String expectedErrorMessage = "packet destination port doesn't match the counterparty's port";
        Executable wrongPort = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                wrongPort);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_wrongChannelId() {
        // Arrange
        basePacket.setSourceChannel("other channel id");

        // Act & Assert
        String expectedErrorMessage = "packet destination channel doesn't match the counterparty's channel";
        Executable wrongChannelId = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                wrongChannelId);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_nonOpenConnection() {
        // Arrange
        baseConnection.setState(ConnectionEnd.State.STATE_TRYOPEN);
        packet.invoke(owner, "setConnection", connectionId, baseConnection);

        // Act & Assert
        String expectedErrorMessage = "connection state is not OPEN";
        Executable nonOpenConnection = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                nonOpenConnection);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_lowTimeoutHeight() {
        // Arrange
        timeOutHeight.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight()));
        basePacket.setTimeoutHeight(timeOutHeight);

        // Act & Assert
        String expectedErrorMessage = "block height >= packet timeout height";
        Executable lowTimeoutHeight = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                lowTimeoutHeight);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_lowTimeoutTimestamp() {
        // Arrange
        basePacket.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp()));

        // Act & Assert
        String expectedErrorMessage = "block timestamp >= packet timeout timestamp";
        Executable lowTimeoutTimestamp = () -> packet.invoke(owner, "recvPacket", basePacket, new byte[0], new byte[0]);
        AssertionError e = assertThrows(AssertionError.class,
                lowTimeoutTimestamp);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_doubleReceive_UnOrdered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_UNORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        byte[] proof = new byte[1];

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        byte[] commitmentBytes = createPacketCommitment(basePacket);

        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath, commitmentBytes)).thenReturn(true);

        packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());

        // Act & Assert
        String expectedErrorMessage = "packet sequence already has been received";
        Executable alreadyReceived = () -> packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());
        AssertionError e = assertThrows(AssertionError.class,
                alreadyReceived);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_outOfOrder_UnOrdered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_UNORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        byte[] proof = new byte[1];

        byte[] commitmentPath1 = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        byte[] commitmentPath2 = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence().add(BigInteger.ONE));
        byte[] commitmentBytes = createPacketCommitment(basePacket);

        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath1, commitmentBytes)).thenReturn(true);
        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath2, commitmentBytes)).thenReturn(true);
        // Act
        basePacket.setSequence(BigInteger.TWO);
        packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());
        // Assert
        basePacket.setSequence(BigInteger.ONE);
        packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());
    }

    @Test
    void recvPacket_futureReceive_Ordered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        basePacket.setSequence(BigInteger.TWO);
        byte[] proof = new byte[1];

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        byte[] commitmentBytes = createPacketCommitment(basePacket);

        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO, proof, prefix.getKeyPrefix(), commitmentPath, commitmentBytes)).thenReturn(true);

        // Act & Assert
        String expectedErrorMessage = "packet sequence != next receive sequence";
        Executable notNext = () -> packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());
        AssertionError e = assertThrows(AssertionError.class,
                notNext);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void recvPacket_UnOrdered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_UNORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        byte[] proof = new byte[1];

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        byte[] commitmentBytes = createPacketCommitment(basePacket);

        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath, commitmentBytes)).thenReturn(true);
        // Act
        packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());

        // Assert
        assertEquals(BigInteger.ONE, packet.call("getPacketReceipt", portId, channelId, basePacket.getSequence()));
    }

    @Test
    void recvPacket_Ordered() {
        // Arrange
        baseChannel.setOrdering(Channel.Order.ORDER_ORDERED);
        packet.invoke(owner, "setChannel", portId, channelId, baseChannel);
        byte[] proof = new byte[1];

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        byte[] commitmentBytes = createPacketCommitment(basePacket);

        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(), baseConnection.getDelayPeriod(),
                BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath, commitmentBytes)).thenReturn(true);
        // Act
        packet.invoke(owner, "recvPacket", basePacket, proof, proofHeight.encode());

        // Assert
        assertEquals(basePacket.getSequence().add(BigInteger.ONE),
                packet.call("getNextSequenceReceive", portId, channelId));
    }

    @Test
    void writeAcknowledgement() {
        // Arrange
        byte[] acknowledgement = new byte[5];
        BigInteger sequence = BigInteger.ONE;

        // Act
        packet.invoke(owner, "writeAcknowledgement", baseCounterparty.getPortId(), baseCounterparty.getChannelId(),
                sequence, acknowledgement);

        // Assert
        byte[] ackCommitmentKey = IBCCommitment.packetAcknowledgementCommitmentKey(baseCounterparty.getPortId(),
                baseCounterparty.getChannelId(), sequence);
        byte[] storedCommitment = (byte[]) packet.call("getCommitment", ackCommitmentKey);

        byte[] expectedCommitment = IBCCommitment.keccak256(IBCCommitment.sha256(acknowledgement));
        assertArrayEquals(expectedCommitment, storedCommitment);
        verify(packetSpy).sendBTPMessage(ByteUtil.join(ackCommitmentKey, expectedCommitment));
    }

    @Test
    void acknowledgePacket() {
        // Arrange
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.ZERO);
        latestHeight.setRevisionNumber(BigInteger.ZERO);
        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(latestHeight.encode());
        when(lightClient.mock.getTimestampAtHeight(clientId, latestHeight.encode())).thenReturn(BigInteger.ZERO);

        packet.invoke(owner, "sendPacket", basePacket);

        byte[] acknowledgement = new byte[4];
        byte[] proof = new byte[5];

        byte[] commitmentPath = IBCCommitment.packetAcknowledgementCommitmentPath(basePacket.getDestinationPort(),
                basePacket.getDestinationChannel(), basePacket.getSequence());
        when(lightClient.mock.verifyMembership(clientId, proofHeight.encode(),
                baseConnection.getDelayPeriod(), BigInteger.ZERO,
                proof, prefix.getKeyPrefix(), commitmentPath,
                IBCCommitment.sha256(acknowledgement))).thenReturn(true);

        // Act
        packet.invoke(owner, "acknowledgePacket", basePacket, acknowledgement, proof, proofHeight.encode());

        // Assert
        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(basePacket.getSourcePort(),
                basePacket.getSourceChannel(), basePacket.getSequence());
        Object storedCommitment = packet.call("getCommitment", packetCommitmentKey);
        assertNull(storedCommitment);
        assertEquals(BigInteger.TWO, packet.call("getNextSequenceAcknowledgement", basePacket.getSourcePort(),
                basePacket.getSourceChannel()));
    }

    private byte[] createPacketCommitment(Packet packet) {
        return IBCCommitment.keccak256(IBCCommitment.sha256(
                ByteUtil.join(
                        packet.getTimeoutTimestamp().toByteArray(),
                        packet.getTimeoutHeight().getRevisionNumber().toByteArray(),
                        packet.getTimeoutHeight().getRevisionHeight().toByteArray(),
                        packet.getData())));
    }
}
