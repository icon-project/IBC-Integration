package ibc.core.integration;

import static ibc.icon.integration.ScoreIntegrationTest.createWalletWithBalance;
import static ibc.icon.integration.ScoreIntegrationTest.deploy;
import static ibc.icon.integration.ScoreIntegrationTest.setupPrep;
import static ibc.icon.integration.ScoreIntegrationTest.waitByHeight;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

import ibc.icon.structs.messages.*;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;

import foundation.icon.jsonrpc.model.TransactionResult;
import foundation.icon.score.client.DefaultScoreClient;
import foundation.icon.score.client.Wallet;
import ibc.icon.integration.ScoreIntegrationTest;
import ibc.icon.interfaces.IIBCChannelHandshakeScoreClient;
import ibc.icon.interfaces.IIBCClientScoreClient;
import ibc.icon.interfaces.IIBCConnectionScoreClient;
import ibc.icon.interfaces.IIBCHandlerScoreClient;
import ibc.icon.interfaces.IIBCHostScoreClient;
import ibc.icon.interfaces.IIBCPacketScoreClient;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.Version;
import score.Address;

public class IBCIntegrationTest implements ScoreIntegrationTest {
    static Wallet owner;
    static Wallet relayer;
    static DefaultScoreClient ibcClient;
    static DefaultScoreClient mockLightClient;
    static DefaultScoreClient mockApp;
    static final String clientType = "mockTendermint";
    static final String counterPartyClientId =  "btpClient-1";
    static final String counterPartyConnectionId = "connection-1";
    static final String counterpartyChannelId = "counterpartyChannelId";
    static final String counterpartyPortId = "counterpartyPortId";
    static final String port = "testPort";
    static String clientID;
    static String prevConnectionId;
    static String prevChannelId;
    static Packet prevSentPacket = new Packet();

    @BeforeAll
    static void setup() throws Exception {
        owner = createWalletWithBalance(BigInteger.TEN.pow(22));
        relayer = createWalletWithBalance(BigInteger.TEN.pow(22));
        ibcClient = deploy(owner, "ibc", Map.of());
        mockLightClient = deploy(owner, "mockclient", Map.of());
        mockApp = deploy(owner, "mockapp", Map.of("ibcHandler", ibcClient._address()));
        setupPrep();
    }

    @Test
    @Order(0)
    void registerClient() {
        getClientInterface(owner).registerClient(clientType, mockLightClient._address());
    }

    @Test
    @Order(1)
    void createClient() {
        int networkId = openBTPNetwork(owner, ibcClient._address());
        MsgCreateClient msg = new MsgCreateClient();
        msg.setClientType(clientType);
        msg.setConsensusState(new byte[0]);
        msg.setClientState(new byte[0]);
        msg.setBtpNetworkId(networkId);

        IIBCClientScoreClient client = getClientInterface(owner);
        var consumer = client.CreateClient((logs) -> {clientID = logs.get(0).getIdentifier();}, null);
        client.createClient(consumer, msg);
    }

    @Test
    @Order(2)
    void updateClient() {
        MsgUpdateClient msg = new MsgUpdateClient();

        msg.setClientId(clientType + "-0");
        msg.setClientMessage(new byte[0]);

        getClientInterface(relayer).updateClient(msg);
    }

    @Test
    @Order(3)
    void establishConnection_fromICON() {
        IIBCConnectionScoreClient client = getConnectionInterface(relayer);

        MsgConnectionOpenInit msgInit = new MsgConnectionOpenInit();
        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix("ibc".getBytes());
        icon.proto.core.connection.Counterparty counterparty = new icon.proto.core.connection.Counterparty();
        counterparty.setClientId(counterPartyClientId);
        counterparty.setConnectionId(counterPartyConnectionId);
        counterparty.setPrefix(prefix);
        msgInit.setClientId(clientID);
        msgInit.setCounterparty(counterparty.encode());
        msgInit.setDelayPeriod(BigInteger.ZERO);

        var consumer = client.ConnectionOpenInit((logs) -> {prevConnectionId = logs.get(0).getConnectionId();}, null);
        client.connectionOpenInit(consumer, msgInit);

        MsgConnectionOpenAck msgAck = new MsgConnectionOpenAck();
        Version version = new Version();
        version.setFeatures(List.of("f1"));
        version.setIdentifier("id");
        msgAck.setConnectionId(prevConnectionId);
        msgAck.setClientStateBytes(new byte[0]);
        msgAck.setVersion(version.encode());
        msgAck.setCounterpartyConnectionID(counterPartyConnectionId);
        msgAck.setProofTry(new byte[0]);
        msgAck.setProofClient(new byte[0]);
        msgAck.setProofConsensus(new byte[0]);
        msgAck.setProofHeight(new byte[0]);
        msgAck.setConsensusHeight(new byte[0]);

        client.connectionOpenAck(msgAck);
    }

    @Test
    @Order(4)
    void establishConnection_fromCounterParty() {
        IIBCConnectionScoreClient client = getConnectionInterface(relayer);

        MsgConnectionOpenTry msgTry= new MsgConnectionOpenTry();
        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix("ibc".getBytes());

        Version version = new Version();
        version.setFeatures(List.of("f1"));
        version.setIdentifier("id");

        icon.proto.core.connection.Counterparty counterparty = new icon.proto.core.connection.Counterparty();
        counterparty.setClientId(counterPartyClientId);
        counterparty.setConnectionId(counterPartyConnectionId);

        counterparty.setPrefix(prefix);
        msgTry.setPreviousConnectionId(prevConnectionId);
        msgTry.setCounterparty(counterparty.encode());
        msgTry.setDelayPeriod(BigInteger.ZERO);
        msgTry.setClientId(clientType + "-0");
        msgTry.setClientStateBytes(new byte[0]);
        msgTry.setCounterpartyVersions(new byte[][] { version.encode() });
        msgTry.setProofInit(new byte[0]);
        msgTry.setProofClient(new byte[0]);
        msgTry.setProofConsensus(new byte[0]);
        msgTry.setProofHeight(new byte[0]);
        msgTry.setConsensusHeight(new byte[0]);

        var consumer = client.ConnectionOpenTry((logs) -> {prevConnectionId = logs.get(0).getConnectionId();}, null);
        client.connectionOpenTry(consumer, msgTry);

        MsgConnectionOpenConfirm msgConfirm = new MsgConnectionOpenConfirm();
        msgConfirm.setConnectionId(prevConnectionId);
        msgConfirm.setProofAck(new byte[0]);
        msgConfirm.setProofHeight(new byte[0]);

        client.connectionOpenConfirm(msgConfirm);
    }

    @Test
    @Order(5)
    void setupModule() {
        getHandlerInterface(owner).bindPort(port, mockApp._address());
    }

    @Test
    @Order(6)
    void establishChannel_fromICON() {
        IIBCChannelHandshakeScoreClient client = getChannelInterface(relayer);
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPortId);

        Channel channel = new Channel();
        channel.setState(Channel.State.STATE_INIT);
        channel.setConnectionHops(List.of(prevConnectionId));
        channel.setOrdering(Channel.Order.ORDER_ORDERED);
        channel.setVersion("version");
        channel.setCounterparty(counterparty);

        MsgChannelOpenInit msgInit = new MsgChannelOpenInit();
        msgInit.setPortId(port);
        msgInit.setChannel(channel.encode());

        var consumer = client.ChannelOpenInit((logs) -> {prevChannelId = logs.get(0).getChannelId();}, null);
        client.channelOpenInit(consumer, msgInit);

        MsgChannelOpenAck msgAck = new MsgChannelOpenAck();
        msgAck.setPortId(port);
        msgAck.setChannelId(prevChannelId);
        msgAck.setCounterpartyVersion("version");
        msgAck.setCounterpartyChannelId(counterpartyChannelId);
        msgAck.setProofTry(new byte[0]);
        msgAck.setProofHeight(new byte[0]);

        client.channelOpenAck(msgAck);
    }

    @Test
    @Order(7)
    void closeChannel_fromICON() {
        MsgChannelCloseInit msg = new MsgChannelCloseInit();
        msg.setChannelId(prevChannelId);
        msg.setPortId(port);

        getChannelInterface(relayer).channelCloseInit(msg);

        byte[] channelPb = getHostInterface(relayer).getChannel(port, prevChannelId);
        Channel channel = Channel.decode(channelPb);

        assertEquals(Channel.State.STATE_CLOSED, channel.getState());
    }

    @Test
    @Order(8)
    void establishChannel_fromCounterparty() {
        IIBCChannelHandshakeScoreClient client = getChannelInterface(relayer);
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPortId);

        Channel channel = new Channel();
        channel.setState(Channel.State.STATE_TRYOPEN);
        channel.setConnectionHops(List.of(counterPartyConnectionId));
        channel.setOrdering(Channel.Order.ORDER_ORDERED);
        channel.setVersion("version");
        channel.setCounterparty(counterparty);

        MsgChannelOpenTry msgTry = new MsgChannelOpenTry();
        msgTry.setPortId(port);
        msgTry.setChannel(channel.encode());
        msgTry.setCounterpartyVersion("version");
        msgTry.setProofHeight(new byte[0]);
        msgTry.setProofInit(new byte[0]);


        var consumer = client.ChannelOpenTry((logs) -> {prevChannelId = logs.get(0).getChannelId();}, null);
        client.channelOpenTry(consumer, msgTry);

        MsgChannelOpenConfirm msgConfirm = new MsgChannelOpenConfirm();

        msgConfirm.setPortId(port);
        msgConfirm.setChannelId(prevChannelId);
        msgConfirm.setProofAck(new byte[0]);
        msgConfirm.setProofHeight(new byte[0]);

        client.channelOpenConfirm(msgConfirm);
    }

    @Test
    @Order(9)
    void closeChannel_fromCounterparty() {
        MsgChannelCloseConfirm msg = new MsgChannelCloseConfirm();
        msg.setChannelId(prevChannelId);
        msg.setPortId(port);
        msg.setProofHeight(new byte[0]);
        msg.setProofInit(new byte[0]);

        getChannelInterface(relayer).channelCloseConfirm(msg);

        byte[] channelPb = getHostInterface(relayer).getChannel(port, prevChannelId);
        Channel channel = Channel.decode(channelPb);

        assertEquals(Channel.State.STATE_CLOSED, channel.getState());
    }

    @Test
    @Order(20)
    void setupNewChannel() {
        establishChannel_fromICON();
    }

    @Test
    @Order(21)
    void sendPacket() {
        Consumer<TransactionResult> consumer = getPacketInterface(relayer).SendPacket((logs) -> {prevSentPacket = Packet.decode(logs.get(0).getPacket());}, null);
        byte[] data = "test".getBytes();
        consumer.accept(mockApp._send("sendPacket", Map.of("data", data)));

        assertArrayEquals(data, prevSentPacket.getData());
    }


    @Test
    @Order(21)
    void recvPacket() {
        BigInteger currRecvCount = getHostInterface(relayer).getNextSequenceReceive(port, prevChannelId);
        IIBCPacketScoreClient client = getPacketInterface(relayer);

        Packet pct = new Packet();
        pct.setSequence(currRecvCount);
        pct.setData("test".getBytes());
        pct.setDestinationPort(port);
        pct.setDestinationChannel(prevChannelId);
        pct.setSourcePort(counterpartyPortId);
        pct.setSourceChannel(counterpartyChannelId);

        Height hgt = new Height();
        pct.setTimeoutHeight(hgt);

        pct.setTimeoutTimestamp(BigInteger.ZERO);

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(pct.encode());
        msg.setProof(new byte[0]);
        msg.setProofHeight(new byte[0]);

        var consumer = client.WriteAcknowledgement((logs) -> {assertArrayEquals("ack".getBytes(), logs.get(0).getAcknowledgement());}, null);
        client.recvPacket(msg);
    }

    @Test
    @Order(21)
    void recvPacket_manualAck() {
        BigInteger currRecvCount = getHostInterface(relayer).getNextSequenceReceive(port, prevChannelId);
        IIBCPacketScoreClient client = getPacketInterface(relayer);

        Packet pct = new Packet();
        pct.setSequence(currRecvCount);
        pct.setData("skip ack".getBytes());
        pct.setDestinationPort(port);
        pct.setDestinationChannel(prevChannelId);
        pct.setSourcePort(counterpartyPortId);
        pct.setSourceChannel(counterpartyChannelId);
        pct.setTimeoutHeight(new Height());
        pct.setTimeoutTimestamp(BigInteger.ZERO);

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(pct.encode());
        msg.setProof(new byte[0]);
        msg.setProofHeight(new byte[0]);

        var emptyConsumer = client.WriteAcknowledgement((logs) -> {assertEquals(0, logs.size());}, null);
        client.recvPacket(emptyConsumer, msg);

        byte[] ack = "ack".getBytes();
        Consumer<TransactionResult> consumer = client.WriteAcknowledgement((logs) -> {assertArrayEquals(ack, logs.get(0).getAcknowledgement());}, null);
        consumer.accept(mockApp._send("ackPacket", Map.of("packet", pct.encode(), "ack", ack)));
    }

    @Test
    @Order(21)
    void acknowledgePacket() {
        sendPacket();
        IIBCPacketScoreClient client = getPacketInterface(relayer);

        MsgPacketAcknowledgement msg = new MsgPacketAcknowledgement();
        msg.setPacket(prevSentPacket.encode());
        msg.setAcknowledgement("counterparty ack".getBytes());
        msg.setProof(new byte[0]);
        msg.setProofHeight(new byte[0]);

        client.acknowledgePacket(msg);
    }

    @Test
    @Order(21)
    void requestTimeout() {
        BigInteger currRecvCount = getHostInterface(relayer).getNextSequenceReceive(port, prevChannelId);
        IIBCPacketScoreClient client = getPacketInterface(relayer);

        BigInteger currentHeight = mockApp._lastBlockHeight();
        Height timeoutHeight = new Height();
        timeoutHeight.setRevisionHeight(currentHeight);
        MsgRequestTimeoutPacket requestPacket=new MsgRequestTimeoutPacket();

        Packet pct = new Packet();
        pct.setSequence(currRecvCount);
        pct.setData("timeout".getBytes());
        pct.setDestinationPort(port);
        pct.setDestinationChannel(prevChannelId);
        pct.setSourcePort(counterpartyPortId);
        pct.setSourceChannel(counterpartyChannelId);
        pct.setTimeoutHeight(timeoutHeight);
        pct.setTimeoutTimestamp(BigInteger.ZERO);
        requestPacket.setPacket(pct.encode());
        requestPacket.setProof(new byte[]{});
        requestPacket.setProofHeight(new byte[]{});

        client.requestTimeout(requestPacket);
    }

    // Will close the Channel
    @Test
    @Order(22)
    void timeoutPacket() {
        BigInteger currRecvCount = getHostInterface(relayer).getNextSequenceReceive(port, prevChannelId);
        IIBCPacketScoreClient client = getPacketInterface(relayer);

        BigInteger currentHeight = mockApp._lastBlockHeight();
        BigInteger timeoutHeight = currentHeight.add(BigInteger.TEN);

        Consumer<TransactionResult> consumer = getPacketInterface(relayer).SendPacket((logs) -> {prevSentPacket = Packet.decode(logs.get(0).getPacket());}, null);
        consumer.accept(mockApp._send("sendPacket", Map.of("data", "test".getBytes(), "timeoutHeight", timeoutHeight)));
        waitByHeight(timeoutHeight);

        MsgPacketTimeout msg = new MsgPacketTimeout();
        msg.setPacket(prevSentPacket.encode());
        msg.setNextSequenceRecv(currRecvCount);
        msg.setProof(new byte[0]);
        Height proofHeight = new Height();
        proofHeight.setRevisionHeight(timeoutHeight);
        msg.setProofHeight(proofHeight.encode());
        client.timeoutPacket(msg);

        byte[] channelPb = getHostInterface(relayer).getChannel(port, prevChannelId);
        Channel channel = Channel.decode(channelPb);

        assertEquals(Channel.State.STATE_CLOSED, channel.getState());
    }

    IIBCClientScoreClient getClientInterface(Wallet wallet) {
        return new IIBCClientScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCConnectionScoreClient getConnectionInterface(Wallet wallet) {
        return new IIBCConnectionScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCChannelHandshakeScoreClient getChannelInterface(Wallet wallet) {
        return new IIBCChannelHandshakeScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCPacketScoreClient getPacketInterface(Wallet wallet) {
        return new IIBCPacketScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCHandlerScoreClient getHandlerInterface(Wallet wallet) {
        return new IIBCHandlerScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCHostScoreClient getHostInterface(Wallet wallet) {
        return new IIBCHostScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    int openBTPNetwork(Wallet wallet, Address score) {
        DefaultScoreClient govScore =  new DefaultScoreClient(chain.getEndpointURL(), chain.networkId, wallet,  new foundation.icon.jsonrpc.Address("cx0000000000000000000000000000000000000001"));

        Map<String, Object> params = Map.of(
            "networkTypeName", "eth",
            "name", "testNetwork",
            "owner", score
        );
        TransactionResult res = govScore._send("openBTPNetwork", params);
        TransactionResult.EventLog log = res.getEventLogs().get(0);
        return Integer.decode(log.getIndexed().get(2));
    }
}
