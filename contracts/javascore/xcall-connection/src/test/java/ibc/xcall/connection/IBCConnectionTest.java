package ibc.xcall.connection;

import static org.mockito.AdditionalMatchers.aryEq;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.ServiceManager;

import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;

import java.math.BigInteger;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;
import org.mockito.Mockito;

public class IBCConnectionTest extends IBCConnectionTestBase {

    protected final ServiceManager sm = getServiceManager();

    @BeforeEach
    public void setup() throws Exception {
      super.setup();
    }

    ArgumentCaptor<byte[]> packetCaptor = ArgumentCaptor.forClass(byte[].class);

    @Test
    public void sendMessage_noResponse() {
        // Arrange
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());
        establishDefaultConnection();

        // Act
        doReturn(packetFee).when(connectionSpy).getValue();
        when(ibc.mock.getNextSequenceSend(port, defaultChannel)).thenReturn(seq);

        connection.invoke(xcall.account, "sendMessage", defaultCounterpartyNid, "", BigInteger.ZERO, data);

        // Assert
        latestHeight.setRevisionHeight(latestHeight.getRevisionHeight().add(defaultTimeoutHeight));
        verify(ibc.mock).sendPacket(packetCaptor.capture());
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(IBCConnection.PORT, packet.getSourcePort());
        assertEquals(defaultCounterpartyPort, packet.getDestinationPort());
        assertEquals(defaultChannel, packet.getSourceChannel());
        assertEquals(defaultCounterpartyChannel, packet.getDestinationChannel());
        assertArrayEquals(latestHeight.encode(), packet.getTimeoutHeight().encode());

        Message msg = Message.fromBytes(packet.getData());
        assertEquals(BigInteger.ZERO, msg.getSn());
        assertArrayEquals(data, msg.getData());
        assertEquals(packetFee, msg.getFee());
    }

    @Test
    public void sendMessage_withResponse() {
        // Arrange
        byte[] data = "test".getBytes();
        byte[] ack = "ack".getBytes();
        BigInteger sn = BigInteger.ONE;
        BigInteger seq = BigInteger.TEN;
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());
        establishDefaultConnection_fromCounterparty();
        doReturn(packetFee.add(ackFee)).when(connectionSpy).getValue();
        when(ibc.mock.getNextSequenceSend(port, defaultChannel)).thenReturn(seq);
        connection.invoke(xcall.account, "sendMessage", defaultCounterpartyNid, "", sn, data);
        verify(ibc.mock).sendPacket(packetCaptor.capture());

        // Act
        connection.getAccount().addBalance("ICX", ackFee);
        connection.invoke(ibc.account, "onAcknowledgementPacket", packetCaptor.getValue(), ack, relayer.getAddress());
        // Assert
        verify(xcall.mock).handleMessage(defaultCounterpartyNid, ack);

        latestHeight.setRevisionHeight(latestHeight.getRevisionHeight().add(defaultTimeoutHeight));
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(IBCConnection.PORT, packet.getSourcePort());
        assertEquals(defaultCounterpartyPort, packet.getDestinationPort());
        assertEquals(defaultChannel, packet.getSourceChannel());
        assertEquals(defaultCounterpartyChannel, packet.getDestinationChannel());
        assertArrayEquals(latestHeight.encode(), packet.getTimeoutHeight().encode());

        Message msg = Message.fromBytes(packet.getData());
        assertEquals(sn, msg.getSn());
        assertArrayEquals(data, msg.getData());
        assertEquals(packetFee, msg.getFee());

        assertEquals(ackFee, relayer.getBalance());
    }

    @Test
    public void sendMessage_multipleConnection() {
        // Arrange
        String counterpartyNid2 = "0x1.BSC";
        byte[] data = "test".getBytes();
        BigInteger seq1 = BigInteger.TEN;
        BigInteger seq2 = BigInteger.TWO;
        String client2 = "client-2";
        String connection2 = "connection-2";
        String channel2 = "channel-2";
        String counterpartyPort2 = "cPort-2";
        String counterpartyChannel2 = "cchannel-2";
        BigInteger sn1 = BigInteger.ZERO;
        BigInteger sn2 = BigInteger.TWO;

        BigInteger packetFee2 = BigInteger.valueOf(7);
        BigInteger ackFee2 = BigInteger.valueOf(11);

        connection.invoke(owner, "setFee", counterpartyNid2, packetFee2, ackFee2);

        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());
        when(ibc.mock.getLatestHeight(client2)).thenReturn(latestHeight.encode());
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, defaultChannel)).thenReturn(seq1);
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, channel2)).thenReturn(seq2);

        establishDefaultConnection();
        establishConnection_fromCounterparty(client2, connection2, channel2, counterpartyPort2, counterpartyChannel2, counterpartyNid2, defaultTimeoutHeight);

        // Act
        doReturn(packetFee).when(connectionSpy).getValue();
        connection.invoke(xcall.account, "sendMessage", defaultCounterpartyNid, "", sn1, data);
        doReturn(packetFee2).when(connectionSpy).getValue();
        connection.invoke(xcall.account, "sendMessage", counterpartyNid2, "", sn2, data);

        // Assert
        verify(ibc.mock, Mockito.times(2)).sendPacket(packetCaptor.capture());
        Packet packet1 = Packet.decode(packetCaptor.getAllValues().get(0));
        assertEquals(seq1, packet1.getSequence());
        assertEquals(defaultChannel, packet1.getSourceChannel());
        assertEquals(defaultCounterpartyChannel, packet1.getDestinationChannel());

        Message msg1 = Message.fromBytes(packet1.getData());
        assertEquals(sn1, msg1.getSn());
        assertEquals(packetFee, msg1.getFee());
        assertArrayEquals(data, msg1.getData());

        Packet packet2 = Packet.decode(packetCaptor.getAllValues().get(1));
        assertEquals(seq2, packet2.getSequence());
        assertEquals(channel2, packet2.getSourceChannel());
        assertEquals(counterpartyChannel2, packet2.getDestinationChannel());

        Message msg2 = Message.fromBytes(packet2.getData());
        assertEquals(sn2, msg2.getSn());
        assertEquals(packetFee2, msg2.getFee());
        assertArrayEquals(data, msg2.getData());

        // Act
        packet1.setSequence(packet2.getSequence());
        assertThrows(AssertionError.class, () -> connection.invoke(ibc.account, "onAcknowledgementPacket", packet1.encode(), "ack".getBytes(), relayer.getAddress()));
        connection.getAccount().addBalance("ICX", ackFee2);
        connection.invoke(ibc.account, "onAcknowledgementPacket", packet2.encode(), "ack".getBytes(), relayer.getAddress());

        // Assert
        verify(xcall.mock).handleMessage(counterpartyNid2, "ack".getBytes());
        assertEquals(ackFee2, relayer.getBalance());
    }

    @Test
    public void sendMessage_withTimeout() {
        // Arrange
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.TWO;
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());

        establishDefaultConnection();
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, defaultChannel)).thenReturn(seq);
        connection.invoke(xcall.account, "sendMessage", defaultCounterpartyNid, "", sn, data);
        verify(ibc.mock).sendPacket(packetCaptor.capture());

        // Act
        connection.getAccount().addBalance("ICX", ackFee.add(packetFee));
        connection.invoke(ibc.account, "onTimeoutPacket", packetCaptor.getValue(), relayer.getAddress());

        // Assert
        verify(xcall.mock).handleError( sn );
        assertEquals(ackFee.add(packetFee), relayer.getBalance());
    }

    @Test
    public void recvMessage_noResponse() {
        // Arrange
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.ZERO;

        establishDefaultConnection();

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(sn,BigInteger.ZERO, data).toBytes());
        pct.setSourcePort(defaultCounterpartyPort);
        pct.setSourceChannel(defaultCounterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(defaultChannel);

        // Act
        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());


        // Assert
        verify(xcall.mock).handleMessage(defaultCounterpartyNid, data);
    }

    @Test
    public void recvMessage_withResponse() {
        // Arrange
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.ONE;

        establishDefaultConnection();

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(sn, BigInteger.ZERO, data).toBytes());
        pct.setSourcePort(defaultCounterpartyPort);
        pct.setSourceChannel(defaultCounterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(defaultChannel);

        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());

        // Act
        connection.invoke(xcall.account, "sendMessage", defaultCounterpartyNid, "", sn.negate(), data);

        // Assert
        verify(ibc.mock).writeAcknowledgement(packetCaptor.capture(), aryEq(data));
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(defaultChannel, packet.getDestinationChannel());
        assertEquals(IBCConnection.PORT, packet.getDestinationPort());
    }

    @Test
    public void sendClaimFees() {
        // Arrange
        byte[] data = "test".getBytes();
        BigInteger seq = BigInteger.TEN;
        BigInteger sn = BigInteger.ONE;
        BigInteger fee = BigInteger.valueOf(50);
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, defaultChannel)).thenReturn(seq);

        establishDefaultConnection();

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(sn, fee, data).toBytes());
        pct.setSourcePort(defaultCounterpartyPort);
        pct.setSourceChannel(defaultCounterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(defaultChannel);

        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());
        assertEquals(fee, connection.call("getUnclaimedFees", defaultCounterpartyNid, relayer.getAddress()));

        // Act
        connection.invoke(relayer, "claimFees", defaultCounterpartyNid, relayer.getAddress().toByteArray());

        // Assert
        when(ibc.mock.getNextSequenceSend(IBCConnection.PORT, defaultChannel)).thenReturn(seq);
        verify(ibc.mock).sendPacket(packetCaptor.capture());
        Packet packet = Packet.decode(packetCaptor.getValue());
        assertEquals(seq, packet.getSequence());
        assertEquals(defaultChannel, packet.getSourceChannel());
        assertEquals(port, packet.getSourcePort());
        assertEquals(defaultCounterpartyChannel, packet.getDestinationChannel());
        assertEquals(defaultCounterpartyPort, packet.getDestinationPort());

        Message msg = Message.fromBytes(packet.getData());
        assertArrayEquals(relayer.getAddress().toByteArray(), msg.getData());
        assertEquals(fee, msg.getFee());
        assertNull(msg.getSn());
        assertEquals(BigInteger.ZERO, connection.call("getUnclaimedFees", defaultCounterpartyNid, relayer.getAddress()));
    }

    @Test
    public void recvClaimFees() {
        // Arrange
        BigInteger seq = BigInteger.TEN;
        BigInteger fee = BigInteger.valueOf(30);
        Height latestHeight = new Height();
        latestHeight.setRevisionHeight(BigInteger.valueOf(100));
        when(ibc.mock.getLatestHeight(defaultClientId)).thenReturn(latestHeight.encode());
        when(ibc.mock.getNextSequenceSend(port, defaultChannel)).thenReturn(seq);
        establishDefaultConnection();

        Packet pct = new Packet();
        pct.setSequence(seq);
        pct.setData(new Message(null, fee, relayer.getAddress().toByteArray()).toBytes());
        pct.setSourcePort(defaultCounterpartyPort);
        pct.setSourceChannel(defaultCounterpartyChannel);
        pct.setDestinationPort(IBCConnection.PORT);
        pct.setDestinationChannel(defaultChannel);

        // Act
        connection.getAccount().addBalance("ICX", fee);
        connection.invoke(ibc.account, "onRecvPacket", pct.encode(), relayer.getAddress());

        // Assert
        assertEquals(fee, relayer.getBalance());
    }

    @Test
    public void getFee() {
        String nid = "nid2";
        BigInteger packetFee2 = BigInteger.valueOf(11);
        BigInteger ackFee2 = BigInteger.valueOf(1);
        connection.invoke(owner, "setFee", nid, packetFee2, ackFee2);

        assertEquals(packetFee, connection.call("getFee", defaultCounterpartyNid, false));
        assertEquals(packetFee.add(ackFee), connection.call("getFee", defaultCounterpartyNid, true));

        assertEquals(packetFee2, connection.call("getFee", nid, false));
        assertEquals(packetFee2.add(ackFee2), connection.call("getFee", nid, true));
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
            () -> connection.invoke(nonAuthorized, "configureConnection", "", "", "", "", BigInteger.ZERO));
        assertEquals(expectedErrorMessage, e.getMessage());
    }
}