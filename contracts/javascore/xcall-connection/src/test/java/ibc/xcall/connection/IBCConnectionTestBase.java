package ibc.xcall.connection;

import java.math.BigInteger;

import org.mockito.Mockito;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.IIBCHandler;
import ibc.icon.interfaces.IIBCHandlerScoreInterface;
import ibc.icon.test.MockContract;
import icon.proto.core.channel.Channel;

public class IBCConnectionTestBase extends TestBase {

    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount(1000);
    protected final Account relayer = sm.createAccount();

    protected Score connection;
    protected static String port = "PORT";
    protected static String nid = "0x2.ICON";
    protected static int ORDER = Channel.Order.ORDER_UNORDERED;
    protected static BigInteger packetFee = BigInteger.valueOf(100);
    protected static BigInteger ackFee = BigInteger.valueOf(50);

    protected static String defaultClientId = "clientId";
    protected static String defaultConnectionId = "connectionId";
    protected static String defaultChannel = "channel-1";
    protected static String defaultCounterpartyPort = "counterpartyPort";
    protected static String defaultCounterpartyChannel = "channel-2";
    protected static String defaultCounterpartyNid = "0x1.ETH";
    protected static BigInteger defaultTimeoutHeight = BigInteger.TEN;

    protected MockContract<ICallservice> xcall;
    protected MockContract<IIBCHandler> ibc;
    protected IBCConnection connectionSpy;

    public void setup() throws Exception {
        xcall = new MockContract<>(ICallserviceScoreInterface.class, ICallservice.class, sm, owner);
        ibc = new MockContract<>(IIBCHandlerScoreInterface.class, IIBCHandler.class, sm, owner);

        connection = sm.deploy(owner, IBCConnection.class, xcall.getAddress(), ibc.getAddress(), port);
        connection.invoke(owner, "setFee", defaultCounterpartyNid, packetFee, ackFee);
        connectionSpy = (IBCConnection) Mockito.spy(connection.getInstance());
        connection.setInstance(connectionSpy);
    }

    public void channelOpenInit(String connectionId, String counterpartyPort, String channelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPort);
        counterparty.setChannelId("");
        connection.invoke(ibc.account, "onChanOpenInit", ORDER,  new String[]{connectionId}, port, channelId, counterparty.encode(), "version-TODO");
    }

    public void channelOpenTry(String connectionId, String counterpartyPort, String channelId, String counterpartyChannelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterpartyPort);
        counterparty.setChannelId(counterpartyChannelId);
        connection.invoke(ibc.account, "onChanOpenTry", ORDER,  new String[]{connectionId}, port, channelId, counterparty.encode(), "version-TODO", "version-TODO");
    }

    public void channelOpenAck(String channelId, String counterpartyChannelId) {
        connection.invoke(ibc.account, "onChanOpenAck", port, channelId, counterpartyChannelId,  "version-TODO");
    }

    public void channelOpenConfirm(String channelId) {
        connection.invoke(ibc.account, "onChanOpenConfirm", port, channelId);
    }
    public void establishDefaultConnection_fromCounterparty() {
        establishConnection_fromCounterparty(defaultClientId, defaultConnectionId, defaultChannel, defaultCounterpartyPort, defaultCounterpartyChannel, defaultCounterpartyNid, defaultTimeoutHeight);
    }

    public void establishConnection_fromCounterparty(String clientId, String connectionId, String channelId, String counterpartyPort, String counterpartyChannel, String nid, BigInteger timeoutHeight) {
        connection.invoke(owner, "configureConnection", connectionId, counterpartyPort, nid, clientId, timeoutHeight);
        channelOpenTry(connectionId, counterpartyPort, channelId, counterpartyChannel);
        channelOpenConfirm(channelId);
    }

    public void establishDefaultConnection() {
        establishConnection(defaultClientId, defaultConnectionId, defaultChannel, defaultCounterpartyPort, defaultCounterpartyChannel, defaultCounterpartyNid, defaultTimeoutHeight);
    }

    public void establishConnection(String clientId, String connectionId, String channelId, String counterpartyPort, String counterpartyChannel, String nid, BigInteger timeoutHeight) {
        connection.invoke(owner, "configureConnection", connectionId, counterpartyPort, nid, clientId, timeoutHeight);
        channelOpenInit(connectionId, counterpartyPort, channelId);
        channelOpenAck(channelId, counterpartyChannel);
    }
}