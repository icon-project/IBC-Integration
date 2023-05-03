package foundation.icon.btp.xcall;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.doNothing;
import static org.mockito.Mockito.doThrow;
import static org.mockito.Mockito.verify;

import foundation.icon.btp.xcall.data.CSMessageResponse;
import ibc.icon.interfaces.ICallServiceReceiverScoreInterface;
import java.math.BigInteger;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import org.mockito.MockedConstruction;
import org.mockito.Mockito;
import score.UserRevertedException;


public class CallServiceTest extends CallServiceTestBase {

    @BeforeEach
    public void setup() throws Exception {
        super.setup();
        client.invoke(owner, "setTimeoutHeight", TIMEOUT_HEIGHT);
    }

    @AfterEach
    protected void teardown() {
        super.teardown();
    }


    @Test
    void setTimeoutHeight_unauthorized() {
        Executable call = () -> client.invoke(sm.createAccount(), "setTimeoutHeight", TIMEOUT_HEIGHT);
        AssertionError e = assertThrows(AssertionError.class, call);
        assertTrue(e.getMessage().contains("Only admin is allowed to call method"));
    }

    @Test
    void onChanOpenInit_unauthorized() {
        Executable executable = () -> onChanOpenInit(sm.createAccount());
        AssertionError e = assertThrows(AssertionError.class, executable);
        assertTrue(e.getMessage().contains("Only IBCHandler allowed"));
    }

    @Test
    void onChanOpenInit() {
        onChanOpenInit(ibcHandler);
    }

    @Test
    void onChanOpenAck_unauthorized() {
        onChanOpenInit(ibcHandler);

        Executable executable = () -> onChanOpenAck(sm.createAccount());
        AssertionError e = assertThrows(AssertionError.class, executable);
        assertTrue(e.getMessage().contains("Only IBCHandler allowed"));
    }

    @Test
    void onChanOpenAck() {
        onChanOpenInit(ibcHandler);
        onChanOpenAck(ibcHandler);
    }


    @Test
    void sendCallMessage_withoutRollback() {
        byte[] _data = "sendCallMessageWithoutRollback".getBytes();
        String _to = "to-address";
        sendCallMessage(_to, _data, new byte[0]);
    }


    @Test
    void sendCallMessage_withRollback() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);
    }

    @Test
    void onRecvPacket() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);
    }


    @Test
    void onRecvResponse() {
        byte[] _data = "test-message".getBytes();
        String _to = dApp.getAddress().toString();
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);

        onRecvResponsePacket(CSMessageResponse.SUCCESS, "");
    }

    @Test
    void executeCall_success() {
        byte[] _data = "test-message".getBytes();
        String _to = dApp.getAddress().toString();
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);
        try (MockedConstruction<ICallServiceReceiverScoreInterface> mocked = Mockito.mockConstruction(
                ICallServiceReceiverScoreInterface.class, (mock, context) -> {
                    doNothing().when(mock).handleCallMessage(portId + "/" + channelId, _data);
                })) {
            client.invoke(sm.createAccount(), "executeCall", BigInteger.ONE);

            verify(clientSpy).CallExecuted(BigInteger.ONE, 0, "");
        }

    }

    @Test
    void executeCall_execute_twice() {
        byte[] _data = "test-message".getBytes();
        String _to = dApp.getAddress().toString();
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);

        try (MockedConstruction<ICallServiceReceiverScoreInterface> mocked = Mockito.mockConstruction(
                ICallServiceReceiverScoreInterface.class, (mock, context) -> {
                    doNothing().when(mock).handleCallMessage(counterPartyPortId + "/" + counterPartyChannelId, _data);
                })) {

            client.invoke(sm.createAccount(), "executeCall", BigInteger.ONE);

            Executable executable = () -> client.invoke(sm.createAccount(), "executeCall", BigInteger.ONE);

            AssertionError e = assertThrows(AssertionError.class, executable);
            assertTrue(e.getMessage().contains("InvalidRequestId"));
        }

    }

    @Test
    void executeCall_fail() {
        byte[] _data = "test-message".getBytes();
        String _to = dApp.getAddress().toString();
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);

        try (MockedConstruction<ICallServiceReceiverScoreInterface> mocked = Mockito.mockConstruction(
                ICallServiceReceiverScoreInterface.class, (mock, context) -> {
                    doThrow(new UserRevertedException("Invalid request")).when(mock)
                            .handleCallMessage(counterPartyPortId + "/" + counterPartyChannelId, _data);
                })) {
            client.invoke(sm.createAccount(), "executeCall", BigInteger.ONE);
            verify(clientSpy).CallExecuted(BigInteger.ONE, -1, "UserReverted(0)");
        }
    }

    @Test
    void executeRollback_without_error_response() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);

        Executable executable = () -> client.invoke(sm.createAccount(), "executeRollback", BigInteger.ONE);

        AssertionError e = assertThrows(AssertionError.class, executable);
        assertTrue(e.getMessage().contains("RollbackNotEnabled"));
    }


    @Test
    void executeRollback_success() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);

        onRecvResponsePacket(CSMessageResponse.FAILURE, "Exception");
        verify(clientSpy).RollbackMessage(BigInteger.ONE);

        try (MockedConstruction<ICallServiceReceiverScoreInterface> mocked = Mockito.mockConstruction(
                ICallServiceReceiverScoreInterface.class, (mock, context) -> {
                    doNothing().when(mock).handleCallMessage(portId + "/" + channelId, _rollback);
                })) {
            client.invoke(sm.createAccount(), "executeRollback", BigInteger.ONE);

            verify(clientSpy).RollbackExecuted(BigInteger.ONE, 0, "");
        }
    }


    @Test
    void executeRollback_twice() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);

        onRecvResponsePacket(CSMessageResponse.FAILURE, "Exception");
        verify(clientSpy).RollbackMessage(BigInteger.ONE);

        try (MockedConstruction<ICallServiceReceiverScoreInterface> mocked = Mockito.mockConstruction(
                ICallServiceReceiverScoreInterface.class, (mock, context) -> {
                    doNothing().when(mock).handleCallMessage(portId + "/" + channelId, _rollback);
                })) {
            client.invoke(sm.createAccount(), "executeRollback", BigInteger.ONE);

            verify(clientSpy).RollbackExecuted(BigInteger.ONE, 0, "");

            Executable executable = () -> client.invoke(sm.createAccount(), "executeRollback", BigInteger.ONE);

            AssertionError e = assertThrows(AssertionError.class, executable);
            assertTrue(e.getMessage().contains("InvalidSerialNum"));
        }
    }

}
