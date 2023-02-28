package ibc.ics25.handler;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.when;
import static org.mockito.Mockito.any;
import static org.mockito.Mockito.eq;
import static org.mockito.Mockito.verify;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import com.iconloop.score.test.Account;

import ibc.icon.structs.messages.*;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;

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
    void establishCommunication() {
        createClient();

        createConnection();
        acknowledgeConnection();

        openChannel();
        acknowledgeChannel();

        updateClient();
    }

    @Test
    void connection_FromCounterparty() {
        createClient();

        tryOpenConnection();
        confirmConnection();

        tryOpenChannel();
        confirmChannel();

        confirmCloseChannel();
    }

    @Test
    void connection_ChannelFromCounterparty() {
        createClient();

        createConnection();
        acknowledgeConnection();

        tryOpenChannel();
        confirmChannel();

        closeChannel();
    }

    @Test
    void connection_ConnectionFromCounterparty() {
        createClient();

        tryOpenConnection();
        confirmConnection();

        openChannel();
        acknowledgeChannel();

        confirmCloseChannel();
    }

    @Test
    void receivePackets_withSeparateAck() {
        establishCommunication();

        receivePacket();
        writeAcknowledgement();
    }

    @Test
    void receivePackets_withAckResponse() {
        establishCommunication();

        receivePacket_withAcK();
    }

    @Test
    void sendAndAckPacket() {
        establishCommunication();

        sendPacket();
        acknowledgePacket();
    }

    @Test
    void channel_WithoutPortAllocations() {
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

        msgInit.portId = portId;
        msgTry.portId = portId;
        msgAck.portId = portId;
        msgConfirm.portId = portId;
        msgCloseInit.portId = portId;
        msgCloseConfirm.portId = portId;

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
    void sendPacket_WithoutAuthorization() {
        // Arrange
        establishCommunication();
        Account nonAuthModule = sm.createAccount();
        Packet packet = getBasePacket();

        // Act && Assert
        String expectedErrorMessage = "failed to authenticate " + nonAuthModule.getAddress();
        Executable sendNonAuthPacket = () -> handler.invoke(nonAuthModule, "sendPacket", packet);
        AssertionError e = assertThrows(AssertionError.class, sendNonAuthPacket);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void writePacketAck_WithoutAuthorization() {
        // Arrange
        Account nonAuthModule = sm.createAccount();
        byte[] acknowledgement = new byte[1];
        establishCommunication();

        // Act
        receivePacket();
        Packet lastPacket = Packet.fromBytes(lastPacketCaptor.getValue());

        // Assert
        String expectedErrorMessage = "failed to authenticate " + nonAuthModule.getAddress();
        Executable nonAuthPacketAck = () -> handler.invoke(nonAuthModule, "writeAcknowledgement",
                lastPacket.getDestinationPort(), lastPacket.getDestinationChannel(), lastPacket.getSequence(),
                acknowledgement);
        AssertionError e = assertThrows(AssertionError.class, nonAuthPacketAck);
        assertTrue(e.getMessage().contains(expectedErrorMessage));

    }

    @Test
    void setExpectedTimePerBlock() {
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
        msg.packet = packet;
        msg.proof = new byte[0];
        msg.proofHeight = new Height(BigInteger.ONE, BigInteger.ONE);

        when(module.mock.onRecvPacket(msg.packet, relayer.getAddress())).thenReturn(new byte[0]);

        // Act
        handler.invoke(owner, "setExpectedTimePerBlock", expectedTimePerBlock);
        handler.invoke(relayer, "recvPacket", msg);

        verify(lightClient.mock).verifyMembership(any(String.class), any(Height.class), eq(delayPeriod),
                eq(expectedDelayTime), any(byte[].class), any(String.class), any(byte[].class), any(byte[].class));
    }

    @Test
    void handlerAdminPermissions() {
        // TODO: should porbably be a admin and not a owner.
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
