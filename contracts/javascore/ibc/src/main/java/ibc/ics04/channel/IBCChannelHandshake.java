package ibc.ics04.channel;

import java.math.BigInteger;
import java.util.List;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;
import icon.proto.core.channel.Channel;
import icon.proto.core.connection.ConnectionEnd;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import score.Context;

public class IBCChannelHandshake extends IBCConnection {

    public String _channelOpenInit(MsgChannelOpenInit msg) {
        Channel channel = Channel.decode(msg.getChannel());
        Context.require(channel.getConnectionHops().size() == 1, "connection_hops length must be 1");

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        Context.require(
                connection.getVersions().size() == 1,
                "single version must be negotiated on connection before opening channel");

        Context.require(channel.getState() == Channel.State.STATE_INIT,
                "channel state must be STATE_INIT");

        // TODO: verifySupportedFeature
        // TODO: authenticates a port binding

        String channelId = generateChannelIdentifier();
        channels.at(msg.getPortId()).set(channelId, msg.getChannel());
        nextSequenceSends.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.getPortId()).set(channelId, BigInteger.ONE);

        updateChannelCommitment(connection.getClientId(), msg.getPortId(), channelId, msg.getChannel());

        return channelId;
    }

    public String _channelOpenTry(MsgChannelOpenTry msg) {
        Channel channel = Channel.decode(msg.getChannel());
        Context.require(channel.getConnectionHops().size() == 1, "connection_hops length must be 1");
        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        Context.require(
                connection.getVersions().size() == 1,
                "single version must be negotiated on connection before opening channel");
        Context.require(channel.getState() == Channel.State.STATE_TRYOPEN,
                "channel state must be STATE_TRYOPEN");

        // TODO verifySupportedFeature

        // TODO authenticates a port binding

        Channel.Counterparty expectedCounterparty = new Channel.Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_INIT);
        expectedChannel.setOrdering(channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(connection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(msg.getCounterpartyVersion());

        verifyChannelState(
                connection,
                msg.getProofHeight(),
                msg.getProofInit(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        String channelId = generateChannelIdentifier();
        channels.at(msg.getPortId()).set(channelId, msg.getChannel());
        nextSequenceSends.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.getPortId()).set(channelId, BigInteger.ONE);

        updateChannelCommitment(connection.getClientId(), msg.getPortId(), channelId, msg.getChannel());

        return channelId;
    }

    public byte[] _channelOpenAck(MsgChannelOpenAck msg) {
        byte[] channelPb = channels.at(msg.getPortId()).get(msg.getChannelId());
        Context.require(channelPb != null, "channel does not exist");
        Channel channel = Channel.decode(channelPb);
        Context.require(channel.getConnectionHops().size() == 1);

        Context.require(
                channel.getState() == Channel.State.STATE_INIT
                        || channel.getState() == Channel.State.STATE_TRYOPEN,
                "invalid channel state");

        // TODO authenticates a port binding

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        Channel.Counterparty expectedCounterparty = new Channel.Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_TRYOPEN);
        expectedChannel.setOrdering(channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(connection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(msg.getCounterpartyVersion());

        verifyChannelState(
                connection,
                msg.getProofHeight(),
                msg.getProofTry(),
                channel.getCounterparty().getPortId(),
                msg.getCounterpartyChannelId(),
                expectedChannel);
        channel.setState(Channel.State.STATE_OPEN);
        channel.setVersion(msg.getCounterpartyVersion());
        channel.getCounterparty().setChannelId(msg.getCounterpartyChannelId());

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(connection.getClientId(), msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);

        return encodedChannel;
    }

    public byte[] _channelOpenConfirm(MsgChannelOpenConfirm msg) {
        Channel channel = Channel.decode(channels.at(msg.getPortId()).get(msg.getChannelId()));
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.getConnectionHops().size() == 1);
        Context.require(channel.getState() == Channel.State.STATE_TRYOPEN, "channel state is not TRYOPEN");

        // TODO authenticates a port binding

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        Channel.Counterparty expectedCounterparty = new Channel.Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_OPEN);
        expectedChannel.setOrdering(channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(connection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(channel.getVersion());
        verifyChannelState(
                connection,
                msg.getProofHeight(),
                msg.getProofAck(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.setState(Channel.State.STATE_OPEN);

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(connection.getClientId(), msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);

        return encodedChannel;
    }

    public byte[] _channelCloseInit(MsgChannelCloseInit msg) {
        Channel channel = Channel.decode(channels.at(msg.getPortId()).get(msg.getChannelId()));
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.getState() != Channel.State.STATE_CLOSED, "channel state is already CLOSED");

        // TODO authenticates a port binding

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        channel.setState(Channel.State.STATE_CLOSED);

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(connection.getClientId(), msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);

        return encodedChannel;
    }

    public byte[] _channelCloseConfirm(MsgChannelCloseConfirm msg) {
        Channel channel = Channel.decode(channels.at(msg.getPortId()).get(msg.getChannelId()));
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.getState() != Channel.State.STATE_CLOSED, "channel state is already CLOSED");
        Context.require(channel.getConnectionHops().size() == 1);

        // TODO authenticates a port binding

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        Channel.Counterparty expectedCounterparty = new Channel.Counterparty();
        expectedCounterparty.setPortId(msg.getPortId());
        expectedCounterparty.setChannelId(msg.getChannelId());

        Channel expectedChannel = new Channel();
        expectedChannel.setState(Channel.State.STATE_CLOSED);
        expectedChannel.setOrdering(channel.getOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(List.of(connection.getCounterparty().getConnectionId()));
        expectedChannel.setVersion(channel.getVersion());

        verifyChannelState(
                connection,
                msg.getProofHeight(),
                msg.getProofInit(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.setState(Channel.State.STATE_CLOSED);

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(connection.getClientId(), msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);

        return encodedChannel;
    }

    protected void updateChannelCommitment(String clientId, String portId, String channelId, byte[] channel) {
        sendBTPMessage(clientId, ByteUtil.join(IBCCommitment.channelCommitmentKey(portId, channelId),
                IBCCommitment.keccak256(channel)));
    }

    /* Verification functions */

    private void verifyChannelState(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            String portId,
            String channelId,
            Channel channel) {
        ILightClient client = getClient(connection.getClientId());
        client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                IBCCommitment.channelPath(portId, channelId),
                channel.encode());

    }

    /* Internal functions */

    private String generateChannelIdentifier() {
        BigInteger currChannelSequence = nextChannelSequence.getOrDefault(BigInteger.ZERO);
        String identifier = "channel-" + currChannelSequence.toString();
        nextChannelSequence.set(currChannelSequence.add(BigInteger.ONE));

        return identifier;
    }
}
