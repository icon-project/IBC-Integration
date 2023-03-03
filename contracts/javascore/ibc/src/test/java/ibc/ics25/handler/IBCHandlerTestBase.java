package ibc.ics25.handler;

import static org.mockito.Mockito.when;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.any;
import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.verify;

import java.math.BigInteger;

import org.mockito.ArgumentCaptor;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.*;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Counterparty;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;

public class IBCHandlerTestBase extends TestBase {
    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account relayer = sm.createAccount();
    protected Score handler;
    protected IBCHandler handlerSpy;
    protected MockContract<ILightClient> lightClient;

    protected MockContract<IIBCModule> module;

    protected String clientType = "mockClient";

    ArgumentCaptor<String> clientIdCaptor = ArgumentCaptor.forClass(String.class);
    protected String clientId;

    ArgumentCaptor<String> connectionIdCaptor = ArgumentCaptor.forClass(String.class);
    protected String connectionId;

    ArgumentCaptor<String> channelIdCaptor = ArgumentCaptor.forClass(String.class);
    protected String channelId;

    ArgumentCaptor<byte[]> lastPacketCaptor = ArgumentCaptor.forClass(byte[].class);

    protected MerklePrefix prefix;
    protected String counterPartyClientId = "ICONClient";
    protected String counterPartyConnectionId = "connection-1";
    protected String counterPartyChannelId = "channel-1";
    protected String counterPartyPortId = "counterPartyPort";
    protected BigInteger delayPeriod = BigInteger.ONE;
    protected Version baseVersion = new Version();
    protected String portId = "portId";

    protected BigInteger nextRecvId = BigInteger.ONE;

    protected void setup() throws Exception {
        handler = sm.deploy(owner, IBCHandler.class);

        handlerSpy = (IBCHandler) spy(handler.getInstance());
        handler.setInstance(handlerSpy);

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);
        module = new MockContract<>(IIBCModuleScoreInterface.class, IIBCModule.class, sm, owner);

        when(lightClient.mock.verifyMembership(any(String.class), any(Height.class), any(BigInteger.class),
                any(BigInteger.class),
                any(byte[].class), any(String.class), any(byte[].class), any(byte[].class))).thenReturn(true);
        when(lightClient.mock.getClientState(any(String.class))).thenReturn(new byte[0]);

        prefix = new MerklePrefix();
        prefix.setKeyPrefix("ibc");

        baseVersion.identifier = IBCConnection.v1Identifier;
        baseVersion.features = IBCConnection.supportedV1Features;
    }

    void createClient() {
        // Arrange
        handler.invoke(owner, "registerClient", clientType, lightClient.getAddress());
        MsgCreateClient msg = new MsgCreateClient();
        msg.clientState = new byte[0];
        msg.consensusState = new byte[0];
        msg.clientType = clientType;

        ConsensusStateUpdate update = new ConsensusStateUpdate(new byte[0],
                new Height(BigInteger.ZERO, BigInteger.ZERO));
        UpdateClientResponse response = new UpdateClientResponse(new byte[0], update, true);
        when(lightClient.mock.createClient(any(String.class), any(byte[].class), any(byte[].class)))
                .thenReturn(response);

        // Act
        handler.invoke(module.account, "createClient", msg);

        // Assert
        verify(handlerSpy).GeneratedClientIdentifier(clientIdCaptor.capture());
        clientId = clientIdCaptor.getValue();
    }

    void updateClient() {
        // Arrange
        MsgUpdateClient msg = new MsgUpdateClient();
        msg.clientId = clientId;
        msg.clientMessage = new byte[4];

        byte[] clientStateCommitment = new byte[4];
        byte[] consensusStateCommitment = new byte[5];

        Height consensusHeight = new Height();
        consensusHeight.setRevisionHeight(BigInteger.ONE);
        consensusHeight.setRevisionNumber(BigInteger.TWO);

        ConsensusStateUpdate update = new ConsensusStateUpdate(consensusStateCommitment, consensusHeight);

        UpdateClientResponse response = new UpdateClientResponse(clientStateCommitment, update, true);

        when(lightClient.mock.updateClient(msg.clientId, msg.clientMessage)).thenReturn(response);

        // Act
        handler.invoke(relayer, "updateClient", msg);
    }

    void createConnection() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();

        Counterparty counterparty = new Counterparty();
        counterparty.setPrefix(prefix);
        counterparty.setClientId(counterPartyClientId);
        counterparty.setConnectionId("");
        msg.clientId = clientId;
        msg.counterparty = counterparty;
        msg.delayPeriod = delayPeriod;

        handler.invoke(module.account, "connectionOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void tryOpenConnection() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        msg.counterparty = new Counterparty();
        msg.counterparty.setClientId(counterPartyClientId);
        msg.counterparty.setConnectionId(counterPartyConnectionId);
        msg.counterparty.setPrefix(prefix);
        msg.delayPeriod = delayPeriod;
        msg.clientId = clientId;
        msg.clientStateBytes = new byte[0];
        msg.counterpartyVersions = new Version[] { baseVersion };
        msg.proofInit = new byte[0];
        msg.proofClient = new byte[0];
        msg.proofConsensus = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);
        msg.consensusHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        // Act
        handler.invoke(module.account, "connectionOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void acknowledgeConnection() {
        // Arrange
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.connectionId = connectionId;
        msg.clientStateBytes = new byte[0];
        msg.version = baseVersion;
        msg.counterpartyConnectionID = counterPartyConnectionId;
        msg.proofTry = new byte[0];
        msg.proofClient = new byte[0];
        msg.proofConsensus = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);
        msg.consensusHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        // Act
        handler.invoke(module.account, "connectionOpenAck", msg);

        // Assert
        ConnectionEnd connection = (ConnectionEnd) handler.call("getConnection", connectionId);
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.connectionState());

    }

    void confirmConnection() {
        // Arrange
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.connectionId = connectionId;
        msg.proofAck = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        // Act
        handler.invoke(module.account, "connectionOpenConfirm", msg);

        // Assert
        ConnectionEnd connection = (ConnectionEnd) handler.call("getConnection", connectionId);
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.connectionState());
    }

    void openChannel() {
        // Arrange
        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.portId = portId;
        msg.channel = new Channel();
        msg.channel.updateOrder(Channel.Order.ORDER_UNORDERED);
        msg.channel.updateState(Channel.State.STATE_INIT);
        msg.channel.setConnectionHops(new String[] { connectionId });
        msg.channel.setCounterparty(new ibc.icon.structs.proto.core.channel.Counterparty());
        msg.channel.getCounterparty().setPortId(counterPartyPortId);
        msg.channel.getCounterparty().setChannelId("");
        msg.channel.setVersion("");

        // Act
        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenInit(
                msg.channel.channelOrdering(),
                msg.channel.getConnectionHops(),
                msg.portId,
                channelId,
                msg.channel.getCounterparty(),
                msg.channel.getVersion());
    }

    void tryOpenChannel() {
        // Arrange
        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.portId = portId;
        msg.channel = new Channel();
        msg.channel.updateOrder(Channel.Order.ORDER_UNORDERED);
        msg.channel.updateState(Channel.State.STATE_TRYOPEN);
        msg.channel.setConnectionHops(new String[] { connectionId });
        msg.channel.setCounterparty(new ibc.icon.structs.proto.core.channel.Counterparty());
        msg.channel.getCounterparty().setPortId(counterPartyPortId);
        msg.channel.getCounterparty().setChannelId(counterPartyChannelId);
        msg.channel.setVersion("");
        msg.counterpartyVersion = baseVersion.identifier;
        msg.proofInit = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenTry(msg.channel.channelOrdering(), msg.channel.connectionHops, portId, channelId,
                msg.channel.counterparty, msg.channel.version, msg.counterpartyVersion);
    }

    void acknowledgeChannel() {
        // Arrange
        MsgChannelOpenAck msg = new MsgChannelOpenAck();
        msg.portId = portId;
        msg.channelId = channelId;
        msg.counterpartyVersion = IBCConnection.v1Identifier;
        msg.counterpartyChannelId = counterPartyChannelId;
        msg.proofTry = new byte[0];
        msg.proofHeight = new Height(BigInteger.ZERO, BigInteger.ZERO);

        // Act
        handler.invoke(module.account, "channelOpenAck", msg);

        // Assert
        Channel channel = (Channel) handler.call("getChannel", portId, channelId);
        assertEquals(Channel.State.STATE_OPEN, channel.channelState());

        verify(module.mock).onChanOpenAck(portId, channelId, msg.counterpartyVersion);
    }

    void confirmChannel() {
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();
        msg.portId = portId;
        msg.channelId = channelId;
        msg.proofAck = new byte[0];
        msg.proofHeight = new Height(BigInteger.ZERO, BigInteger.ZERO);

        // Act
        handler.invoke(module.account, "channelOpenConfirm", msg);

        // Assert
        Channel channel = (Channel) handler.call("getChannel", portId, channelId);
        assertEquals(Channel.State.STATE_OPEN, channel.channelState());

        verify(module.mock).onChanOpenConfirm(portId, channelId);
    }

    void closeChannel() {
        // Arrange
        MsgChannelCloseInit msg = new MsgChannelCloseInit();
        msg.channelId = channelId;
        msg.portId = portId;

        // Act
        handler.invoke(module.account, "channelCloseInit", msg);

        // Assert
        Channel channel = (Channel) handler.call("getChannel", portId, channelId);
        assertEquals(Channel.State.STATE_CLOSED, channel.channelState());

        verify(module.mock).onChanCloseInit(portId, channelId);
    }

    void confirmCloseChannel() {
        // Arrange
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();
        msg.channelId = channelId;
        msg.portId = portId;
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);
        msg.proofInit = new byte[1];

        // Act
        handler.invoke(module.account, "channelCloseConfirm", msg);

        // Assert
        Channel channel = (Channel) handler.call("getChannel", portId, channelId);
        assertEquals(Channel.State.STATE_CLOSED, channel.channelState());

        verify(module.mock).onChanCloseConfirm(portId, channelId);
    }

    void sendPacket() {
        // Arrange
        Packet packet = getBasePacket();

        // Act
        handler.invoke(module.account, "sendPacket", packet);

        // Assert
        verify(handlerSpy).SendPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toBytes(), lastPacketCaptor.getValue());
    }

    void receivePacket() {
        // Arrange
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = packet;
        msg.proof = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        when(module.mock.onRecvPacket(msg.packet, relayer.getAddress())).thenReturn(new byte[0]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toBytes(), lastPacketCaptor.getValue());

        verify(module.mock).onRecvPacket(msg.packet, relayer.getAddress());

    }

    void receivePacket_withAcK() {
        // Arrange
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.packet = packet;
        msg.proof = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        when(module.mock.onRecvPacket(msg.packet, relayer.getAddress())).thenReturn(new byte[1]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toBytes(), lastPacketCaptor.getValue());

        verify(handlerSpy).WriteAcknowledgement(packet.getDestinationPort(),
                packet.getDestinationChannel(), packet.getSequence(), new byte[1]);
    }

    void writeAcknowledgement() {
        // Arrange
        byte[] acknowledgement = new byte[1];
        Packet lastPacket = Packet.fromBytes(lastPacketCaptor.getValue());

        // Act
        handler.invoke(module.account, "writeAcknowledgement", lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), lastPacket.getSequence(), acknowledgement);

        // Assert
        verify(handlerSpy).WriteAcknowledgement(lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), lastPacket.getSequence(), acknowledgement);
    }

    void acknowledgePacket() {
        MsgPacketAcknowledgement msg = new MsgPacketAcknowledgement();
        msg.acknowledgement = new byte[1];
        msg.proof = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);
        msg.packet = Packet.fromBytes(lastPacketCaptor.getValue());

        // Act
        handler.invoke(relayer, "acknowledgePacket", msg);

        // Assert
        verify(handlerSpy).AcknowledgePacket(msg.packet.toBytes(), msg.acknowledgement);
        verify(module.mock).onAcknowledgementPacket(msg.packet, msg.acknowledgement, relayer.getAddress());
    }

    protected Packet getBasePacket() {
        Packet packet = new Packet();
        BigInteger nextPacketSeq = (BigInteger) handler.call("getNextSequenceSend", portId, channelId);
        packet.setSequence(nextPacketSeq);
        packet.setSourcePort(portId);
        packet.setSourceChannel(channelId);
        packet.setDestinationPort(counterPartyPortId);
        packet.setDestinationChannel(counterPartyChannelId);
        packet.setData("test");
        packet.setTimeoutHeight(new Height(BigInteger.ONE, BigInteger.valueOf(sm.getBlock().getHeight() + 100)));
        packet.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp() * 2));

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(new Height(BigInteger.ZERO, BigInteger.ZERO));
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(Height.class))).thenReturn(BigInteger.ONE);

        return packet;
    }

    protected Packet getBaseCounterPacket() {
        Packet packet = new Packet();
        packet.setSequence(nextRecvId);
        nextRecvId = nextRecvId.add(BigInteger.ONE);
        packet.setDestinationChannel(channelId);
        packet.setDestinationPort(portId);
        packet.setSourceChannel(counterPartyChannelId);
        packet.setSourcePort(counterPartyPortId);
        packet.setData("test");
        packet.setTimeoutHeight(new Height(BigInteger.ONE, BigInteger.valueOf(sm.getBlock().getHeight() + 100)));
        packet.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp() * 2));

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(new Height(BigInteger.ZERO, BigInteger.ZERO));
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(Height.class))).thenReturn(BigInteger.ONE);

        return packet;
    }
}
