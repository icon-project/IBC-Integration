package ibc.ics25.handler;

import com.google.protobuf.ByteString;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.*;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import org.mockito.ArgumentCaptor;
import test.proto.core.channel.ChannelOuterClass.Channel;
import test.proto.core.channel.ChannelOuterClass.Packet;
import test.proto.core.client.Client.Height;
import test.proto.core.connection.Connection.ConnectionEnd;
import test.proto.core.connection.Connection.Counterparty;
import test.proto.core.connection.Connection.MerklePrefix;
import test.proto.core.connection.Connection.Version;

import java.math.BigInteger;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.*;

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
    protected Version baseVersion;
    protected String portId = "portId";

    protected BigInteger nextRecvId = BigInteger.ONE;
    protected Height baseHeight;

    protected void setup() throws Exception {
        handler = sm.deploy(owner, IBCHandler.class);

        handlerSpy = (IBCHandler) spy(handler.getInstance());
        handler.setInstance(handlerSpy);
        doNothing().when(handlerSpy).sendBTPMessage(any(byte[].class));

        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);
        module = new MockContract<>(IIBCModuleScoreInterface.class, IIBCModule.class, sm, owner);

        when(lightClient.mock.verifyMembership(any(String.class), any(byte[].class), any(BigInteger.class),
                any(BigInteger.class),
                any(byte[].class), any(byte[].class), any(byte[].class), any(byte[].class))).thenReturn(true);
        when(lightClient.mock.getClientState(any(String.class))).thenReturn(new byte[0]);

        prefix = MerklePrefix.newBuilder()
                .setKeyPrefix(ByteString.copyFrom("ibc".getBytes())).build();
        baseVersion = Version.newBuilder()
                .setIdentifier(IBCConnection.v1Identifier)
                .addAllFeatures(IBCConnection.supportedV1Features).build();
        baseHeight = Height.newBuilder()
                .setRevisionHeight(1)
                .setRevisionNumber(1).build();
    }

    void createClient() {
        // Arrange
        handler.invoke(owner, "registerClient", clientType, lightClient.getAddress());
        MsgCreateClient msg = new MsgCreateClient();
        msg.setClientState(new byte[0]);
        msg.setConsensusState(new byte[0]);
        msg.setClientType(clientType);

        ConsensusStateUpdate update = new ConsensusStateUpdate(new byte[0],
                Height.getDefaultInstance().toByteArray());
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
        msg.setClientId(clientId);
        msg.setClientMessage(new byte[4]);

        byte[] clientStateCommitment = new byte[4];
        byte[] consensusStateCommitment = new byte[5];

        Height consensusHeight = Height.newBuilder()
                .setRevisionHeight(1)
                .setRevisionNumber(2).build();

        ConsensusStateUpdate update = new ConsensusStateUpdate(consensusStateCommitment, consensusHeight.toByteArray());

        UpdateClientResponse response = new UpdateClientResponse(clientStateCommitment, update, true);

        when(lightClient.mock.updateClient(msg.getClientId(), msg.getClientMessage())).thenReturn(response);

        // Act
        handler.invoke(relayer, "updateClient", msg);
    }

    void createConnection() {
        // Arrange
        MsgConnectionOpenInit msg = new MsgConnectionOpenInit();

        Counterparty counterparty = Counterparty.newBuilder()
                .setPrefix(prefix)
                .setClientId(counterPartyClientId)
                .setConnectionId("").build();
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.toByteArray());
        msg.setDelayPeriod(delayPeriod);

        handler.invoke(module.account, "connectionOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void tryOpenConnection() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        Counterparty counterparty = Counterparty.newBuilder()
                .setPrefix(prefix)
                .setClientId(counterPartyClientId)
                .setConnectionId(counterPartyConnectionId).build();
        msg.setCounterparty(counterparty.toByteArray());
        msg.setDelayPeriod(delayPeriod);
        msg.setClientId(clientId);
        msg.setClientStateBytes(new byte[0]);
        msg.setCounterpartyVersions(new byte[][]{baseVersion.toByteArray()});
        msg.setProofInit(new byte[0]);
        msg.setProofClient(new byte[0]);
        msg.setProofConsensus(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());
        msg.setConsensusHeight(baseHeight.toByteArray());

        // Act
        handler.invoke(module.account, "connectionOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void acknowledgeConnection() throws Exception {
        // Arrange
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.setConnectionId(connectionId);
        msg.setClientStateBytes(new byte[0]);
        msg.setVersion(baseVersion.toByteArray());
        msg.setCounterpartyConnectionID(counterPartyConnectionId);
        msg.setProofTry(new byte[0]);
        msg.setProofClient(new byte[0]);
        msg.setProofConsensus(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());
        msg.setConsensusHeight(baseHeight.toByteArray());

        // Act
        handler.invoke(module.account, "connectionOpenAck", msg);

        // Assert
        ConnectionEnd connection = ConnectionEnd.parseFrom((byte[]) handler.call("getConnection", connectionId));
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.getState());

    }

    void confirmConnection() throws Exception {
        // Arrange
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.setConnectionId(connectionId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());

        // Act
        handler.invoke(module.account, "connectionOpenConfirm", msg);

        // Assert
        ConnectionEnd connection = ConnectionEnd.parseFrom((byte[]) handler.call("getConnection", connectionId));
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.getState());
    }

    void openChannel() {
        // Arrange
        Channel.Counterparty counterparty = Channel.Counterparty.newBuilder()
                .setPortId(counterPartyPortId).build();

        Channel channel = Channel.newBuilder()
                .setOrdering(Channel.Order.ORDER_UNORDERED)
                .setState(Channel.State.STATE_INIT)
                .addAllConnectionHops(List.of(connectionId))
                .setCounterparty(counterparty)
                .setVersion("").build();

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(channel.toByteArray());

        // Act
        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenInit(
                channel.getOrderingValue(),
                channel.getConnectionHopsList(),
                msg.getPortId(),
                channelId,
                channel.getCounterparty().toByteArray(),
                channel.getVersion());
    }

    void tryOpenChannel() {
        // Arrange

        Channel.Counterparty counterparty = Channel.Counterparty.newBuilder()
                .setPortId(counterPartyPortId)
                .setChannelId(counterPartyChannelId).build();

        Channel channel = Channel.newBuilder()
                .setOrdering(Channel.Order.ORDER_UNORDERED)
                .setState(Channel.State.STATE_TRYOPEN)
                .addAllConnectionHops(List.of(connectionId))
                .setCounterparty(counterparty)
                .setVersion("").build();

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setCounterpartyVersion(baseVersion.getIdentifier());
        msg.setChannel(channel.toByteArray());
        msg.setProofInit(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());

        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenTry(channel.getOrderingValue(), channel.getConnectionHopsList(), portId,
                channelId, channel.getCounterparty().toByteArray(), channel.getVersion(), msg.getCounterpartyVersion());
    }

    void acknowledgeChannel() throws Exception {
        // Arrange
        MsgChannelOpenAck msg = new MsgChannelOpenAck();
        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setCounterpartyVersion(IBCConnection.v1Identifier);
        msg.setCounterpartyChannelId(counterPartyChannelId);
        msg.setProofTry(new byte[0]);
        msg.setProofHeight(Height.getDefaultInstance().toByteArray());

        // Act
        handler.invoke(module.account, "channelOpenAck", msg);

        // Assert
        Channel channel = Channel.parseFrom((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_OPEN, channel.getState());

        verify(module.mock).onChanOpenAck(portId, channelId, msg.getCounterpartyVersion());
    }

    void confirmChannel() throws Exception {
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();
        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(Height.getDefaultInstance().toByteArray());

        // Act
        handler.invoke(module.account, "channelOpenConfirm", msg);

        // Assert
        Channel channel = Channel.parseFrom((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_OPEN, channel.getState());

        verify(module.mock).onChanOpenConfirm(portId, channelId);
    }

    void closeChannel() throws Exception {
        // Arrange
        MsgChannelCloseInit msg = new MsgChannelCloseInit();
        msg.setChannelId(channelId);
        msg.setPortId(portId);

        // Act
        handler.invoke(module.account, "channelCloseInit", msg);

        // Assert
        Channel channel = Channel.parseFrom((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_CLOSED, channel.getState());

        verify(module.mock).onChanCloseInit(portId, channelId);
    }

    void confirmCloseChannel() throws Exception {
        // Arrange
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();
        msg.setChannelId(channelId);
        msg.setPortId(portId);
        msg.setProofHeight(baseHeight.toByteArray());
        msg.setProofInit(new byte[1]);

        // Act
        handler.invoke(module.account, "channelCloseConfirm", msg);

        // Assert
        Channel channel = Channel.parseFrom((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_CLOSED, channel.getState());

        verify(module.mock).onChanCloseConfirm(portId, channelId);
    }

    void sendPacket() {
        // Arrange
        Packet packet = getBasePacket();

        // Act
        handler.invoke(module.account, "sendPacket", packet.toByteArray());

        // Assert
        verify(handlerSpy).SendPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toByteArray(), lastPacketCaptor.getValue());
    }

    void receivePacket() {
        // Arrange
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(packet.toByteArray());
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());

        when(module.mock.onRecvPacket(packet.toByteArray(), relayer.getAddress())).thenReturn(new byte[0]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toByteArray(), lastPacketCaptor.getValue());

        verify(module.mock).onRecvPacket(packet.toByteArray(), relayer.getAddress());

    }

    void receivePacket_withAcK() {
        // Arrange
        Packet packet = getBaseCounterPacket();
        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(packet.toByteArray());
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());

        when(module.mock.onRecvPacket(packet.toByteArray(), relayer.getAddress())).thenReturn(new byte[1]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.toByteArray(), lastPacketCaptor.getValue());

        verify(handlerSpy).WriteAcknowledgement(packet.getDestinationPort(),
                packet.getDestinationChannel(), BigInteger.valueOf(packet.getSequence()), new byte[1]);
    }

    void writeAcknowledgement() throws Exception {
        // Arrange
        byte[] acknowledgement = new byte[1];
        Packet lastPacket = Packet.parseFrom(lastPacketCaptor.getValue());

        // Act
        handler.invoke(module.account, "writeAcknowledgement", lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), BigInteger.valueOf(lastPacket.getSequence()), acknowledgement);

        // Assert
        verify(handlerSpy).WriteAcknowledgement(lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), BigInteger.valueOf(lastPacket.getSequence()), acknowledgement);
    }

    void acknowledgePacket() throws Exception {
        MsgPacketAcknowledgement msg = new MsgPacketAcknowledgement();
        msg.setAcknowledgement(new byte[1]);
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());
        msg.setPacket(Packet.parseFrom(lastPacketCaptor.getValue()).toByteArray());

        // Act
        handler.invoke(relayer, "acknowledgePacket", msg);

        // Assert
        verify(handlerSpy).AcknowledgePacket(msg.getPacketRaw(), msg.getAcknowledgement());
        verify(module.mock).onAcknowledgementPacket(msg.getPacketRaw(), msg.getAcknowledgement(), relayer.getAddress());
    }

    protected Packet getBasePacket() {
        Height timeoutHeight = Height.newBuilder()
                .setRevisionNumber(1)
                .setRevisionHeight(sm.getBlock().getHeight() + 100).build();

        BigInteger nextPacketSeq = (BigInteger) handler.call("getNextSequenceSend", portId, channelId);

        Packet packet = Packet.newBuilder()
                .setSequence(nextPacketSeq.longValue())
                .setSourcePort(portId)
                .setSourceChannel(channelId)
                .setDestinationPort(counterPartyPortId)
                .setDestinationChannel(counterPartyChannelId)
                .setTimeoutHeight(timeoutHeight)
                .setTimeoutTimestamp(sm.getBlock().getTimestamp() * 2).build();

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(Height.getDefaultInstance().toByteArray());
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(byte[].class))).thenReturn(BigInteger.ONE);

        return packet;
    }

    protected Packet getBaseCounterPacket() {
        Height timeoutHeight = Height.newBuilder()
                .setRevisionNumber(1)
                .setRevisionHeight(sm.getBlock().getHeight() + 100).build();

        Packet packet = Packet.newBuilder()
                .setSequence(nextRecvId.longValue())
                .setDestinationChannel(channelId)
                .setDestinationPort(portId)
                .setSourceChannel(counterPartyChannelId)
                .setSourcePort(counterPartyPortId)
                .setData(ByteString.copyFrom(new byte[7]))
                .setTimeoutHeight(timeoutHeight)
                .setTimeoutTimestamp(sm.getBlock().getTimestamp() * 2).build();

        nextRecvId = nextRecvId.add(BigInteger.ONE);

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(Height.getDefaultInstance().toByteArray());
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(byte[].class))).thenReturn(BigInteger.ONE);

        return packet;
    }
}
