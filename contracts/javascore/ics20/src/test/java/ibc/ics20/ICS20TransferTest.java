package ibc.ics20;

import com.eclipsesource.json.JsonObject;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import icon.ibc.interfaces.IIBCHandler;
import icon.ibc.interfaces.IIBCHandlerScoreInterface;
import ibc.icon.test.MockContract;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import ics20.ICS20Lib;

import java.lang.String;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;
import score.Address;
import score.Context;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.function.Executable;
import org.mockito.Mockito;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.*;

class ICS20TransferTest extends TestBase {
    private static final ServiceManager sm = getServiceManager();

    private static final Account owner = sm.createAccount();
    private static final Account admin = sm.createAccount();
    private static final Account user = sm.createAccount();
    private static final Account sender = sm.createAccount();
    private static final Account relayer = sm.createAccount();
    private static final Address receiver = sm.createAccount().getAddress();

    private static final Account dest_irc2_token = Account.newScoreAccount(1);

    private static final Account src_irc2_token = Account.newScoreAccount(2);

    // private MockContract<IRC2> token1 = new
    // MockContract<>(IRC2ScoreInterface.class, IRC2.class, sm, owner);

    private Score ics20Transfer;
    private ICS20Transfer ics20TransferSpy;
    private MockContract<IIBCHandler> ibcHandler;
    public static final String TAG = "ICS20";
    protected static String port = "transfer";
    protected static int ORDER = Channel.Order.ORDER_UNORDERED;
    public static final String ICS20_VERSION = "ics20-1";

    private final byte[] irc2Bytes = "test".getBytes();

    @BeforeEach
    void setup() throws Exception {
        ibcHandler = new MockContract<>(IIBCHandlerScoreInterface.class, IIBCHandler.class, sm, owner);
        ics20Transfer = sm.deploy(owner, ICS20Transfer.class, ibcHandler.getAddress(), irc2Bytes);
        ics20TransferSpy = (ICS20Transfer) spy(ics20Transfer.getInstance());
        ics20Transfer.setInstance(ics20TransferSpy);

        ics20Transfer.invoke(owner, "setAdmin", admin.getAddress());

        channelOpenInit("connection-0", "transfer", "channel-0");
        channelOpenAck("channel-0", "channel-1");

        registerCosmosToken(admin, "transfer/channel-0/dest_irc2_token", "Arch", 18, dest_irc2_token);
        ics20Transfer.invoke(admin, "registerIconToken", src_irc2_token.getAddress());

    }

    @Test
    void testGetIBCAddress() {
        assertEquals(ibcHandler.getAddress(), ics20Transfer.call("getIBCAddress"));
    }

    @Test
    void testAdmin() {
        assertEquals(admin.getAddress(), ics20Transfer.call("getAdmin"));

        Executable setAdmin = () -> ics20Transfer.invoke(owner, "setAdmin",
                owner.getAddress());
        expectErrorMessage(setAdmin, "Reverted(0): ICS20 : Caller is not admin");

        ics20Transfer.invoke(admin, "setAdmin", owner.getAddress());
        assertEquals(owner.getAddress(), ics20Transfer.call("getAdmin"));
    }

    @Test
    void testRegisterCosmosToken() {
        Executable cosmosToken = () -> ics20Transfer.invoke(user,
                "registerCosmosToken", "test", "test", 0);
        expectErrorMessage(cosmosToken, "Reverted(0): ICS20 : Caller is not admin");

        registerCosmosToken(admin, "abc", "ab", 18, dest_irc2_token);

        assertEquals(dest_irc2_token.getAddress(),
                ics20Transfer.call("getTokenContractAddress", "abc"));
    }

    @Test
    void testRegisterIconToken() {
        Executable icon = () -> ics20Transfer.invoke(user, "registerIconToken",
                src_irc2_token.getAddress());
        expectErrorMessage(icon, "Reverted(0): ICS20 : Caller is not admin");

        ics20Transfer.invoke(admin, "registerIconToken",
                src_irc2_token.getAddress());
        assertEquals(src_irc2_token.getAddress(),
                ics20Transfer.call("getTokenContractAddress",
                        src_irc2_token.getAddress().toString()));
    }

    @Test
    void testTokenFallbackExceptions() {
        byte[] data = "test".getBytes();
        Executable tokenFallback = () -> ics20Transfer.invoke(user, "tokenFallback",
                user.getAddress(), BigInteger.ZERO, data);
        expectErrorMessage(tokenFallback, "Reverted(0): ICS20 Invalid data: " +
                data.toString());

        byte[] data2 = createByteArray("method", "iconToken", ICX,
                sender.getAddress().toString(), admin.getAddress().toString(), "transfer",
                "channel-0", BigInteger.ONE, BigInteger.ONE, BigInteger.valueOf(10000),
                "memo");
        tokenFallback = () -> ics20Transfer.invoke(user, "tokenFallback",
                user.getAddress(), BigInteger.ZERO, data2);
        expectErrorMessage(tokenFallback, "Reverted(0): ICS20 : Unknown method");

        byte[] data3 = createByteArray("sendFungibleTokens", "iconToken", ICX,
                sender.getAddress().toString(), admin.getAddress().toString(), "transfer",
                "channel-0", BigInteger.ONE, BigInteger.ONE, BigInteger.valueOf(10000),
                "memo");
        tokenFallback = () -> ics20Transfer.invoke(user, "tokenFallback",
                user.getAddress(), BigInteger.ZERO, data3);
        expectErrorMessage(tokenFallback, "Reverted(0): ICS20 : Mismatched amount");

        byte[] data4 = createByteArray("sendFungibleTokens", "iconToken", ICX,
                sender.getAddress().toString(), admin.getAddress().toString(), "transfer",
                "channel-0", BigInteger.ONE, BigInteger.ONE, BigInteger.valueOf(10000),
                "memo");
        tokenFallback = () -> ics20Transfer.invoke(user, "tokenFallback",
                user.getAddress(), ICX, data4);
        expectErrorMessage(tokenFallback, "Reverted(0): ICS20 : Sender address mismatched");

        byte[] data5 = createByteArray("sendFungibleTokens", "iconToken", ICX,
                sender.getAddress().toString(), admin.getAddress().toString(), "transfer",
                "channel-0", BigInteger.ONE, BigInteger.ONE, BigInteger.valueOf(10000),
                "memo");
        tokenFallback = () -> ics20Transfer.invoke(user, "tokenFallback",
                sender.getAddress(), ICX, data5);
        expectErrorMessage(tokenFallback, "Reverted(0): ICS20 : Sender Token Contract not registered");

    }

    @Test
    void testTokenFallbackSourceToken() {

        byte[] data4 = createByteArray("sendFungibleTokens",
                src_irc2_token.getAddress().toString(), ICX, sender.getAddress().toString(),
                admin.getAddress().toString(), "transfer", "channel-0", BigInteger.ONE,
                BigInteger.ONE, BigInteger.valueOf(10000), "memo");

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.call(BigInteger.class, ibcHandler.getAddress(),
                    "getNextSequenceSend", "transfer",
                    "channel-0")).thenReturn(BigInteger.ONE);
            contextMock.when(() -> Context.call(eq(ibcHandler.getAddress()),
                    eq("sendPacket"), any())).thenReturn(true);

            ics20Transfer.invoke(src_irc2_token, "tokenFallback", sender.getAddress(),
                    ICX, data4);
        }

    }

    @Test
    void testTokenFallbackDestToken() {

        byte[] data4 = createByteArray("sendFungibleTokens",
                "transfer/channel-0/dest_irc2_token", ICX, sender.getAddress().toString(),
                admin.getAddress().toString(), "transfer", "channel-0", BigInteger.ONE,
                BigInteger.ONE, BigInteger.valueOf(10000), "memo");

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.call(BigInteger.class, ibcHandler.getAddress(),
                    "getNextSequenceSend", "transfer",
                    "channel-0")).thenReturn(BigInteger.ONE);
            contextMock.when(() -> Context.call(eq(ibcHandler.getAddress()),
                    eq("sendPacket"), any())).thenReturn(true);
            contextMock.when(() -> Context.call(dest_irc2_token.getAddress(), "burn",
                    ICX)).thenReturn(true);

            ics20Transfer.invoke(dest_irc2_token, "tokenFallback", sender.getAddress(),
                    ICX, data4);
        }

    }

    @Test
    void testSendICX() {
        BigInteger amount = BigInteger.TEN.multiply(ICX);
        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.getValue()).thenReturn(amount);
            // for the non configured port or channel id
            Executable sendICX = () -> ics20Transfer.invoke(sender, "sendICX",
                    receiver.toString(), "transfer", "channel-1", new Height(), amount, "memo");
            expectErrorMessage(sendICX, "Reverted(0): ICS20 : Connection not properly Configured");
        }

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.getValue()).thenReturn(amount);
            contextMock.when(() -> Context.call(BigInteger.class, ibcHandler.getAddress(),
                    "getNextSequenceSend", "transfer",
                    "channel-0")).thenReturn(BigInteger.ONE);
            contextMock.when(() -> Context.call(eq(ibcHandler.getAddress()),
                    eq("sendPacket"), any())).thenReturn(true);

            ics20Transfer.invoke(admin, "registerIconToken",
                    src_irc2_token.getAddress());
            ics20Transfer.invoke(sender, "sendICX", receiver.toString(), "transfer",
                    "channel-0", new Height(), amount, "memo");
        }

    }

    @Test
    void testOnRecvPacket_icx() {

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.transfer(receiver, ICX)).then(invocationOnMock -> null);
            _onRecvPacket(ICX, "transfer/channel-1/icx");
        }
    }

    @Test
    void testOnRecvPacket_source() {
        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS)) {
            contextMock
                    .when(() -> Context.call(src_irc2_token.getAddress(), "transfer", receiver, ICX,
                            "memo".getBytes()))
                    .thenReturn(true);

            _onRecvPacket(ICX, "transfer/channel-1/" + src_irc2_token.getAddress().toString());
        }

    }

    @Test
    void testOnRecvPacket_dest() {
        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS)) {
            contextMock
                    .when(() -> Context.call(dest_irc2_token.getAddress(), "mint", receiver, ICX))
                    .thenReturn(true);
            _onRecvPacket(ICX, "dest_irc2_token");
        }

    }

    @Test
    void testOnAcknowledgement_successful() {

        Packet packet = _onRefundPacket(ICX, "src_irc2_token");
        Executable e = () -> ics20Transfer.invoke(admin, "onAcknowledgementPacket",
                packet.encode(), ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON,
                relayer.getAddress());
        expectErrorMessage(e, "Reverted(0): ICS20 : Caller is not IBC Contract");

        ics20Transfer.invoke(ibcHandler.account, "onAcknowledgementPacket",
                packet.encode(), ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON,
                relayer.getAddress());

    }

    @Test
    void testOnAcknowledgement_failure_icx() {

        Packet packet = _onRefundPacket(ICX, "icx");

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.transfer(sender.getAddress(),
                    ICX)).then(invocationOnMock -> null);
            ics20Transfer.invoke(ibcHandler.account, "onAcknowledgementPacket",
                    packet.encode(), ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON, relayer.getAddress());
        }

    }

    @Test
    void testOnAcknowledgement_failure_source_token() {

        Packet packet = _onRefundPacket(ICX, src_irc2_token.getAddress().toString());

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock
                    .when(() -> Context.call(src_irc2_token.getAddress(), "transfer", sender.getAddress(), ICX,
                            "memo".getBytes()))
                    .thenReturn(true);
            contextMock.when(() -> Context.call(any(), any(), any())).thenReturn(true);

            ics20Transfer.invoke(ibcHandler.account, "onAcknowledgementPacket",
                    packet.encode(), ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON, relayer.getAddress());
        }

    }

    @Test
    void testOnTimeOutPacket_dest_token() {

        Packet packet = _onRefundPacket(ICX, "transfer/channel-0/dest_irc2_token");

        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class,
                Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.call(BigInteger.class, ibcHandler.getAddress(),
                    "getNextSequenceSend", "transfer",
                    "channel-0")).thenReturn(BigInteger.ONE);
            contextMock.when(() -> Context.call(dest_irc2_token.getAddress(),
                    "mint", sender.getAddress(), ICX)).thenReturn(true);

            ics20Transfer.invoke(ibcHandler.account, "onTimeoutPacket", packet.encode(),
                    relayer.getAddress());
        }
    }

    void _onRecvPacket(BigInteger amount, String denom) {
        String source_channel = "channel-1";
        String dest_channel = "channel-0";
        Packet packet = createPacket(denom, amount, "sender", receiver.toString(), source_channel, dest_channel);
        ics20Transfer.invoke(ibcHandler.account, "onRecvPacket", packet.encode(), relayer.getAddress());
    }

    Packet _onRefundPacket(BigInteger amount, String denom) {
        String source_channel = "channel-0";
        String dest_channel = "channel-1";
        Packet packet = createPacket(denom, amount, sender.getAddress().toString(), "receiver", source_channel,
                dest_channel);
        return packet;
    }

    private Packet createPacket(String denom, BigInteger amount, String sender, String receiver, String source_channel,
            String dest_channel) {

        Height timeOutHeight = new Height();
        timeOutHeight.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight()));
        timeOutHeight.setRevisionNumber(BigInteger.ONE);

        String data = "{" +
                "\"amount\":\"" + ICX.toString() + "\"," +
                "\"denom\":\"" + denom + "\"," +
                "\"receiver\":\"" + receiver + "\"," +
                "\"sender\":\"" + sender + "\"," +
                "\"memo\":\"" + "memo" + "\"" +
                "}";

        Packet packet = new Packet();
        packet.setSequence(BigInteger.ONE);
        packet.setSourcePort("transfer");
        packet.setSourceChannel(source_channel);
        packet.setDestinationPort("transfer");
        packet.setDestinationChannel(dest_channel);
        packet.setTimeoutHeight(timeOutHeight);
        packet.setTimeoutTimestamp(BigInteger.valueOf(10000));
        packet.setData(data.getBytes());

        return packet;

    }

    private void expectErrorMessage(Executable executable, String expectedErrorMessage) {
        AssertionError e = assertThrows(AssertionError.class, executable);
        assertEquals(expectedErrorMessage, e.getMessage());
    }

    private void registerCosmosToken(Account deployer, String name, String symbol, int decimals, Account token) {
        try (MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS)) {
            contextMock.when(() -> Context.deploy(irc2Bytes, name, symbol, decimals)).thenReturn(token.getAddress());
            ics20Transfer.invoke(deployer, "registerCosmosToken", name, symbol, decimals);
        }
    }

    private byte[] createByteArray(String methodName, String denomination, BigInteger amount, String sender,
            String receiver, String sourcePort, String sourceChannel, BigInteger latestHeight,
            BigInteger revisionNumber, BigInteger timeoutTimestamp, String memo) {

        JsonObject timeoutHeight = new JsonObject()
                .add("latestHeight", latestHeight.longValue())
                .add("revisionNumber", revisionNumber.longValue());

        JsonObject internalParameters = new JsonObject()
                .add("denomination", denomination.toString())
                .add("amount", amount.longValue())
                .add("sender", sender.toString())
                .add("receiver", receiver.toString())
                .add("sourcePort", sourcePort.toString())
                .add("sourceChannel", sourceChannel.toString())
                .add("timeoutHeight", timeoutHeight)
                .add("timeoutTimestamp", timeoutTimestamp.longValue())
                .add("memo", memo.toString());

        JsonObject jsonData = new JsonObject()
                .add("method", methodName.toString())
                .add("params", internalParameters);

        return jsonData.toString().getBytes();
    }

    public void channelOpenInit(String connectionId, String counterpartyPort, String channelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPort);
        counterparty.setChannelId("");
        ics20Transfer.invoke(ibcHandler.account, "onChanOpenInit", ORDER, new String[] { connectionId }, port,
                channelId, counterparty.encode(), ICS20_VERSION);
    }

    public void channelOpenTry(String connectionId, String counterpartyPort, String channelId,
            String counterpartyChannelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPort);
        counterparty.setChannelId(counterpartyChannelId);
        ics20Transfer.invoke(ibcHandler.account, "onChanOpenTry", ORDER, new String[] { connectionId }, port, channelId,
                counterparty.encode(), ICS20_VERSION, ICS20_VERSION);
    }

    public void channelOpenAck(String channelId, String counterpartyChannelId) {
        ics20Transfer.invoke(ibcHandler.account, "onChanOpenAck", port, channelId, counterpartyChannelId,
                ICS20_VERSION);
    }

    public void onChanCloseInit(String channelId) {
        ics20Transfer.invoke(ibcHandler.account, "onChanCloseInit", port, channelId);
    }
}
