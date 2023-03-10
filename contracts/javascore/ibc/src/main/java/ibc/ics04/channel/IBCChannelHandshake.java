package ibc.ics04.channel;

import ibc.icon.interfaces.IIBCChannelHandshake;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.structs.messages.*;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Counterparty;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.ics03.connection.IBCConnection;
import ibc.ics24.host.IBCCommitment;
import score.Context;

import java.math.BigInteger;

public class IBCChannelHandshake extends IBCConnection implements IIBCChannelHandshake {

    public String channelOpenInit(MsgChannelOpenInit msg) {
        Context.require(msg.channel.getConnectionHops().length == 1, "connection_hops length must be 1");

        ConnectionEnd connection = connections.get(msg.channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(
                connection.getVersions().length == 1,
                "single version must be negotiated on connection before opening channel");

        Context.require(msg.channel.channelState().equals(Channel.State.STATE_INIT),
                "channel state must be STATE_INIT");

        // TODO: verifySupportedFeature
        // TODO: authenticates a port binding

        String channelId = generateChannelIdentifier();
        channels.at(msg.portId).set(channelId, msg.channel);
        nextSequenceSends.at(msg.portId).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.portId).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.portId).set(channelId, BigInteger.ONE);

        updateChannelCommitment(msg.portId, channelId, msg.channel);

        return channelId;
    }

    public String channelOpenTry(MsgChannelOpenTry msg) {
        Context.require(msg.channel.getConnectionHops().length == 1, "connection_hops length must be 1");
        ConnectionEnd connection = connections.get(msg.channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(
                connection.getVersions().length == 1,
                "single version must be negotiated on connection before opening channel");
        Context.require(msg.channel.channelState().equals(Channel.State.STATE_TRYOPEN),
                "channel state must be STATE_TRYOPEN");

        // TODO verifySupportedFeature

        // TODO authenticates a port binding

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId("");

        Channel expectedChannel = new Channel();
        expectedChannel.updateState(Channel.State.STATE_INIT);
        expectedChannel.updateOrder(msg.channel.channelOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(getCounterpartyHops(msg.channel.getConnectionHops()[0]));
        expectedChannel.setVersion(msg.counterpartyVersion);

        verifyChannelState(
                connection,
                msg.proofHeight,
                msg.proofInit,
                msg.channel.getCounterparty().getPortId(),
                msg.channel.getCounterparty().getChannelId(),
                expectedChannel);

        String channelId = generateChannelIdentifier();
        channels.at(msg.portId).set(channelId, msg.channel);
        nextSequenceSends.at(msg.portId).set(channelId, BigInteger.ONE);
        nextSequenceReceives.at(msg.portId).set(channelId, BigInteger.ONE);
        nextSequenceAcknowledgements.at(msg.portId).set(channelId, BigInteger.ONE);

        updateChannelCommitment(msg.portId, channelId, msg.channel);

        return channelId;
    }

    public void channelOpenAck(MsgChannelOpenAck msg) {
        Channel channel = channels.at(msg.portId).get(msg.channelId);
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.getConnectionHops().length == 1);

        Context.require(
                channel.channelState().equals(Channel.State.STATE_INIT)
                        || channel.channelState().equals(Channel.State.STATE_TRYOPEN),
                "invalid channel state");

        // TODO authenticates a port binding

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.connectionState().equals(ConnectionEnd.State.STATE_OPEN),
                "connection state is not OPEN");

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.updateState(Channel.State.STATE_TRYOPEN);
        expectedChannel.updateOrder(channel.channelOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(getCounterpartyHops(channel.getConnectionHops()[0]));
        expectedChannel.setVersion(msg.counterpartyVersion);

        verifyChannelState(
                connection,
                msg.proofHeight,
                msg.proofTry,
                channel.getCounterparty().getPortId(),
                msg.counterpartyChannelId,
                expectedChannel);
        channel.updateState(Channel.State.STATE_OPEN);
        channel.setVersion(msg.counterpartyVersion);
        channel.getCounterparty().setChannelId(msg.counterpartyChannelId);

        updateChannelCommitment(msg.portId, msg.channelId, channel);
        channels.at(msg.portId).set(msg.channelId, channel);
    }

    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        Channel channel = channels.at(msg.portId).get(msg.channelId);
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.getConnectionHops().length == 1);
        Context.require(channel.channelState().equals(Channel.State.STATE_TRYOPEN), "channel state is not TRYOPEN");

        // TODO authenticates a port binding

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.connectionState().equals(ConnectionEnd.State.STATE_OPEN),
                "connection state is not OPEN");

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.updateState(Channel.State.STATE_OPEN);
        expectedChannel.updateOrder(channel.channelOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(getCounterpartyHops(channel.getConnectionHops()[0]));
        expectedChannel.setVersion(channel.getVersion());

        verifyChannelState(
                connection,
                msg.proofHeight,
                msg.proofAck,
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.updateState(Channel.State.STATE_OPEN);

        updateChannelCommitment(msg.portId, msg.channelId, channel);
        channels.at(msg.portId).set(msg.channelId, channel);
    }

    public void channelCloseInit(MsgChannelCloseInit msg) {
        Channel channel = channels.at(msg.portId).get(msg.channelId);
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.channelState() != Channel.State.STATE_CLOSED, "channel state is already CLOSED");

        // TODO authenticates a port binding

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.connectionState().equals(ConnectionEnd.State.STATE_OPEN),
                "connection state is not OPEN");

        channel.updateState(Channel.State.STATE_CLOSED);

        updateChannelCommitment(msg.portId, msg.channelId, channel);
        channels.at(msg.portId).set(msg.channelId, channel);
    }

    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        Channel channel = channels.at(msg.portId).get(msg.channelId);
        Context.require(channel != null, "channel does not exist");
        Context.require(channel.channelState() != Channel.State.STATE_CLOSED, "channel state is already CLOSED");
        Context.require(channel.getConnectionHops().length == 1);

        // TODO authenticates a port binding

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.connectionState().equals(ConnectionEnd.State.STATE_OPEN),
                "connection state is not OPEN");

        Counterparty expectedCounterparty = new Counterparty();
        expectedCounterparty.setPortId(msg.portId);
        expectedCounterparty.setChannelId(msg.channelId);

        Channel expectedChannel = new Channel();
        expectedChannel.updateState(Channel.State.STATE_CLOSED);
        expectedChannel.updateOrder(channel.channelOrdering());
        expectedChannel.setCounterparty(expectedCounterparty);
        expectedChannel.setConnectionHops(getCounterpartyHops(channel.getConnectionHops()[0]));
        expectedChannel.setVersion(channel.getVersion());

        verifyChannelState(
                connection,
                msg.proofHeight,
                msg.proofInit,
                channel.getCounterparty().getPortId(),
                channel.getCounterparty().getChannelId(),
                expectedChannel);

        channel.updateState(Channel.State.STATE_CLOSED);

        updateChannelCommitment(msg.portId, msg.channelId, channel);
        channels.at(msg.portId).set(msg.channelId, channel);
    }

    private void updateChannelCommitment(String portId, String channelId, Channel channel) {
        commitments.set(IBCCommitment.channelCommitmentKey(portId, channelId),
                IBCCommitment.keccak256(channel.toBytes()));
    }

    /* Verification functions */

    private void verifyChannelState(
            ConnectionEnd connection,
            Height height,
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
                channel.toBytes());
        Context.require(ok, "failed to verify channel state");

    }

    /* Internal functions */

    private String[] getCounterpartyHops(String connectionId) {
        String hop = connections.get(connectionId).getCounterparty().getConnectionId();
        String[] hops = new String[]{hop};
        return hops;
    }

    private String generateChannelIdentifier() {
        BigInteger currChannelSequence = nextChannelSequence.getOrDefault(BigInteger.ZERO);
        String identifier = "channel-" + currChannelSequence.toString();
        nextChannelSequence.set(currChannelSequence.add(BigInteger.ONE));

        return identifier;
    }
}
