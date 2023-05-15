package ibc.xcall.connection;

import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.verify;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import foundation.icon.btp.xcall.CSMessage;
import foundation.icon.btp.xcall.CSMessageRequest;
import foundation.icon.btp.xcall.CSMessageResponse;
import ibc.icon.interfaces.IIBCHandler;
import ibc.icon.interfaces.IIBCHandlerScoreInterface;
import ibc.icon.test.MockContract;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import java.math.BigInteger;
import java.util.List;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import org.mockito.stubbing.Answer;
import score.Context;

public class IBCConnectionTestBase extends TestBase {

    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account relayer = sm.createAccount();

    protected Score connection;
    protected static String nid = "0x2.ICON";
    protected static int ORDER = Channel.Order.ORDER_UNORDERED;
    protected static BigInteger timeoutHeight = BigInteger.valueOf(100);

    protected MockContract<ICallservice> xcall;
    protected MockContract<IIBCHandler> ibc;

    public void setup() throws Exception {
        xcall = new MockContract<>(ICallserviceScoreInterface.class, ICallservice.class, sm, owner);
        ibc = new MockContract<>(IIBCHandlerScoreInterface.class, IIBCHandler.class, sm, owner);

        connection = sm.deploy(owner, IBCConnection.class, xcall.getAddress(), ibc.getAddress(), nid, timeoutHeight);
    }

    public void channelOpenInit(String channelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(IBCConnection.PORT);
        counterparty.setChannelId("");
        connection.invoke(ibc.account, "onChanOpenInit", ORDER,  new String[]{"TODO"}, IBCConnection.PORT, channelId, counterparty.encode(), "version-TODO");
    }

    public void channelOpenTry(String channelId, String counterpartyChannelId) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(IBCConnection.PORT);
        counterparty.setChannelId(counterpartyChannelId);
        connection.invoke(ibc.account, "onChanOpenTry", ORDER,  new String[]{"TODO"}, IBCConnection.PORT, channelId, counterparty.encode(), "version-TODO", "version-TODO");
    }

    public void channelOpenAck(String channelId, String counterpartyChannelId) {
        connection.invoke(ibc.account, "onChanOpenAck", IBCConnection.PORT, channelId, counterpartyChannelId,  "version-TODO");
    }

    public void channelOpenConfirm(String channelId) {
        connection.invoke(ibc.account, "onChanOpenConfirm", IBCConnection.PORT, channelId);
    }

    public void establishConnection_fromCounterparty(String channelId, String counterpartyChannel, String nid) {
        channelOpenTry(channelId, counterpartyChannel);
        channelOpenConfirm(channelId);
        connection.invoke(owner, "configureChannel", channelId, nid);
    }

    public void establishConnection(String channelId, String counterpartyChannel, String nid) {
        channelOpenInit(channelId);
        channelOpenAck(channelId, counterpartyChannel);
        connection.invoke(owner, "configureChannel", channelId, nid);
    }
}