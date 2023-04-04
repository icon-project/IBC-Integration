package ibc.ics25.handler;

import com.iconloop.score.test.Account;
import ibc.icon.structs.messages.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import test.proto.core.channel.ChannelOuterClass.Packet;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.*;

public class IBCHandlerTest extends IBCHandlerTestBase {
    @BeforeEach
    public void setup() throws Exception {
        super.setup();
    }

    @Test
    void name() {
        assertEquals(IBCHandler.name, handler.call("name"));
    }

    @Test
    void establishCommunication() throws Exception {
        createClient();

        createConnection();
        acknowledgeConnection();

        openChannel();
        acknowledgeChannel();

        updateClient();
    }

    @Test
    void connection_FromCounterparty() throws Exception {
        createClient();

        tryOpenConnection();
        confirmConnection();

        tryOpenChannel();
        confirmChannel();

        confirmCloseChannel();
    }

    @Test
    void connection_ChannelFromCounterparty() throws Exception {
        createClient();

        createConnection();
        acknowledgeConnection();

        tryOpenChannel();
        confirmChannel();

        closeChannel();
    }

    @Test
    void connection_ConnectionFromCounterparty() throws Exception {
        createClient();

        tryOpenConnection();
        confirmConnection();

        openChannel();
        acknowledgeChannel();

        confirmCloseChannel();
    }

    @Test
    void receivePackets_withSeparateAck() throws Exception {
        establishCommunication();

        receivePacket();
        writeAcknowledgement();
    }

    @Test
    void receivePackets_withAckResponse() throws Exception {
        establishCommunication();

        receivePacket_withAcK();
    }

    @Test
    void sendAndAckPacket() throws Exception {
        establishCommunication();

        sendPacket();
        acknowledgePacket();
    }

    @Test
    void sendAndTimeoutPacket() throws Exception {
        establishCommunication();

        sendPacket();
        timeoutPacket();
    }

    @Test
    void requestTimeoutPacket() throws Exception {
        establishCommunication();

        requestTimeout(getBaseCounterPacket());
    }

    @Test
    void channel_WithoutPortAllocations() throws Exception {
        // Arrange
        createClient();

        tryOpenConnection();
        confirmConnection();

        MsgChannelOpenInit msgInit = new MsgChannelOpenInit();
        MsgChannelOpenTry msgTry = new MsgChannelOpenTry();
        MsgChannelOpenAck msgAck = new MsgChannelOpenAck();
        MsgChannelOpenConfirm msgConfirm = new MsgChannelOpenConfirm();
        MsgChannelCloseInit msgCloseInit = new MsgChannelCloseInit();
        MsgChannelCloseConfirm msgCloseConfirm = new MsgChannelCloseConfirm();

        msgInit.setPortId(portId);
        msgTry.setPortId(portId);
        msgAck.setPortId(portId);
        msgConfirm.setPortId(portId);
        msgCloseInit.setPortId(portId);
        msgCloseConfirm.setPortId(portId);

        // Act && Assert
        String expectedErrorMessage = "Module not found";
        Executable openInit = () -> handler.invoke(module.account, "channelOpenInit", msgInit);
        Executable openTry = () -> handler.invoke(module.account, "channelOpenTry", msgTry);
        Executable openAck = () -> handler.invoke(module.account, "channelOpenAck", msgAck);
        Executable openConfirm = () -> handler.invoke(module.account, "channelOpenConfirm", msgConfirm);
        Executable closeInit = () -> handler.invoke(module.account, "channelCloseInit", msgCloseInit);
        Executable closeConfirm = () -> handler.invoke(module.account, "channelCloseConfirm", msgCloseConfirm);

        AssertionError e = assertThrows(AssertionError.class, openInit);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
        e = assertThrows(AssertionError.class, openTry);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
        e = assertThrows(AssertionError.class, openAck);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
        e = assertThrows(AssertionError.class, openConfirm);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
        e = assertThrows(AssertionError.class, closeInit);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
        e = assertThrows(AssertionError.class, closeConfirm);
        assertTrue(e.getMessage().contains(expectedErrorMessage));

    }

    @Test
    void sendPacket_WithoutAuthorization() throws Exception {
        // Arrange
        establishCommunication();
        Account nonAuthModule = sm.createAccount();
        Packet packet = getBasePacket();

        // Act && Assert
        String expectedErrorMessage = "failed to authenticate " + nonAuthModule.getAddress();
        Executable sendNonAuthPacket = () -> handler.invoke(nonAuthModule, "sendPacket", packet.toByteArray());
        AssertionError e = assertThrows(AssertionError.class, sendNonAuthPacket);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void writePacketAck_WithoutAuthorization() throws Exception {
        // Arrange
        Account nonAuthModule = sm.createAccount();
        byte[] acknowledgement = new byte[1];
        establishCommunication();

        // Act
        receivePacket();
        Packet lastPacket = Packet.parseFrom(lastPacketCaptor.getValue());

        // Assert
        String expectedErrorMessage = "failed to authenticate " + nonAuthModule.getAddress();
        Executable nonAuthPacketAck = () -> handler.invoke(nonAuthModule, "writeAcknowledgement",
                lastPacket.getDestinationPort(), lastPacket.getDestinationChannel(),
                BigInteger.valueOf(lastPacket.getSequence()), acknowledgement);
        AssertionError e = assertThrows(AssertionError.class, nonAuthPacketAck);
        assertTrue(e.getMessage().contains(expectedErrorMessage));

    }

    @Test
    void setExpectedTimePerBlock() throws Exception {
        // Arrange
        // 10 seconds delay
        delayPeriod = BigInteger.valueOf(10).multiply(BigInteger.TEN.pow(6));
        // 2 second block time
        BigInteger expectedTimePerBlock = BigInteger.TWO.multiply(BigInteger.TEN.pow(6));
        BigInteger expectedDelayTime = delayPeriod.add(expectedTimePerBlock).subtract(BigInteger.ONE)
                .divide(expectedTimePerBlock);

        establishCommunication();
        Packet packet = getBaseCounterPacket();

        MsgPacketRecv msg = new MsgPacketRecv();
        msg.setPacket(packet.toByteArray());
        msg.setProof(new byte[0]);
        msg.setProofHeight(baseHeight.toByteArray());

        when(module.mock.onRecvPacket(msg.getPacket(), relayer.getAddress())).thenReturn(new byte[0]);

        // Act
        handler.invoke(owner, "setExpectedTimePerBlock", expectedTimePerBlock);
        handler.invoke(relayer, "recvPacket", msg);

        verify(lightClient.mock).verifyMembership(any(String.class), any(byte[].class), eq(delayPeriod),
                eq(expectedDelayTime), any(byte[].class), any(byte[].class), any(byte[].class), any(byte[].class));
    }

    @Test
    void handlerAdminPermissions() {
        // TODO: should be a admin and not a owner.
        assertOnlyCallableBy(owner, "bindPort", portId);
        assertOnlyCallableBy(owner, "registerClient", clientType, lightClient.getAddress());
        assertOnlyCallableBy(owner, "setExpectedTimePerBlock", BigInteger.TWO);
    }

    private void assertOnlyCallableBy(Account authorizedCaller, String method, Object... params) {
        Account nonOwner = sm.createAccount();
        String expectedErrorMessage = "Reverted(0): SenderNotScoreOwner: Sender=" + nonOwner.getAddress()
                + "Authorized=" + authorizedCaller.getAddress();
        Executable notAuthorizedCall = () -> handler.invoke(nonOwner, method, params);
        AssertionError e = assertThrows(AssertionError.class, notAuthorizedCall);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }
}
