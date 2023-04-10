package ibc.xcall.integration;

import static ibc.icon.integration.ScoreIntegrationTest.createWalletWithBalance;
import static ibc.icon.integration.ScoreIntegrationTest.deploy;
import static ibc.icon.integration.ScoreIntegrationTest.setupPrep;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;

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
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.mock.RollbackData;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import score.Address;
import score.annotation.Optional;

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
        xCall = deploy(owner, "xcall", Map.of("_ibc", ibcClient._address()));

        mockLightClient = deploy(owner, "mockclient", Map.of());
        mockApp = deploy(owner, "mock-dapp", Map.of("_callService", xCall._address()));
        setupPrep();
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
    @Order(21)
    void sendPacket() {

        Consumer<TransactionResult> consumer = getPacketInterface(relayer).SendPacket(
                (logs) -> {prevSentPacket = Packet.decode(logs.get(0).getPacket());}, null);
        byte[] data = "data".getBytes();
        byte[] rollback = new RollbackData(BigInteger.ONE, "rollback".getBytes()).toBytes();
        consumer.accept(mockApp._send("sendMessage", Map.of("_to", data, "_data", data, "_rollback", rollback)));

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

        var consumer = client.WriteAcknowledgement(
                (logs) -> {assertArrayEquals("ack".getBytes(), logs.get(0).getAcknowledgement());}, null);
        client.recvPacket(msg);
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

    int openBTPNetwork(Wallet wallet, Address score) {
        DefaultScoreClient govScore = new DefaultScoreClient(chain.getEndpointURL(), chain.networkId, wallet,
                new foundation.icon.jsonrpc.Address("cx0000000000000000000000000000000000000001"));

        Map<String, Object> params = Map.of("networkTypeName", "eth", "name", "testNetwork", "owner", score);
        TransactionResult res = govScore._send("openBTPNetwork", params);
        TransactionResult.EventLog log = res.getEventLogs().get(0);
        return Integer.decode(log.getIndexed().get(2));
    }
}
