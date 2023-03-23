package ibc.ics04.channel;

import ibc.icon.interfaces.IIBCChannelHandshake;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.*;
import icon.proto.core.channel.Channel;
import icon.proto.core.connection.ConnectionEnd;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import score.Context;

import java.math.BigInteger;
import java.util.List;

public class IBCChannelHandshake extends IBCConnection {

    public String channelOpenInit(MsgChannelOpenInit msg) {
        Channel channel = msg.getChannel();
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
        channels.at(msg.getPortId()).set(channelId, msg.getChannelRaw());
        nextSequenceSends.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.getPortId()).set(channelId, BigInteger.ONE);

        updateChannelCommitment(msg.getPortId(), channelId, msg.getChannelRaw());

        return channelId;
    }

    public String channelOpenTry(MsgChannelOpenTry msg) {
        Channel channel = msg.getChannel();
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
                msg.getProofHeightRaw(),
                msg.getProofInit(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        String channelId = generateChannelIdentifier();
        channels.at(msg.getPortId()).set(channelId, msg.getChannelRaw());
        nextSequenceSends.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.getPortId()).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.getPortId()).set(channelId, BigInteger.ONE);

        updateChannelCommitment(msg.getPortId(), channelId, msg.getChannelRaw());

        return channelId;
    }

    public void channelOpenAck(MsgChannelOpenAck msg) {
        Channel channel = Channel.decode(channels.at(msg.getPortId()).get(msg.getChannelId()));
        Context.require(channel != null, "channel does not exist");
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
                msg.getProofHeightRaw(),
                msg.getProofTry(),
                channel.getCounterparty().getPortId(),
                msg.getCounterpartyChannelId(),
                expectedChannel);
        channel.setState(Channel.State.STATE_OPEN);
        channel.setVersion(msg.getCounterpartyVersion());
        channel.getCounterparty().setChannelId(msg.getCounterpartyChannelId());

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);
    }

    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
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
                msg.getProofHeightRaw(),
                msg.getProofAck(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.setState(Channel.State.STATE_OPEN);

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);
    }

    public void channelCloseInit(MsgChannelCloseInit msg) {
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
        updateChannelCommitment(msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);
    }

    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
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
                msg.getProofHeightRaw(),
                msg.getProofInit(),
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.setState(Channel.State.STATE_CLOSED);

        byte[] encodedChannel = channel.encode();
        updateChannelCommitment(msg.getPortId(), msg.getChannelId(), encodedChannel);
        channels.at(msg.getPortId()).set(msg.getChannelId(), encodedChannel);
    }

    private void updateChannelCommitment(String portId, String channelId, byte[] channel) {
        sendBTPMessage(ByteUtil.join(IBCCommitment.channelCommitmentKey(portId, channelId),
                IBCCommitment.keccak256(channel)));
        // commitments.set(IBCCommitment.channelCommitmentKey(portId, channelId),
        // IBCCommitment.keccak256(channel.toBytes()));
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
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                BigInteger.ZERO,
                BigInteger.ZERO,
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                IBCCommitment.channelPath(portId, channelId),
                channel.encode());
        Context.require(ok, "failed to verify channel state");

    }

    /* Internal functions */

    private String generateChannelIdentifier() {
        BigInteger currChannelSequence = nextChannelSequence.getOrDefault(BigInteger.ZERO);
        String identifier = "channel-" + currChannelSequence.toString();
        nextChannelSequence.set(currChannelSequence.add(BigInteger.ONE));

        return identifier;
    }
}
