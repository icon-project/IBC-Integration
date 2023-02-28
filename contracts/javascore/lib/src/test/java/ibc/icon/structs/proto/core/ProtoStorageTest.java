package ibc.icon.structs.proto.core;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Counterparty;
import ibc.icon.structs.proto.core.connection.Version;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import score.VarDB;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;
import static score.Context.newVarDB;

public class ProtoStorageTest extends TestBase {

    private static final ServiceManager sm = getServiceManager();
    private static final Account owner = sm.createAccount();
    private static Score dummyScore;

    public static class DummyScore {

        VarDB<ConnectionEnd> connectionEndDB = newVarDB("connectionEndDB",
                ConnectionEnd.class);
        VarDB<Channel> channelDB = newVarDB("channelDB", Channel.class);
        VarDB<Packet> packetDB = newVarDB("packetDB", Packet.class);

        public DummyScore() {
        }

        public void setConnectionEnd(ConnectionEnd connectionEnd) {
            connectionEndDB.set(connectionEnd);
        }

        public ConnectionEnd getConnectionEnd() {
            return connectionEndDB.get();
        }

        public void setChannel(Channel channel) {
            channelDB.set(channel);
        }

        public Channel getChannel() {
            return channelDB.get();
        }

        public void setPacket(Packet packet) {
            packetDB.set(packet);
        }

        public Packet getPacket() {
            return packetDB.get();
        }
    }

    @BeforeEach
    public void setup() throws Exception {
        dummyScore = sm.deploy(owner, DummyScore.class);
    }

    @Test
    public void storeConnectionEnd() {
        // Arrange
        MerklePrefix prefix = new MerklePrefix();
        prefix.setKeyPrefix("merkle");

        Counterparty counterparty = new Counterparty();
        counterparty.setClientId("clientId");
        counterparty.setConnectionId("connectionId");
        counterparty.setPrefix(prefix);

        Version version1 = new Version();
        Version version2 = new Version();
        version1.setIdentifier("version1");
        version2.setIdentifier("version2");
        version1.setFeatures(new String[] { "v1Feature1", "v1Feature2" });
        version2.setFeatures(new String[] { "v2Feature1", "v2Feature2", "v2Feature3" });

        ConnectionEnd connectionEnd = new ConnectionEnd();
        connectionEnd.setClientId("clientId");
        connectionEnd.setVersions(new Version[] { version1, version2 });
        connectionEnd.setState(ConnectionEnd.State.STATE_INIT);
        connectionEnd.setCounterparty(counterparty);
        connectionEnd.setDelayPeriod(BigInteger.ONE);

        // Act
        dummyScore.invoke(owner, "setConnectionEnd", connectionEnd);

        // Assert
        ConnectionEnd storedConnectionEnd = (ConnectionEnd) dummyScore.call("getConnectionEnd");

        assertEquals(connectionEnd.getCounterparty().getClientId(),
                storedConnectionEnd.getCounterparty().getClientId());
        assertEquals(connectionEnd.getCounterparty().getConnectionId(),
                storedConnectionEnd.getCounterparty().getConnectionId());
        assertEquals(connectionEnd.getCounterparty().getPrefix().getKeyPrefix(),
                storedConnectionEnd.getCounterparty().getPrefix().getKeyPrefix());

        assertEquals(connectionEnd.getVersions()[0].getIdentifier(),
                storedConnectionEnd.getVersions()[0].getIdentifier());
        assertArrayEquals(connectionEnd.getVersions()[0].getFeatures(),
                storedConnectionEnd.getVersions()[0].getFeatures());
        assertEquals(connectionEnd.getVersions()[1].getIdentifier(),
                storedConnectionEnd.getVersions()[1].getIdentifier());
        assertArrayEquals(connectionEnd.getVersions()[1].getFeatures(),
                storedConnectionEnd.getVersions()[1].getFeatures());

        assertEquals(connectionEnd.getClientId(), storedConnectionEnd.getClientId());
        assertEquals(connectionEnd.connectionState(),
                storedConnectionEnd.connectionState());
        assertEquals(connectionEnd.getDelayPeriod(),
                storedConnectionEnd.getDelayPeriod());
    }

    @Test
    public void storeChannel() {
        // Arrange
        ibc.icon.structs.proto.core.channel.Counterparty counterparty = new ibc.icon.structs.proto.core.channel.Counterparty();
        counterparty.setPortId("portId");
        counterparty.setChannelId("channelId");

        Channel channel = new Channel();
        channel.updateState(Channel.State.STATE_TRYOPEN);
        channel.updateOrder(Channel.Order.ORDER_ORDERED);
        channel.setCounterparty(counterparty);
        channel.setConnectionHops(new String[] { "Aerw", "were" });
        channel.setVersion("version");

        // Act
        dummyScore.invoke(owner, "setChannel", channel);

        // Assert
        Channel storedChannel = (Channel) dummyScore.call("getChannel");

        assertEquals(channel.channelState(), storedChannel.channelState());
        assertEquals(channel.channelOrdering(), storedChannel.channelOrdering());
        assertEquals(channel.getCounterparty().getChannelId(),
                storedChannel.getCounterparty().getChannelId());
        assertEquals(channel.getCounterparty().getPortId(),
                storedChannel.getCounterparty().getPortId());
        assertArrayEquals(channel.getConnectionHops(),
                storedChannel.getConnectionHops());
        assertEquals(channel.getVersion(), storedChannel.getVersion());
    }

    @Test
    public void storePacket() {
        // Arrange
        Packet packet = new Packet();
        packet.setSequence(BigInteger.ONE);
        packet.setSourcePort("sourcePort");
        packet.setSourceChannel("sourceChannel");
        packet.setDestinationPort("destinationPort");
        packet.setDestinationChannel("destinationChannel");
        packet.setData("data");

        Height timeoutHeight = new Height();
        timeoutHeight.setRevisionNumber(BigInteger.valueOf(2));
        timeoutHeight.setRevisionHeight(BigInteger.valueOf(3));

        packet.setTimeoutHeight(timeoutHeight);
        packet.setTimeoutTimestamp(BigInteger.valueOf(3));
        // Act
        dummyScore.invoke(owner, "setPacket", packet);

        // Assert
        Packet storedPacket = (Packet) dummyScore.call("getPacket");

        assertEquals(packet.getSequence(), storedPacket.getSequence());
        assertEquals(packet.getSourcePort(), storedPacket.getSourcePort());
        assertEquals(packet.getSourceChannel(), storedPacket.getSourceChannel());
        assertEquals(packet.getDestinationPort(), storedPacket.getDestinationPort());
        assertEquals(packet.getDestinationChannel(),
                storedPacket.getDestinationChannel());
        assertEquals(packet.getData(), storedPacket.getData());
        assertEquals(packet.getTimeoutHeight().getRevisionNumber(),
                storedPacket.getTimeoutHeight().getRevisionNumber());
        assertEquals(packet.getTimeoutHeight().getRevisionHeight(),
                storedPacket.getTimeoutHeight().getRevisionHeight());
        assertEquals(packet.getTimeoutTimestamp(),
                storedPacket.getTimeoutTimestamp());
    }
}
