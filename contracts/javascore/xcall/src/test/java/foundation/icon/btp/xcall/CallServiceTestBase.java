package foundation.icon.btp.xcall;

import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.verify;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import foundation.icon.btp.xcall.data.CSMessage;
import foundation.icon.btp.xcall.data.CSMessageRequest;
import foundation.icon.btp.xcall.data.CSMessageResponse;
import ibc.icon.interfaces.ICallServiceReceiver;
import ibc.icon.interfaces.ICallServiceReceiverScoreInterface;
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

public class CallServiceTestBase extends TestBase {

    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account relayer = sm.createAccount();

    protected Score client;

    protected CallServiceImpl clientSpy;

    protected final String counterPartyPortId = "counterparty-port-id";
    protected final String counterPartyChannelId = "counterparty-channel-id";
    protected final String portId = "port-id";
    protected final String channelId = "channel-id";
    protected final String connectionId = "connection-id";

    protected MockContract<ICallServiceReceiver> dApp;
    protected Account ibcHandler;


    protected final BigInteger TIMEOUT_HEIGHT = BigInteger.valueOf(997L);


    protected final MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS);

    public void setup() throws Exception {
        dApp = new MockContract<>(ICallServiceReceiverScoreInterface.class, ICallServiceReceiver.class, sm, owner);
        ibcHandler =  Account.newScoreAccount(1001);

        client = sm.deploy(owner, CallServiceImpl.class, ibcHandler.getAddress(), BigInteger.valueOf(1000));
        clientSpy = (CallServiceImpl) spy(client.getInstance());
        client.setInstance(clientSpy);

    }


    protected void teardown() {
        contextMock.close();
    }

    protected void onChanOpenInit(Account account) {
        Channel.Counterparty counterparty = new Channel.Counterparty();
        counterparty.setPortId(counterPartyPortId);
        counterparty.setChannelId(counterPartyChannelId);

        client.invoke(account, "onChanOpenInit", Channel.Order.ORDER_ORDERED, List.of(connectionId), portId, channelId,
                counterparty.encode(), "");

    }


    protected void onChanOpenAck(Account account) {
        client.invoke(account, "onChanOpenAck", portId, channelId, counterPartyChannelId, "");
    }

    protected void sendCallMessage(String _to, byte[] _data, byte[] rollback) {
        onChanOpenInit(ibcHandler);
        onChanOpenAck(ibcHandler);

        contextMock.when(Context::getValue).thenReturn(BigInteger.ONE);
        Packet packet = getRequestPacket(_to, _data, rollback);

        contextMock.when(() -> {
            Context.call(this.ibcHandler.getAddress(), "sendPacket", new Object[]{packet.encode()});
        }).thenAnswer((Answer<Void>) invocation -> null);

        client.invoke(dApp.account, "sendCallMessage", _to, _data, rollback);

        verify(clientSpy).CallMessageSent(dApp.getAddress(), _to, BigInteger.ONE, BigInteger.ONE);
    }


    protected void onRecvPacket(String _from, byte[] _data, byte[] rollback) {
        onChanOpenInit(ibcHandler);
        onChanOpenAck(ibcHandler);

        Packet packet = getRecvRequestPacket(_from, _data, rollback);
        byte[] data = packet.encode();
        client.invoke(ibcHandler, "onRecvPacket", data, relayer.getAddress());
        verify(clientSpy).CallMessage(counterPartyPortId + "/" + counterPartyChannelId, dApp.getAddress().toString(),
                BigInteger.ONE, BigInteger.ONE);
    }

    protected void onRecvResponsePacket(int code, String msg) {
        Packet packet = getResponsePacket(code, msg);
        byte[] data = packet.encode();
        client.invoke(ibcHandler, "onRecvPacket", data, relayer.getAddress());
        verify(clientSpy).ResponseMessage(BigInteger.ONE, code, msg);
    }

    protected Packet getRequestPacket(String _to, byte[] data, byte[] rollback) {
        BigInteger nextRecvId = BigInteger.ONE;
        Height height = new Height();
        height.setRevisionNumber(BigInteger.ZERO);
        height.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight()).add(BigInteger.ONE).add(TIMEOUT_HEIGHT));

        CSMessageRequest msg = new CSMessageRequest(dApp.getAddress().toString(), _to, nextRecvId,
                rollback != null && rollback.length > 0, data);

        CSMessage message = new CSMessage(CSMessage.REQUEST, msg.toBytes());

        Packet packet = new Packet();
        packet.setSequence(nextRecvId);
        packet.setData(message.toBytes());
        packet.setSourcePort(portId);
        packet.setSourceChannel(channelId);
        packet.setDestinationPort(counterPartyPortId);
        packet.setDestinationChannel(counterPartyChannelId);
        packet.setTimeoutHeight(height);
        packet.setTimeoutTimestamp(BigInteger.ZERO);

        return packet;

    }

    protected Packet getRecvRequestPacket(String _from, byte[] data, byte[] rollback) {
        BigInteger nextRecvId = BigInteger.ONE;

        CSMessageRequest msg = new CSMessageRequest(_from, dApp.getAddress().toString(), nextRecvId,
                rollback != null && rollback.length > 0, data);

        CSMessage message = new CSMessage(CSMessage.REQUEST, msg.toBytes());

        return getResponsePacket(nextRecvId, message);

    }

    protected Packet getResponsePacket(int code, String message) {

        CSMessageResponse msg = new CSMessageResponse(BigInteger.ONE, code, message);

        CSMessage _message = new CSMessage(CSMessage.RESPONSE, msg.toBytes());

        return getResponsePacket(BigInteger.ONE, _message);

    }

    private Packet getResponsePacket(BigInteger nextRecvId, CSMessage _message) {

        Height height = new Height();
        height.setRevisionNumber(BigInteger.ZERO);
        height.setRevisionHeight(BigInteger.valueOf(sm.getBlock().getHeight()).add(BigInteger.ONE).add(TIMEOUT_HEIGHT));

        Packet packet = new Packet();
        packet.setSequence(nextRecvId);
        packet.setData(_message.toBytes());
        packet.setSourcePort(counterPartyPortId);
        packet.setSourceChannel(counterPartyChannelId);
        packet.setDestinationPort(portId);
        packet.setDestinationChannel(channelId);
        packet.setTimeoutHeight(height);
        packet.setTimeoutTimestamp(BigInteger.ZERO);

        return packet;
    }
}
