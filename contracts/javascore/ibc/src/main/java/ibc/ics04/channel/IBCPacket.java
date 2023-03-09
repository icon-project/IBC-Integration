package ibc.ics04.channel;

import java.math.BigInteger;

import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.ics24.host.IBCCommitment;

import score.Context;
import score.DictDB;

// TODO verify packet commitments follow a correct format
public class IBCPacket extends IBCChannelHandshake implements IIBCPacket {
    public void sendPacket(Packet packet) {
        Channel channel = channels.at(packet.getSourcePort()).get(packet.getSourceChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");
        Context.require(
                packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        ILightClient client = getClient(connection.getClientId());
        Height latestHeight = client.getLatestHeight(connection.getClientId());

        Context.require(
                packet.getTimeoutHeight().isZero() || latestHeight.lt(packet.getTimeoutHeight()),
                "receiving chain block height >= packet timeout height");
        BigInteger latestTimestamp = client.getTimestampAtHeight(connection.getClientId(), latestHeight);
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

    public void recvPacket(MsgPacketRecv msg) {
        Channel channel = channels.at(msg.packet.getDestinationPort()).get(msg.packet.getDestinationChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        // TODO
        // Authenticate capability to ensure caller has authority to receive packet on
        // this channel

        Context.require(
                msg.packet.getSourcePort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                msg.packet.getSourceChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        Context.require(
                msg.packet.getTimeoutHeight().getRevisionHeight().equals(BigInteger.ZERO)
                        || BigInteger.valueOf(Context.getBlockHeight())
                        .compareTo(msg.packet.getTimeoutHeight().getRevisionHeight()) < 0,
                "block height >= packet timeout height");
        Context.require(
                msg.packet.getTimeoutTimestamp().equals(BigInteger.ZERO)
                        || BigInteger.valueOf(Context.getBlockTimestamp())
                        .compareTo(msg.packet.getTimeoutTimestamp()) < 0,
                "block timestamp >= packet timeout timestamp");

        byte[] commitmentPath = IBCCommitment.packetCommitmentPath(msg.packet.getSourcePort(),
                msg.packet.getSourceChannel(), msg.packet.getSequence());
        byte[] commitmentBytes = IBCCommitment.keccak256(createPacketCommitment(msg.packet));

        verifyPacketCommitment(
                connection,
                msg.proofHeight,
                msg.proof,
                commitmentPath,
                commitmentBytes);

        if (channel.getOrdering() == Channel.Order.ORDER_UNORDERED) {
            DictDB<BigInteger, BigInteger> packetReceipt = packetReceipts.at(msg.packet.getDestinationPort())
                    .at(msg.packet.getDestinationChannel());
            Context.require(
                    packetReceipt.get(msg.packet.getSequence()) == null,
                    "packet sequence already has been received");
            packetReceipt.set(msg.packet.getSequence(), BigInteger.ONE);
        } else if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceDestinationPort =
                    nextSequenceReceives.at(msg.packet.getDestinationPort());
            BigInteger nextSequenceRecv = nextSequenceDestinationPort.getOrDefault(msg.packet.getDestinationChannel()
                    , BigInteger.ZERO);
            Context.require(
                    nextSequenceRecv.equals(msg.packet.sequence),
                    "packet sequence != next receive sequence");
            nextSequenceDestinationPort.set(msg.packet.getDestinationChannel(), nextSequenceRecv.add(BigInteger.ONE));
        } else {
            Context.revert("unknown ordering type");
        }
    }

    public void writeAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
                                     byte[] acknowledgement) {
        Context.require(acknowledgement.length > 0, "acknowledgement cannot be empty");

        Channel channel = channels.at(destinationPortId).get(destinationChannel);
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        byte[] ackCommitmentKey = IBCCommitment.packetAcknowledgementCommitmentKey(destinationPortId,
                destinationChannel, sequence);
        Context.require(commitments.get(ackCommitmentKey) == null, "acknowledgement for packet already exists");
        byte[] ackCommitment = IBCCommitment.keccak256(IBCCommitment.sha256(acknowledgement));
        commitments.set(ackCommitmentKey, ackCommitment);
        sendBTPMessage(ByteUtil.join(ackCommitmentKey, ackCommitment));

    }

    public void acknowledgePacket(MsgPacketAcknowledgement msg) {
        Channel channel = channels.at(msg.packet.getSourcePort()).get(msg.packet.getSourceChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        Context.require(
                msg.packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                msg.packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(msg.packet.getSourcePort(),
                msg.packet.getSourceChannel(), msg.packet.getSequence());
        byte[] packetCommitment = commitments.get(packetCommitmentKey);
        Context.require(packetCommitment != null, "packet commitment not found");
        byte[] commitment = IBCCommitment.keccak256(createPacketCommitment(msg.packet));

        Context.require(IBCCommitment.equals(packetCommitment, commitment), "commitment byte[] are not equal");

        byte[] packetAckPath = IBCCommitment.packetAcknowledgementCommitmentPath(msg.packet.destinationPort,
                msg.packet.destinationChannel, msg.packet.sequence);
        verifyPacketAcknowledgement(
                connection,
                msg.proofHeight,
                msg.proof,
                packetAckPath,
                IBCCommitment.sha256(msg.acknowledgement));

        if (channel.getOrdering() == Channel.Order.ORDER_ORDERED)  {
            DictDB<String, BigInteger> nextSequenceAckSourcePort =
                    nextSequenceAcknowledgements.at(msg.packet.getSourcePort());
            BigInteger nextSequenceAck = nextSequenceAckSourcePort.get(msg.packet.getSourceChannel());
            Context.require(
                    nextSequenceAck.equals(msg.packet.sequence),
                    "packet sequence != next ack sequence");
            nextSequenceAckSourcePort.set(msg.packet.getSourceChannel(), nextSequenceAck.add(BigInteger.ONE));
        }

        commitments.set(packetCommitmentKey, null);
    }

    /* Verification functions */

    private void verifyPacketCommitment(
            ConnectionEnd connection,
            Height height,
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
            Height height,
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
}
