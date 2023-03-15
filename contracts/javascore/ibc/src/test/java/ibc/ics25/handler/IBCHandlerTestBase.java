package ibc.ics25.handler;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.*;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.ConnectionEnd;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.Version;
import ibc.icon.test.MockContract;
import ibc.ics03.connection.IBCConnection;
import org.mockito.ArgumentCaptor;

import java.math.BigInteger;

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
    protected Version baseVersion = new Version();
    protected String portId = "portId";

    protected BigInteger nextRecvId = BigInteger.ONE;
    protected Height baseHeight = new Height();

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

        prefix = new MerklePrefix();
        prefix.setKeyPrefix("ibc".getBytes());

        baseVersion.setIdentifier(IBCConnection.v1Identifier);
        baseVersion.setFeatures(IBCConnection.supportedV1Features);

        baseHeight.setRevisionHeight(BigInteger.ONE);
        baseHeight.setRevisionNumber(BigInteger.ONE);
    }

    void createClient() {
        // Arrange
        handler.invoke(owner, "registerClient", clientType, lightClient.getAddress());
        MsgCreateClient msg = new MsgCreateClient();
        msg.setClientState(new byte[0]);
        msg.setConsensusState(new byte[0]);
        msg.setClientType(clientType);

        ConsensusStateUpdate update = new ConsensusStateUpdate(new byte[0],
                new Height().encode());
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

        Height consensusHeight = new Height();
        consensusHeight.setRevisionHeight(BigInteger.ONE);
        consensusHeight.setRevisionNumber(BigInteger.TWO);

        ConsensusStateUpdate update = new ConsensusStateUpdate(consensusStateCommitment, consensusHeight.encode());

        UpdateClientResponse response = new UpdateClientResponse(clientStateCommitment, update, true);

        when(lightClient.mock.updateClient(msg.getClientId(), msg.getClientMessage())).thenReturn(response);

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
        msg.setClientId(clientId);
        msg.setCounterparty(counterparty.encode());
        msg.setDelayPeriod(delayPeriod);

        handler.invoke(module.account, "connectionOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void tryOpenConnection() {
        // Arrange
        MsgConnectionOpenTry msg = new MsgConnectionOpenTry();
        Counterparty counterparty = new Counterparty();
        counterparty.setClientId(counterPartyClientId);
        counterparty.setConnectionId(counterPartyConnectionId);
        counterparty.setPrefix(prefix);
        msg.setCounterparty(counterparty.encode());
        msg.setDelayPeriod(delayPeriod);
        msg.setClientId(clientId);
        msg.setClientStateBytes(new byte[0]);
        msg.setCounterpartyVersions(new byte[][]{baseVersion.encode()});
        msg.setProofInit(new byte[0]);
        msg.setProofClient(new byte[0]);
        msg.setProofConsensus(new byte[0]);
        msg.setProofHeight(baseHeight.encode());
        msg.setConsensusHeight(baseHeight.encode());

        // Act
        handler.invoke(module.account, "connectionOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedConnectionIdentifier(connectionIdCaptor.capture());
        connectionId = connectionIdCaptor.getValue();
    }

    void acknowledgeConnection() {
        // Arrange
        MsgConnectionOpenAck msg = new MsgConnectionOpenAck();
        msg.setConnectionId(connectionId);
        msg.setClientStateBytes(new byte[0]);
        msg.setVersion(baseVersion.encode());
        msg.setCounterpartyConnectionID(counterPartyConnectionId);
        msg.setProofTry(new byte[0]);
        msg.setProofClient(new byte[0]);
        msg.setProofConsensus(new byte[0]);
        msg.setProofHeight(baseHeight.encode());
        msg.setConsensusHeight(baseHeight.encode());

        // Act
        handler.invoke(module.account, "connectionOpenAck", msg);

        // Assert
        ConnectionEnd connection = ConnectionEnd.decode((byte[]) handler.call("getConnection", connectionId));
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.getState());

    }

    void confirmConnection() {
        // Arrange
        MsgConnectionOpenConfirm msg = new MsgConnectionOpenConfirm();
        msg.setConnectionId(connectionId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(baseHeight.encode());

        // Act
        handler.invoke(module.account, "connectionOpenConfirm", msg);

        // Assert
        ConnectionEnd connection = ConnectionEnd.decode((byte[]) handler.call("getConnection", connectionId));
        assertEquals(ConnectionEnd.State.STATE_OPEN, connection.getState());
    }

    void openChannel() {
        // Arrange
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterPartyPortId);

        Channel channel = new Channel();
        channel.setOrdering(Channel.Order.ORDER_UNORDERED);
        channel.setState(Channel.State.STATE_INIT);
        channel.setConnectionHops(List.of(connectionId));
        channel.setCounterparty(counterparty);
        channel.setVersion("");

        MsgChannelOpenInit msg = new MsgChannelOpenInit();
        msg.setPortId(portId);
        msg.setChannel(channel.encode());

        // Act
        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenInit", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenInit(
                msg.getChannel().getOrdering(),
                msg.getChannel().getConnectionHops(),
                msg.getPortId(),
                channelId,
                msg.getChannel().getCounterparty().encode(),
                msg.getChannel().getVersion());
    }

    void tryOpenChannel() {
        // Arrange
        Channel channel = new Channel();

        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterPartyPortId);
        counterparty.setChannelId(counterPartyChannelId);

        channel.setOrdering(Channel.Order.ORDER_UNORDERED);
        channel.setState(Channel.State.STATE_TRYOPEN);
        channel.setConnectionHops(List.of(connectionId));
        channel.setCounterparty(counterparty);
        channel.getCounterparty().setPortId(counterPartyPortId);
        channel.getCounterparty().setChannelId(counterPartyChannelId);
        channel.setVersion("");

        MsgChannelOpenTry msg = new MsgChannelOpenTry();
        msg.setPortId(portId);
        msg.setCounterpartyVersion(baseVersion.getIdentifier());
        msg.setChannel(channel.encode());
        msg.setProofInit(new byte[0]);
        msg.setProofHeight(baseHeight.encode());

        handler.invoke(owner, "bindPort", portId, module.getAddress());
        handler.invoke(module.account, "channelOpenTry", msg);

        // Assert
        verify(handlerSpy).GeneratedChannelIdentifier(channelIdCaptor.capture());
        channelId = channelIdCaptor.getValue();

        verify(module.mock).onChanOpenTry(channel.getOrdering(), channel.getConnectionHops(), portId, channelId,
                channel.getCounterparty().encode(), channel.getVersion(), msg.getCounterpartyVersion());
    }

    void acknowledgeChannel() {
        // Arrange
        MsgChannelOpenAck msg = new MsgChannelOpenAck();
        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setCounterpartyVersion(IBCConnection.v1Identifier);
        msg.setCounterpartyChannelId(counterPartyChannelId);
        msg.setProofTry(new byte[0]);
        msg.setProofHeight(new Height().encode());

        // Act
        handler.invoke(module.account, "channelOpenAck", msg);

        // Assert
        Channel channel = Channel.decode((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_OPEN, channel.getState());

        verify(module.mock).onChanOpenAck(portId, channelId, msg.getCounterpartyVersion());
    }

    void confirmChannel() {
        MsgChannelOpenConfirm msg = new MsgChannelOpenConfirm();
        msg.setPortId(portId);
        msg.setChannelId(channelId);
        msg.setProofAck(new byte[0]);
        msg.setProofHeight(new Height().encode());

        // Act
        handler.invoke(module.account, "channelOpenConfirm", msg);

        // Assert
        Channel channel = Channel.decode((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_OPEN, channel.getState());

        verify(module.mock).onChanOpenConfirm(portId, channelId);
    }

    void closeChannel() {
        // Arrange
        MsgChannelCloseInit msg = new MsgChannelCloseInit();
        msg.setChannelId(channelId);
        msg.setPortId(portId);

        // Act
        handler.invoke(module.account, "channelCloseInit", msg);

        // Assert
        Channel channel = Channel.decode((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_CLOSED, channel.getState());

        verify(module.mock).onChanCloseInit(portId, channelId);
    }

    void confirmCloseChannel() {
        // Arrange
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();
        msg.setChannelId(channelId);
        msg.setPortId(portId);
        msg.setProofHeight(baseHeight.encode());
        msg.setProofInit(new byte[1]);

        // Act
        handler.invoke(module.account, "channelCloseConfirm", msg);

        // Assert
        Channel channel = Channel.decode((byte[]) handler.call("getChannel", portId, channelId));
        assertEquals(Channel.State.STATE_CLOSED, channel.getState());

        verify(module.mock).onChanCloseConfirm(portId, channelId);
    }

    void sendPacket() {
        // Arrange
        Packet packet = getBasePacket();

        // Act
        handler.invoke(module.account, "sendPacket", packet.encode());

        // Assert
        verify(handlerSpy).SendPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.encode(), lastPacketCaptor.getValue());
    }

    void receivePacket() {
        // Arrange
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(packet.encode());
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.encode());

        when(module.mock.onRecvPacket(packet.encode(), relayer.getAddress())).thenReturn(new byte[0]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.encode(), lastPacketCaptor.getValue());

        verify(module.mock).onRecvPacket(packet.encode(), relayer.getAddress());

    }

    void receivePacket_withAcK() {
        // Arrange
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(packet.encode());
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.encode());

        when(module.mock.onRecvPacket(packet.encode(), relayer.getAddress())).thenReturn(new byte[1]);

        // Act
        handler.invoke(relayer, "recvPacket", msg);

        // Assert
        verify(handlerSpy).RecvPacket(lastPacketCaptor.capture());
        assertArrayEquals(packet.encode(), lastPacketCaptor.getValue());

        verify(handlerSpy).WriteAcknowledgement(packet.getDestinationPort(),
                packet.getDestinationChannel(), packet.getSequence(), new byte[1]);
    }

    void writeAcknowledgement() {
        // Arrange
        byte[] acknowledgement = new byte[1];
        Packet lastPacket = Packet.decode(lastPacketCaptor.getValue());

        // Act
        handler.invoke(module.account, "writeAcknowledgement", lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), lastPacket.getSequence(), acknowledgement);

        // Assert
        verify(handlerSpy).WriteAcknowledgement(lastPacket.getDestinationPort(),
                lastPacket.getDestinationChannel(), lastPacket.getSequence(), acknowledgement);
    }

    void acknowledgePacket() {
        MsgPacketAcknowledgement msg = new MsgPacketAcknowledgement();
        msg.setAcknowledgement(new byte[1]);
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.encode());
        msg.setPacket(Packet.decode(lastPacketCaptor.getValue()).encode());

        // Act
        handler.invoke(relayer, "acknowledgePacket", msg);

        // Assert
        verify(handlerSpy).AcknowledgePacket(msg.getPacketRaw(), msg.getAcknowledgement());
        verify(module.mock).onAcknowledgementPacket(msg.getPacketRaw(), msg.getAcknowledgement(), relayer.getAddress());
    }

    protected Packet getBasePacket() {
        Height timeoutHeight = new Height();
        timeoutHeight.setRevisionNumber(BigInteger.ONE);
        timeoutHeight.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight() + 100));

        Packet packet = new Packet();
        BigInteger nextPacketSeq = (BigInteger) handler.call("getNextSequenceSend", portId, channelId);
        packet.setSequence(nextPacketSeq);
        packet.setSourcePort(portId);
        packet.setSourceChannel(channelId);
        packet.setDestinationPort(counterPartyPortId);
        packet.setDestinationChannel(counterPartyChannelId);
        packet.setData(new byte[7]);
        packet.setTimeoutHeight(timeoutHeight);
        packet.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp() * 2));

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(new Height().encode());
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(byte[].class))).thenReturn(BigInteger.ONE);

        return packet;
    }

    protected Packet getBaseCounterPacket() {
        Height timeoutHeight = new Height();
        timeoutHeight.setRevisionNumber(BigInteger.ONE);
        timeoutHeight.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight() + 100));

        Packet packet = new Packet();
        packet.setSequence(nextRecvId);
        nextRecvId = nextRecvId.add(BigInteger.ONE);
        packet.setDestinationChannel(channelId);
        packet.setDestinationPort(portId);
        packet.setSourceChannel(counterPartyChannelId);
        packet.setSourcePort(counterPartyPortId);
        packet.setData(new byte[7]);
        packet.setTimeoutHeight(timeoutHeight);
        packet.setTimeoutTimestamp(BigInteger.valueOf(sm.getBlock().getTimestamp() * 2));

        when(lightClient.mock.getLatestHeight(clientId)).thenReturn(new Height().encode());
        when(lightClient.mock.getTimestampAtHeight(any(String.class), any(byte[].class))).thenReturn(BigInteger.ONE);

        return packet;
    }
}
