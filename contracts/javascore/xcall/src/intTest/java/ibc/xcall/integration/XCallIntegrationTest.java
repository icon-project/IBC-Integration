package ibc.xcall.integration;

import static ibc.icon.integration.ScoreIntegrationTest.createWalletWithBalance;
import static ibc.icon.integration.ScoreIntegrationTest.deploy;
import static ibc.icon.integration.ScoreIntegrationTest.setupPrep;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import foundation.icon.btp.xcall.data.CSMessage;
import foundation.icon.btp.xcall.data.CSMessageRequest;
import foundation.icon.btp.xcall.interfaces.CallServiceScoreClient;
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
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgPacketRecv;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.MerklePrefix;
import icon.proto.core.connection.Version;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import score.Address;

public class XCallIntegrationTest implements ScoreIntegrationTest {

    static Wallet owner;
    static Wallet relayer;
    static DefaultScoreClient ibcClient;
    static DefaultScoreClient xCall;
    static DefaultScoreClient mockLightClient;
    static DefaultScoreClient mockApp;
    static final String clientType = "mockTendermint";
    static final String counterPartyClientId = "btpClient-1";
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
        xCall = deploy(owner, "xcall",
                Map.of("_ibc", ibcClient._address(), "_timeoutHeight", BigInteger.valueOf(1000)));

        mockLightClient = deploy(owner, "mockclient", Map.of());
        mockApp = deploy(owner, "mock-dapp", Map.of("_callService", xCall._address()));
        setupPrep();


    }


    @Test
    @Order(0)
    void createClient() {
        getClientInterface(owner).registerClient(clientType, mockLightClient._address());

        //create client
        int networkId = openBTPNetwork(owner, ibcClient._address());
        MsgCreateClient msg = new MsgCreateClient();
        msg.setClientType(clientType);
        msg.setConsensusState(new byte[0]);
        msg.setClientState(new byte[0]);
        msg.setBtpNetworkId(networkId);

        IIBCClientScoreClient client = getClientInterface(owner);
        var consumer = client.CreateClient((logs) -> {clientID = logs.get(0).getIdentifier();}, null);
        client.createClient(consumer, msg);

//connection
        IIBCConnectionScoreClient connection = getConnectionInterface(relayer);

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

        var connectionConsumer = connection.ConnectionOpenInit(
                (logs) -> {prevConnectionId = logs.get(0).getConnectionId();}, null);
        connection.connectionOpenInit(connectionConsumer, msgInit);

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

        connection.connectionOpenAck(msgAck);

    }


    @Test
    @Order(1)
    void setupModule() {
        getHandlerInterface(owner).bindPort(port, xCall._address());
    }

    @Test
    @Order(2)
    void establishChannel() {
        IIBCChannelHandshakeScoreClient client = getChannelInterface(relayer);
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPortId);
        counterparty.setChannelId(counterpartyChannelId);

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
    @Order(3)
    void sendPacket() {

        Consumer<TransactionResult> consumer = getPacketInterface(relayer).SendPacket(
                (logs) -> {prevSentPacket = Packet.decode(logs.get(0).getPacket());}, null);
        byte[] data = "data".getBytes();
        byte[] rollback = "rollback".getBytes();
        consumer.accept(mockApp._send("sendMessage",
                Map.of("_to", xCall._address().toString(), "_data", data, "_rollback", rollback)));

        CSMessageRequest request = CSMessageRequest.fromBytes(CSMessage.fromBytes(prevSentPacket.getData()).getData());
        assertArrayEquals(data, request.getData());
        assertEquals(xCall._address().toString(), request.getTo());
        assertEquals(mockApp._address().toString(), request.getFrom());

    }


    @Test
    @Order(4)
    void recvPacket() {
        BigInteger currRecvCount = getHostInterface(relayer).getNextSequenceReceive(port, prevChannelId);

        CallServiceScoreClient client = getCallService(relayer);

        CSMessage req = new CSMessage(CSMessage.REQUEST,
                new CSMessageRequest("xyz", mockApp._address().toString(), BigInteger.TEN, false,
                        "_data".getBytes()).toBytes());

        Packet pct = new Packet();
        pct.setSequence(currRecvCount);
        pct.setData(req.toBytes());
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

        var consumer = client.CallMessage((logs) -> {
            assertEquals(counterpartyPortId + "/" + counterpartyChannelId, logs.get(0).get_from());
            assertEquals(BigInteger.TEN, logs.get(0).get_sn());
            assertEquals(mockApp._address().toString(), logs.get(0).get_to());
        }, null);

        consumer.accept(ibcClient._send("recvPacket", Map.of("msg", msg)));
    }

    IIBCClientScoreClient getClientInterface(Wallet wallet) {
        return new IIBCClientScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCConnectionScoreClient getConnectionInterface(Wallet wallet) {
        return new IIBCConnectionScoreClient(chain.getEndpointURL(), chain.networkId, wallet, ibcClient._address());
    }

    IIBCChannelHandshakeScoreClient getChannelInterface(Wallet wallet) {
        return new IIBCChannelHandshakeScoreClient(chain.getEndpointURL(), chain.networkId, wallet,
                ibcClient._address());
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

    CallServiceScoreClient getCallService(Wallet wallet) {
        return new CallServiceScoreClient(chain.getEndpointURL(), chain.networkId, wallet, xCall._address());
    }

    int openBTPNetwork(Wallet wallet, Address score) {
        DefaultScoreClient govScore = new DefaultScoreClient(chain.getEndpointURL(), chain.networkId, wallet,
                new foundation.icon.jsonrpc.Address("cx0000000000000000000000000000000000000001"));

        Map<String, Object> params = Map.of("networkTypeName", "eth", "name", "testNetwork", "owner", score);
        TransactionResult res = govScore._send("openBTPNetwork", params);
        TransactionResult.EventLog log = res.getEventLogs().get(0);
        return Integer.decode(log.getIndexed().get(2));
    }
}
