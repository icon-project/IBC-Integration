package ibc.ics04.channel;

import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.ics24.host.IBCCommitment;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.ConnectionEnd;
import score.Context;
import score.DictDB;

import java.math.BigInteger;

// TODO verify packet commitments follow a correct format
public class IBCPacket extends IBCChannelHandshake implements IIBCPacket {
    public void sendPacket(Packet packet) {
        Channel channel = Channel.decode(channels.at(packet.getSourcePort()).get(packet.getSourceChannel()));
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");
        Context.require(
                packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = ConnectionEnd.decode(connections.get(channel.getConnectionHops().get(0)));
        Context.require(connection != null, "connection does not exist");
        ILightClient client = getClient(connection.getClientId());
        byte[] latestHeightRaw = client.getLatestHeight(connection.getClientId());
        Height latestHeight = Height.decode(latestHeightRaw);

        Context.require(
                isZero(packet.getTimeoutHeight()) || lt(latestHeight, packet.getTimeoutHeight()),
                "receiving chain block height >= packet timeout height");
        BigInteger latestTimestamp = client.getTimestampAtHeight(connection.getClientId(), latestHeightRaw);
        Context.require(latestTimestamp != null, "consensusState not found");
        Context.require(
                packet.getTimeoutTimestamp().equals(BigInteger.ZERO)
                        || latestTimestamp.compareTo(packet.getTimeoutTimestamp()) < 0,
                "receiving chain block timestamp >= packet timeout timestamp");

        DictDB<String, BigInteger> nextSequenceSourcePort = nextSequenceSends.at(packet.getSourcePort());
        BigInteger nextSequenceSend = nextSequenceSourcePort.getOrDefault(packet.getSourceChannel(), BigInteger.ZERO);
        Context.require(
                packet.getSequence().equals(nextSequenceSend),
                "packet sequence != next send sequence");

        nextSequenceSourcePort.set(packet.getSourceChannel(), nextSequenceSend.add(BigInteger.ONE));

        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(packet.getSourcePort(),
                packet.getSourceChannel(),
                packet.getSequence());

        byte[] packetCommitment = IBCCommitment.keccak256(createPacketCommitment(packet));
        commitments.set(packetCommitmentKey, packetCommitment);

        sendBTPMessage(ByteUtil.join(packetCommitmentKey, packetCommitment));

    }

    public void recvPacket(Packet packet, byte[] proof, byte[] proofHeight) {
        Channel channel = Channel.decode(channels.at(packet.getDestinationPort()).get(packet.getDestinationChannel()));
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        // TODO
        // Authenticate capability to ensure caller has authority to receive packet on
        // this channel

        Context.require(
                packet.getSourcePort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getSourceChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = ConnectionEnd.decode(connections.get(channel.getConnectionHops().get(0)));
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        Context.require(
                packet.getTimeoutHeight().getRevisionHeight().equals(BigInteger.ZERO)
                        || BigInteger.valueOf(Context.getBlockHeight())
                        .compareTo(packet.getTimeoutHeight().getRevisionHeight()) < 0,
                "block height >= packet timeout height");
        Context.require(
                packet.getTimeoutTimestamp().equals(BigInteger.ZERO)
                        || BigInteger.valueOf(Context.getBlockTimestamp())
                        .compareTo(packet.getTimeoutTimestamp()) < 0,
                "block timestamp >= packet timeout timestamp");

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(packet.getSourcePort(),
                packet.getSourceChannel(), packet.getSequence());
        byte[] commitmentBytes = IBCCommitment.keccak256(createPacketCommitment(packet));

        verifyPacketCommitment(
                connection,
                proofHeight,
                proof,
                commitmentPath,
                commitmentBytes);

        if (channel.getOrdering() == Channel.Order.ORDER_UNORDERED) {
            DictDB<BigInteger, BigInteger> packetReceipt = packetReceipts.at(packet.getDestinationPort())
                    .at(packet.getDestinationChannel());
            Context.require(
                    packetReceipt.get(packet.getSequence()) == null,
                    "packet sequence already has been received");
            packetReceipt.set(packet.getSequence(), BigInteger.ONE);
        } else if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceDestinationPort =
                    nextSequenceReceives.at(packet.getDestinationPort());
            BigInteger nextSequenceRecv = nextSequenceDestinationPort.getOrDefault(packet.getDestinationChannel()
                    , BigInteger.ZERO);
            Context.require(
                    nextSequenceRecv.equals(packet.getSequence()),
                    "packet sequence != next receive sequence");
            nextSequenceDestinationPort.set(packet.getDestinationChannel(), nextSequenceRecv.add(BigInteger.ONE));
        } else {
            Context.revert("unknown ordering type");
        }
    }

    public void writeAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
                                     byte[] acknowledgement) {
        Context.require(acknowledgement.length > 0, "acknowledgement cannot be empty");

        Channel channel = Channel.decode(channels.at(destinationPortId).get(destinationChannel));
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        byte[] ackCommitmentKey = IBCCommitment.packetAcknowledgementCommitmentKey(destinationPortId,
                destinationChannel, sequence);
        Context.require(commitments.get(ackCommitmentKey) == null, "acknowledgement for packet already exists");
        byte[] ackCommitment = IBCCommitment.keccak256(IBCCommitment.sha256(acknowledgement));
        commitments.set(ackCommitmentKey, ackCommitment);
        sendBTPMessage(ByteUtil.join(ackCommitmentKey, ackCommitment));

    }

    public void acknowledgePacket(Packet packet, byte[] acknowledgement, byte[] proof, byte[] proofHeight) {
        Channel channel = Channel.decode(channels.at(packet.getSourcePort()).get(packet.getSourceChannel()));
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        Context.require(
                packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = ConnectionEnd.decode(connections.get(channel.getConnectionHops().get(0)));
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(packet.getSourcePort(),
                packet.getSourceChannel(), packet.getSequence());
        byte[] packetCommitment = commitments.get(packetCommitmentKey);
        Context.require(packetCommitment != null, "packet commitment not found");
        byte[] commitment = IBCCommitment.keccak256(createPacketCommitment(packet));

        Context.require(IBCCommitment.equals(packetCommitment, commitment), "commitment byte[] are not equal");

        byte[] packetAckPath = IBCCommitment.packetAcknowledgementCommitmentPath(packet.getDestinationPort(),
                packet.getDestinationChannel(), packet.getSequence());
        verifyPacketAcknowledgement(
                connection,
                proofHeight,
                proof,
                packetAckPath,
                IBCCommitment.sha256(acknowledgement));

        if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceAckSourcePort =
                    nextSequenceAcknowledgements.at(packet.getSourcePort());
            BigInteger nextSequenceAck = nextSequenceAckSourcePort.get(packet.getSourceChannel());
            Context.require(
                    nextSequenceAck.equals(packet.getSequence()),
                    "packet sequence != next ack sequence");
            nextSequenceAckSourcePort.set(packet.getSourceChannel(), nextSequenceAck.add(BigInteger.ONE));
        }

        commitments.set(packetCommitmentKey, null);
    }

    /* Verification functions */

    private void verifyPacketCommitment(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            byte[] path,
            byte[] commitmentBytes) {
        ILightClient client = getClient(connection.getClientId());
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                commitmentBytes);
        Context.require(ok, "failed to verify packet commitment");
    }

    private void verifyPacketAcknowledgement(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            byte[] path,
            byte[] acknowledgementCommitmentBytes) {
        ILightClient client = getClient(connection.getClientId());
        boolean ok = client.verifyMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                acknowledgementCommitmentBytes);
        Context.require(ok, "failed to verify packet acknowledgement commitment");
    }

    /* Internal functions */
    private BigInteger calcBlockDelay(BigInteger timeDelay) {
        BigInteger blockDelay = BigInteger.ZERO;
        BigInteger timePerBlock = expectedTimePerBlock.get();
        if (timePerBlock != null) {
            blockDelay = timeDelay.add(timePerBlock).subtract(BigInteger.ONE).divide(timePerBlock);
        }

        return blockDelay;
    }

    private byte[] createPacketCommitment(Packet packet) {
        return IBCCommitment.sha256(
                ByteUtil.join(
                        packet.getTimeoutTimestamp().toByteArray(),
                        packet.getTimeoutHeight().getRevisionNumber().toByteArray(),
                        packet.getTimeoutHeight().getRevisionHeight().toByteArray(),
                        packet.getData()));
    }

    private boolean isZero(Height height) {
        return height.getRevisionNumber().equals(BigInteger.ZERO) && height.getRevisionHeight().equals(BigInteger.ZERO);
    }

    private boolean lt(Height h1, Height h2) {
        return h1.getRevisionNumber().compareTo(h2.getRevisionNumber()) < 0
                || (h1.getRevisionNumber().equals(h2.getRevisionNumber())
                && h1.getRevisionHeight().compareTo(h2.getRevisionHeight()) < 0);
    }
}
