package foundation.icon.btp.xcall;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.AdditionalMatchers.aryEq;
import static org.mockito.ArgumentMatchers.anyBoolean;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;
import static org.mockito.Mockito.doThrow;
import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.times;

import java.math.BigInteger;

import javax.swing.text.AbstractDocument.Content;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import foundation.icon.btp.xcall.CSMessage;
import foundation.icon.btp.xcall.CSMessageRequest;
import foundation.icon.btp.xcall.CallServiceImpl;
import foundation.icon.btp.xcall.CallServiceReceiver;
import foundation.icon.btp.xcall.DefaultCallServiceReceiver;
import foundation.icon.btp.xcall.Connection;
import foundation.icon.btp.xcall.NetworkAddress;
import foundation.icon.ee.types.Address;
import ibc.icon.test.MockContract;
import score.UserRevertedException;

public class CallServiceTest extends TestBase {
    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account user = sm.createAccount();

    protected Score xcall;
    protected CallServiceImpl xcallSpy;
    protected static String nid = "0x2.ICON";
    protected static String ethNid = "0x1.ETH";

    protected NetworkAddress ethDapp = new NetworkAddress(ethNid, "0xa");
    protected NetworkAddress iconDappAddress;
    protected String baseEthConnection = "0xb";
    protected MockContract<CallServiceReceiver> dapp;
    protected MockContract<Connection> baseConnection;

    String[] baseSource;
    String[] baseDestination;


    @BeforeEach
    public void setup() throws Exception {
        dapp = new MockContract<>(CallServiceReceiverScoreInterface.class, CallServiceReceiver.class, sm, owner);
        iconDappAddress = new NetworkAddress(nid, dapp.getAddress().toString());
        baseConnection = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        baseSource = new String[] {baseConnection.getAddress().toString()};
        baseDestination = new String[] {baseEthConnection};
        when(baseConnection.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);
        xcall = sm.deploy(owner, CallServiceImpl.class, nid);
        xcallSpy = (CallServiceImpl) spy(xcall.getInstance());
        xcall.setInstance(xcallSpy);
    }

    @Test
    public void sendMessage_singleProtocol() {
        // Arrange
        byte[] data = "test".getBytes();

        // Act
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, null, baseSource, baseDestination);

        // Assert
        CSMessageRequest request = new CSMessageRequest(iconDappAddress.toString(), ethDapp.account.toString(), BigInteger.ONE, false, data, new String[]{baseEthConnection});
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        verify(baseConnection.mock).sendMessage(eq(ethNid), eq("xcall"), eq(BigInteger.ZERO), aryEq(msg.toBytes()));
        verify(xcallSpy).CallMessageSent(dapp.getAddress(), ethDapp.toString(), BigInteger.ONE);
    }

    @Test
    public void sendMessage_defaultProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());

        // Act
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data);

        // Assert
        CSMessageRequest request = new CSMessageRequest(iconDappAddress.toString(), ethDapp.account.toString(), BigInteger.ONE, false, data, null);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        verify(baseConnection.mock).sendMessage(eq(ethNid), eq("xcall"), eq(BigInteger.ZERO), aryEq(msg.toBytes()));
        verify(xcallSpy).CallMessageSent(dapp.getAddress(), ethDapp.toString(), BigInteger.ONE);
    }

    @Test
    public void sendMessage_multiProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        when(connection1.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);
        when(connection2.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);

        String[] destinations = {"0x1eth", "0x2eth"};
        String[] sources = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        // Act
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, null, sources, destinations);

        // Assert
        CSMessageRequest request = new CSMessageRequest(iconDappAddress.toString(), ethDapp.account.toString(), BigInteger.ONE, false, data, destinations);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        verify(connection1.mock).sendMessage(eq(ethNid), eq("xcall"), eq(BigInteger.ZERO), aryEq(msg.toBytes()));
        verify(connection2.mock).sendMessage(eq(ethNid), eq("xcall"), eq(BigInteger.ZERO), aryEq(msg.toBytes()));
        verify(xcallSpy).CallMessageSent(dapp.getAddress(), ethDapp.toString(), BigInteger.ONE);
    }

    @Test
    public void handleResponse_singleProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, new String[]{baseConnection.getAddress().toString()});
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());

        // Act
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Assert
        verify(xcallSpy).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
    }

    @Test
    public void handleResponse_singleProtocol_invalidSender() {
        // Arrange
        byte[] data = "test".getBytes();
        Account otherConnection = sm.createAccount();
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data,  new String[]{baseConnection.getAddress().toString()});
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());

        // Act
        assertThrows(Exception.class, ()->xcall.invoke(otherConnection, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes()));

        // Assert
        verify(xcallSpy, times(0)).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
    }

    @Test
    public void handleResponse_defaultProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, null);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());

        // Act
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Assert
        verify(xcallSpy).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
    }

    @Test
    public void handleResponse_defaultProtocol_invalidSender() {
        // Arrange
        byte[] data = "test".getBytes();
        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, null);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        Account invalidConnection  = sm.createAccount();

        // Act
        assertThrows(Exception.class, ()-> xcall.invoke(invalidConnection, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes()));

        // Assert
        verify(xcallSpy, times(0)).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
    }

    @Test
    public void handleResponse_multiProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        String[] connections = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, connections);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());

        // Act
        xcall.invoke(connection1.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());
        verify(xcallSpy, times(0)).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
        xcall.invoke(connection2.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Assert
        verify(xcallSpy, times(1)).CallMessage(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, BigInteger.ONE);
    }

    @Test
    public void executeCall_singleProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, new String[]{baseConnection.getAddress().toString()});
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Act
        xcall.invoke(user, "executeCall", BigInteger.ONE);

        // Assert
        verify(dapp.mock).handleCallMessage(ethDapp.toString(), data, new String[]{baseConnection.getAddress().toString()});
        verify(xcallSpy).CallExecuted(BigInteger.ONE, 0, "");
    }

    @Test
    public void executeCall_defaultProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        MockContract<DefaultCallServiceReceiver> defaultDapp = new MockContract<>(DefaultCallServiceReceiverScoreInterface.class, DefaultCallServiceReceiver.class, sm, owner);
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), defaultDapp.getAddress().toString(), BigInteger.ONE, false, data, null);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());

        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Act
        xcall.invoke(user, "executeCall", BigInteger.ONE);

        // Assert
        verify(defaultDapp.mock).handleCallMessage(ethDapp.toString(), data);
        verify(xcallSpy).CallExecuted(BigInteger.ONE, 0, "");
    }

    @Test
    public void executeCall_multiProtocol() throws Exception{
        // Arrange
        byte[] data = "test".getBytes();
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        String[] connections = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, false, data, connections);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        xcall.invoke(connection1.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());
        xcall.invoke(connection2.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Act
        xcall.invoke(user, "executeCall", BigInteger.ONE);

        // Assert
        verify(dapp.mock).handleCallMessage(ethDapp.toString(), data, connections);
        verify(xcallSpy).CallExecuted(BigInteger.ONE, 0, "");
    }

    @Test
    public void executeCall_multiProtocol_rollback() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        String[] connections = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, true, data, connections);
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        xcall.invoke(connection1.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());
        xcall.invoke(connection2.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());

        // Act
        xcall.invoke(user, "executeCall", BigInteger.ONE);

        // Assert
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.SUCCESS, "");
        msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());

        verify(dapp.mock).handleCallMessage(ethDapp.toString(), data, connections);
        verify(connection1.mock).sendMessage(ethNid, "xcall", BigInteger.ONE.negate(), msg.toBytes());
        verify(connection2.mock).sendMessage(ethNid, "xcall", BigInteger.ONE.negate(), msg.toBytes());
        verify(xcallSpy).CallExecuted(BigInteger.ONE, 0, "");
    }

    @Test
    public void executeCall_failedExecution() {
        // Arrange
        byte[] data = "test".getBytes();
        CSMessageRequest request = new CSMessageRequest(ethDapp.toString(), dapp.getAddress().toString(), BigInteger.ONE, true, data, new String[]{baseConnection.getAddress().toString()});
        CSMessage msg = new CSMessage(CSMessage.REQUEST, request.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ZERO, msg.toBytes());
        // Act
        doThrow(new UserRevertedException()).when(dapp.mock).handleCallMessage(ethDapp.toString(), data, new String[]{baseConnection.getAddress().toString()});
        xcall.invoke(user, "executeCall", BigInteger.ONE);

        // Assert
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  "score.RevertedException");
        msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        verify(baseConnection.mock).sendMessage(ethNid, "xcall", BigInteger.ONE.negate(), msg.toBytes());
        verify(xcallSpy).CallExecuted(BigInteger.ONE, -1, "score.RevertedException");
    }


    @Test
    public void rollback_singleProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, baseSource, baseDestination);
        String errorMessage = "errorMessage";

        // Act
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Assert
        verify(xcallSpy).ResponseMessage(BigInteger.ONE, CSMessageResponse.FAILURE, errorMessage);
        verify(xcallSpy).RollbackMessage(BigInteger.ONE);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void rollback_defaultProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback);
        String errorMessage = "errorMessage";

        // Act
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Assert
        verify(xcallSpy).ResponseMessage(BigInteger.ONE, CSMessageResponse.FAILURE, errorMessage);
        verify(xcallSpy).RollbackMessage(BigInteger.ONE);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void rollback_defaultProtocol_invalidSender() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback);
        String errorMessage = "errorMessage";
        Account invalidConnection  = sm.createAccount();

        // Act
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        assertThrows(Exception.class, ()->  xcall.invoke(invalidConnection, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes()));

        // Assert
        verify(xcallSpy, times(0)).ResponseMessage(BigInteger.ONE, CSMessageResponse.FAILURE, errorMessage);
        verify(xcallSpy, times(0)).RollbackMessage(BigInteger.ONE);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void rollback_multiProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        when(connection1.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);
        when(connection2.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);

        String[] destinations = {"0x1eth", "0x2eth"};
        String[] sources = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, sources, destinations);
        String errorMessage = "errorMessage";

        // Act
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(connection1.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());
        verify(xcallSpy, times(0)).ResponseMessage(BigInteger.ONE, CSMessageResponse.FAILURE, errorMessage);
        xcall.invoke(connection2.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Assert
        verify(xcallSpy).ResponseMessage(BigInteger.ONE, CSMessageResponse.FAILURE, errorMessage);
        verify(xcallSpy).RollbackMessage(BigInteger.ONE);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void rollback_success() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, baseSource, baseDestination);

        // Act
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.SUCCESS,  "");
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Assert
        verify(xcallSpy).ResponseMessage(BigInteger.ONE, CSMessageResponse.SUCCESS, "");
        verify(xcallSpy, times(0)).RollbackMessage(BigInteger.ONE);

        assertTrue(xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }


    @Test
    public void executeRollback_singleProtocol() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        NetworkAddress xcallAddr = new NetworkAddress(nid, xcall.getAddress());

        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, baseSource, baseDestination);

        String errorMessage = "errorMessage";
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Act
        xcall.invoke(user, "executeRollback", BigInteger.ONE);

        // Assert
        verify(xcallSpy).RollbackExecuted(BigInteger.ONE, 0, "");
        verify(dapp.mock).handleCallMessage(xcallAddr.toString(), rollback, new String[]{baseConnection.getAddress().toString()});
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void executeRollback_failedExecution() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        NetworkAddress xcallAddr = new NetworkAddress(nid, xcall.getAddress());

        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, baseSource, baseDestination);

        String errorMessage = "errorMessage";
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Act
        doThrow(new UserRevertedException()).when(dapp.mock).handleCallMessage(xcallAddr.toString(), rollback, new String[]{baseConnection.getAddress().toString()});
        xcall.invoke(user, "executeRollback", BigInteger.ONE);

        // Assert
        verify(xcallSpy).RollbackExecuted(BigInteger.ONE, CSMessageResponse.FAILURE, "score.RevertedException");
        verify(dapp.mock).handleCallMessage(xcallAddr.toString(), rollback, new String[]{baseConnection.getAddress().toString()});
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void executeRollback_defaultProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        MockContract<DefaultCallServiceReceiver> defaultDapp = new MockContract<>(DefaultCallServiceReceiverScoreInterface.class, DefaultCallServiceReceiver.class, sm, owner);
        NetworkAddress xcallAddr = new NetworkAddress(nid, xcall.getAddress());

        xcall.invoke(owner, "setDefaultConnection", ethDapp.net(), baseConnection.getAddress());
        xcall.invoke(defaultDapp.account, "sendCallMessage", ethDapp.toString(), data, rollback);

        String errorMessage = "errorMessage";
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(baseConnection.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Act
        xcall.invoke(user, "executeRollback", BigInteger.ONE);

        // Assert
        verify(xcallSpy).RollbackExecuted(BigInteger.ONE, 0, "");
        verify(defaultDapp.mock).handleCallMessage(xcallAddr.toString(), rollback);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void executeRollback_multiProtocol() throws Exception {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] rollback = "rollback".getBytes();
        NetworkAddress xcallAddr = new NetworkAddress(nid, xcall.getAddress());

        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        when(connection1.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);
        when(connection2.mock.getFee(anyString(), anyBoolean())).thenReturn(BigInteger.ZERO);

        String[] destinations = {"0x1eth", "0x2eth"};
        String[] sources = {connection1.getAddress().toString(), connection2.getAddress().toString()};

        xcall.invoke(dapp.account, "sendCallMessage", ethDapp.toString(), data, rollback, sources, destinations);
        String errorMessage = "errorMessage";
        CSMessageResponse msgRes = new CSMessageResponse(BigInteger.ONE, CSMessageResponse.FAILURE,  errorMessage);
        CSMessage msg = new CSMessage(CSMessage.RESPONSE, msgRes.toBytes());
        xcall.invoke(connection1.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());
        xcall.invoke(connection2.account, "handleBTPMessage", ethNid, "xcall", BigInteger.ONE, msg.toBytes());

        // Act
        xcall.invoke(user, "executeRollback", BigInteger.ONE);


        // Assert
        verify(xcallSpy).RollbackExecuted(BigInteger.ONE, CSMessageResponse.SUCCESS, "");
        verify(dapp.mock).handleCallMessage(xcallAddr.toString(), rollback, sources);
        assertTrue(!xcall.call(Boolean.class, "verifySuccess", BigInteger.ONE));
    }

    @Test
    public void getFee() throws Exception {
        // Arrange
        String nid = "nid";
        BigInteger fee1 = BigInteger.valueOf(3);
        BigInteger fee2 = BigInteger.valueOf(5);
        BigInteger protocolFee = BigInteger.TEN;
        MockContract<Connection>  connection1 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        MockContract<Connection>  connection2 = new MockContract<>(ConnectionScoreInterface.class, Connection.class, sm, owner);
        when(connection1.mock.getFee(nid, true)).thenReturn(fee1);
        when(connection2.mock.getFee(nid, true)).thenReturn(fee2);

        String[] sources = {connection1.getAddress().toString(), connection2.getAddress().toString()};
        xcall.invoke(owner, "setProtocolFee", protocolFee);

        // Act
        BigInteger fee = xcall.call(BigInteger.class, "getFee", nid, true, sources);


        // Assert
        assertEquals(fee1.add(fee2).add(protocolFee), fee);
    }

    @Test
    public void getFee_default() throws Exception {
        // Arrange
        String nid = "nid";
        BigInteger fee1 = BigInteger.valueOf(3);
        BigInteger protocolFee = BigInteger.TEN;
        when(baseConnection.mock.getFee(nid, true)).thenReturn(fee1);

        xcall.invoke(owner, "setDefaultConnection", nid, baseConnection.getAddress());
        xcall.invoke(owner, "setProtocolFee", protocolFee);

        // Act
        BigInteger fee = xcall.call(BigInteger.class, "getFee", nid, true);


        // Assert
        assertEquals(fee1.add(protocolFee), fee);
    }
}