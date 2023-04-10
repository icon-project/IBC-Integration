package foundation.icon.btp.xcall;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.doNothing;

import java.math.BigInteger;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;


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
    @Order(0)
    void setTimeoutHeight_unauthorized() {
        Executable call = () -> client.invoke(sm.createAccount(), "setTimeoutHeight", TIMEOUT_HEIGHT);
        AssertionError e = assertThrows(AssertionError.class, call);
        assertTrue(e.getMessage().contains("Only admin is allowed to call method"));
    }

    @Test
    @Order(1)
    void onChanOpenInit_unauthorized() {
        Executable executable = () -> onChanOpenInit(sm.createAccount());
        AssertionError e = assertThrows(AssertionError.class, executable);
        assertTrue(e.getMessage().contains("Only IBCHandler allowed"));
    }

    @Test
    @Order(2)
    void onChanOpenInit() {
        onChanOpenInit(MOCK_CONTRACT_ADDRESS.get("ibcHandler"));
    }

    @Test
    @Order(3)
    void onChanOpenAck_unauthorized() {
        onChanOpenInit(MOCK_CONTRACT_ADDRESS.get("ibcHandler"));

        Executable executable = () -> onChanOpenAck(sm.createAccount());
        AssertionError e = assertThrows(AssertionError.class, executable);
        assertTrue(e.getMessage().contains("Only IBCHandler allowed"));
    }

    @Test
    @Order(4)
    void onChanOpenAck() {
        onChanOpenInit(MOCK_CONTRACT_ADDRESS.get("ibcHandler"));
        onChanOpenAck(MOCK_CONTRACT_ADDRESS.get("ibcHandler"));
    }


    @Test
    @Order(5)
    void sendCallMessage_withoutRollback() {
        byte[] _data = "sendCallMessageWithoutRollback".getBytes();
        String _to = "to-address";
        sendCallMessage(_to, _data, new byte[0]);
    }


    @Test
    @Order(6)
    void sendCallMessage_withRollback() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        sendCallMessage(_to, _data, _rollback);
    }

    @Test
    @Order(7)
    void onRecvPacket() {
        byte[] _data = "sendCallMessageWithRollback".getBytes();
        String _to = "to-address";
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);
    }


    @Test
    @Order(7)
    void executeCall() {
        //[116, 101, 115, 116, 45, 109, 101, 115, 115, 97, 103, 101]
        byte[] _data = "test-message".getBytes();
        String _to = mockDApp.getAddress().toString();
        byte[] _rollback = "rollback".getBytes();
        onRecvPacket(_to, _data, _rollback);
        System.out.println("_to = " + _to);

        doNothing().when(mockDApp.mock).handleCallMessage(portId + "/" + channelId, _data);

        client.invoke(sm.createAccount(), "executeCall", BigInteger.ONE);

    }

}
