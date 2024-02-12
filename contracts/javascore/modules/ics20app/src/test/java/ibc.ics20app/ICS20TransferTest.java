package ibc.ics20app;


import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.ics23.commitment.Ops;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;
import score.Address;
import score.Context;

import java.math.BigInteger;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.CALLS_REAL_METHODS;
import static org.mockito.Mockito.spy;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import org.junit.jupiter.api.*;
import org.junit.jupiter.api.function.Executable;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import org.mockito.stubbing.Answer;


public class ICS20TransferTest extends TestBase {
    public static final Address SYSTEM_ADDRESS = Address.fromString("cx0000000000000000000000000000000000000000");
    public static final Address ICS20Bank = Address.fromString("cx0000000000000000000000000000000000000002");
    public static final Address IBCHandler = Address.fromString("cx0000000000000000000000000000000000000003");
    public static final Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");

    public static final ServiceManager sm = getServiceManager();
    public static final Account owner = sm.createAccount();
    public static final Account testingAccount = sm.createAccount();
    public static final String TAG = "ICS20App";
    public Score ics20App;
    ICS20TransferBank ICS20TransferBankSpy;

    public static MockedStatic<Context> contextMock;

    @BeforeEach
    public void setup() throws Exception {
        ics20App = sm.deploy(owner, ICS20TransferBank.class, IBCHandler, ICS20Bank);

        ICS20TransferBank instance = (ICS20TransferBank) ics20App.getInstance();
        ICS20TransferBankSpy = spy(instance);
        ics20App.setInstance(ICS20TransferBankSpy);
        contextMock.reset();
    }

    @BeforeAll
    public static void init(){
        contextMock = Mockito.mockStatic(Context.class, CALLS_REAL_METHODS);
    }


    public void expectErrorMessage(Executable contractCall, String errorMessage) {
        AssertionError e = Assertions.assertThrows(AssertionError.class, contractCall);
        assertEquals(errorMessage, e.getMessage());
    }

    @Test
    void getBank(){
        assertEquals(ICS20Bank, ics20App.call("getBank"));
    }

    @Test
    void getIBCAddress(){
        assertEquals(IBCHandler, ics20App.call("getIBCAddress"));
    }

//    @Test
//    void getDestinationPort(){
//        String channelId = "channel-0";
//        assertEquals("transfer", ics20App.call("getDestinationPort", channelId));
//    }
//
//    @Test
//    void getDestinationChannel(){
//        String channelId = "channel-0";
//        assertEquals("channel-1", ics20App.call("getDestinationChannel", channelId));
//    }

    @Test
    void onlyIBCFailure(){
        expectErrorMessage(
                () -> ics20App.invoke(testingAccount, "onlyIBC"),
                "Reverted(0): ICS20App: Caller is not IBC Contract"
        );
    }

    @Test
    void onlyIBCSuccess(){
        contextMock.when(Context::getCaller).thenReturn(IBCHandler);
        ics20App.invoke(owner, "onlyIBC");
    }


//    @Test
//    void onRecvPacket() {
//        BigInteger sequence = BigInteger.ONE;
//        String sourcePort = "transfer";
//        String sourceChannel = "channel-0";
//        String destinationPort = "transfer";
//        String destinationChannel = "channel-1";
//        byte[] data = "{\"amount\":\"99000000000000000\",\"denom\":\"stake\",\"receiver\":\"hxb6b5791be0b5ef67063b3c10b840fb81514db2fd\",\"sender\":\"centauri1g5r2vmnp6lta9cpst4lzc4syy3kcj2ljte3tlh\"}".getBytes();
//        BigInteger timeoutTimestamp = BigInteger.ZERO;
//        BigInteger timeoutHeight = BigInteger.ZERO;
//
//        Height height = new Height();
//        height.setRevisionNumber(timeoutHeight);
//        height.setRevisionHeight(timeoutHeight);
//
//        Packet packet = new Packet();
//        packet.setSequence(sequence);
//        packet.setSourcePort(sourcePort);
//        packet.setSourceChannel(sourceChannel);
//        packet.setDestinationPort(destinationPort);
//        packet.setDestinationChannel(destinationChannel);
//        packet.setTimeoutHeight(new Height());
//        packet.setTimeoutTimestamp(timeoutTimestamp);
//        packet.setData(data);
//
//        byte[] packetBytes = packet.encode();
//
//        contextMock.when(caller()).thenReturn(IBCHandler);
//        ics20App.invoke(owner, "onRecvPacket", packetBytes, ZERO_ADDRESS);
//
//    }

    public MockedStatic.Verification caller(){
        return () -> Context.getCaller();
    }


}