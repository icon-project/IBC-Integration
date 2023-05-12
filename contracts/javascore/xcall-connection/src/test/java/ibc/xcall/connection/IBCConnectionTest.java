package ibc.xcall.connection;

import static org.mockito.AdditionalMatchers.aryEq;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.ServiceManager;

import icon.proto.core.channel.Packet;
import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;
import org.mockito.Mockito;

public class IBCConnectionTest extends IBCConnectionTestBase {

    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account relayer = sm.createAccount();

    @BeforeEach
    public void setup() throws Exception {
      super.setup();
    }

    ArgumentCaptor<byte[]> packetCaptor = ArgumentCaptor.forClass(byte[].class);

    @Test
    public void sendMessage_noResponse() {
        // Arrange
        String counterpartyNid = "0x1.ETH";
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        String channel = "channel-1";
        String counterpartyChannel = "channel-2";
        establishConnection(channel, counterpartyChannel, counterpartyNid);

        // Act
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel)).thenReturn(seq);
        connection.invoke(xcall.account, "sendMessage", counterpartyNid, "", BigInteger.ZERO, data);

        // Assert
        verify(ibc.mock).sendPacket(packetCaptor.capture());
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(IBCConnection.PORT, packet.getSourcePort());
        assertEquals(IBCConnection.PORT, packet.getDestinationPort());
        assertEquals(channel, packet.getSourceChannel());
        assertEquals(counterpartyChannel, packet.getDestinationChannel());

        Message msg = Message.fromBytes(packet.getData());
        assertEquals(BigInteger.ZERO, msg.getSn());
        assertArrayEquals(data, msg.getData());
    }

    @Test
    public void sendMessage_withResponse() {
        // Arrange
        String counterpartyNid = "0x1.ETH";
        byte[] data = "test".getBytes();
        byte[] ack = "ack".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.TWO;
        String channel = "channel-1";
        String counterpartyChannel = "channel-2";
        establishConnection_fromCounterparty(channel, counterpartyChannel, counterpartyNid);
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel)).thenReturn(seq);
        connection.invoke(xcall.account, "sendMessage", counterpartyNid, "", sn, data);
        verify(ibc.mock).sendPacket(packetCaptor.capture());

        // Act
        connection.invoke(ibc.account, "onAcknowledgementPacket", packetCaptor.getValue(), ack, relayer.getAddress());

        // Assert
        verify(xcall.mock).handleBTPMessage(counterpartyNid, "xcall", sn, ack);
    }

    @Test
    public void sendMessage_multipleConnection() {
        // Arrange
        String counterpartyNid1 = "0x1.ETH";
        String counterpartyNid2 = "0x1.BSC";
        byte[] data = "test".getBytes();
        BigInteger seq1 = BigInteger.TEN;
        BigInteger seq2 = BigInteger.TWO;
        String channel1 = "channel-1";
        String channel2 = "channel-2";
        String counterpartyChannel1 = "channel-2";
        String counterpartyChannel2 = "channel-2";
        BigInteger sn1 = BigInteger.ZERO;
        BigInteger sn2 = BigInteger.TWO;

        establishConnection(channel1, counterpartyChannel1, counterpartyNid1);
        establishConnection_fromCounterparty(channel2, counterpartyChannel2, counterpartyNid2);

        // Act
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel1)).thenReturn(seq1);
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel2)).thenReturn(seq2);
        connection.invoke(xcall.account, "sendMessage", counterpartyNid1, "", sn1, data);
        connection.invoke(xcall.account, "sendMessage", counterpartyNid2, "", sn2, data);

        // Assert
        verify(ibc.mock, Mockito.times(2)).sendPacket(packetCaptor.capture());
        Packet packet1 = Packet.decode(packetCaptor.getAllValues().get(0));
        assertEquals(seq1, packet1.getSequence());
        assertEquals(channel1, packet1.getSourceChannel());
        assertEquals(counterpartyChannel1, packet1.getDestinationChannel());

        Message msg1 = Message.fromBytes(packet1.getData());
        assertEquals(sn1, msg1.getSn());
        assertArrayEquals(data, msg1.getData());

        Packet packet2 = Packet.decode(packetCaptor.getAllValues().get(1));
        assertEquals(seq2, packet2.getSequence());
        assertEquals(channel2, packet2.getSourceChannel());
        assertEquals(counterpartyChannel2, packet2.getDestinationChannel());

        Message msg2 = Message.fromBytes(packet2.getData());
        assertEquals(sn2, msg2.getSn());
        assertArrayEquals(data, msg2.getData());

        // Act
        packet1.setSequence(packet2.getSequence());
        assertThrows(AssertionError.class, () -> connection.invoke(ibc.account, "onAcknowledgementPacket", packet1.encode(), "ack".getBytes(), relayer.getAddress()));
        connection.invoke(ibc.account, "onAcknowledgementPacket", packet2.encode(), "ack".getBytes(), relayer.getAddress());

        // Assert
        verify(xcall.mock).handleBTPMessage(counterpartyNid2, "xcall", sn2, "ack".getBytes());
    }

    @Test
    public void sendMessage_withTimeout() {
        // Arrange
        String counterpartyNid = "0x1.ETH";
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.TWO;
        String channel = "channel-1";
        String counterpartyChannel = "channel-2";
        establishConnection(channel, counterpartyChannel, counterpartyNid);
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel)).thenReturn(seq);
        connection.invoke(xcall.account, "sendMessage", counterpartyNid, "", sn, data);
        verify(ibc.mock).sendPacket(packetCaptor.capture());

        // Act
        connection.invoke(ibc.account, "onTimeoutPacket", packetCaptor.getValue(), relayer.getAddress());

        // Assert
        verify(xcall.mock).handleBTPError("", "xcall", sn, -1, "Timeout");
    }

    @Test
    public void recvMessage_noResponse() {
        // Arrange
        String counterpartyNid = "0x1.ETH";
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.ZERO;
        String channel = "channel-1";
        String counterpartyChannel = "channel-2";
        establishConnection_fromCounterparty(channel, counterpartyChannel, counterpartyNid);

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(sn, data).toBytes());
        pct.setSourcePort(IBCConnection.PORT);
        pct.setSourceChannel(counterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(channel);

        // Act
        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());


        // Assert
        verify(xcall.mock).handleBTPMessage(counterpartyNid, "xcall", sn, data);
    }

    @Test
    public void recvMessage_withResponse() {
        // Arrange
        String counterpartyNid = "0x1.ETH";
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.ONE;
        String channel = "channel-1";
        String counterpartyChannel = "channel-2";
        establishConnection(channel, counterpartyChannel, counterpartyNid);

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(sn, data).toBytes());
        pct.setSourcePort(IBCConnection.PORT);
        pct.setSourceChannel(counterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(channel);

        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());

        // Act
        connection.invoke(xcall.account, "sendMessage", counterpartyNid, "", sn.negate(), data);

        // Assert
        verify(ibc.mock).writeAcknowledgement(packetCaptor.capture(), aryEq(data));
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(channel, packet.getDestinationChannel());
        assertEquals(IBCConnection.PORT, packet.getDestinationPort());
    }

    @Test
    public void entryPermissions() {
        String expectedErrorMessage = "Reverted(0): Only IBCHandler allowed";
        Account nonAuthorized = sm.createAccount();
        AssertionError e;

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanOpenInit", ORDER,  new String[]{"TODO"}, IBCConnection.PORT, "channelId", new byte[0], "version-TODO"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanOpenTry", ORDER,  new String[]{"TODO"}, IBCConnection.PORT, "channelId",  new byte[0], "version-TODO", "version-TODO"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanOpenAck", IBCConnection.PORT, "channelId", "counterpartyChannelId",  "version-TODO"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanOpenConfirm", IBCConnection.PORT, "channelId"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanCloseInit", IBCConnection.PORT, "channelId"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onChanCloseConfirm", IBCConnection.PORT, "channelId"));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onAcknowledgementPacket", new byte[0], new byte[0], relayer.getAddress()));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onRecvPacket", new byte[0],  relayer.getAddress()));
        assertEquals(expectedErrorMessage, e.getMessage());

        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "onTimeoutPacket", new byte[0], relayer.getAddress()));
        assertEquals(expectedErrorMessage, e.getMessage());

        expectedErrorMessage = "Reverted(0): Only XCall allowed";
        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "sendMessage", "", "", BigInteger.ZERO, new byte[0]));
        assertEquals(expectedErrorMessage, e.getMessage());

        expectedErrorMessage = "Reverted(0): Only Admin allowed";
        e = assertThrows(AssertionError.class,
            () -> connection.invoke(nonAuthorized, "configureChannel", "", ""));
        assertEquals(expectedErrorMessage, e.getMessage());
    }
}