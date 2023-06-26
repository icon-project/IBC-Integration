package ibc.ics04.channel;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.ics24.host.IBCCommitment;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import icon.proto.core.connection.ConnectionEnd;
import score.Context;
import score.DictDB;

import java.math.BigInteger;

// TODO verify packet commitments follow a correct format
public class IBCPacket extends IBCChannelHandshake {


    public void _sendPacket(Packet packet) {
        byte[] channelPb = channels.at(packet.getSourcePort()).get(packet.getSourceChannel());
        Context.require(channelPb != null, "channel does not exist");
        Channel channel = Channel.decode(channelPb);

        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");
        Context.require(
                packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);
        ILightClient client = getClient(connection.getClientId());
        byte[] latestHeightRaw = client.getLatestHeight(connection.getClientId());
        Height latestHeight = Height.decode(latestHeightRaw);

        Context.require(lt(latestHeight, packet.getTimeoutHeight()),
                "receiving chain block height >= packet timeout height");
        BigInteger latestTimestamp = client.getTimestampAtHeight(connection.getClientId(), latestHeightRaw);
        Context.require(latestTimestamp != null, "consensusState not found");
        Context.require(packet.getTimeoutTimestamp().equals(BigInteger.ZERO),
                "Timeout timestamps are not available, use timeout height instead");


        DictDB<String, BigInteger> nextSequenceSourcePort = nextSequenceSends.at(packet.getSourcePort());
        BigInteger nextSequenceSend = nextSequenceSourcePort.getOrDefault(packet.getSourceChannel(), BigInteger.ZERO);
        Context.require(
                packet.getSequence().equals(nextSequenceSend),
                "packet sequence != next send sequence");

        nextSequenceSourcePort.set(packet.getSourceChannel(), nextSequenceSend.add(BigInteger.ONE));

        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(packet.getSourcePort(),
                packet.getSourceChannel(),
                packet.getSequence());

        byte[] packetCommitment = createPacketCommitment(packet);
        commitments.set(packetCommitmentKey, packetCommitment);

        sendBTPMessage(connection.getClientId(),
                ByteUtil.join(packetCommitmentKey, packetCommitment));
    }

    public void _recvPacket(Packet packet, byte[] proof, byte[] proofHeight) {
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

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);
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
        byte[] commitmentBytes = createPacketCommitmentBytes(packet);

        verifyPacketCommitment(
                connection,
                proofHeight,
                proof,
                commitmentPath,
                commitmentBytes);

        if (channel.getOrdering() == Channel.Order.ORDER_UNORDERED) {
            DictDB<BigInteger, Boolean> packetReceipt = packetReceipts.at(packet.getDestinationPort())
                    .at(packet.getDestinationChannel());
            Context.require(
                    packetReceipt.get(packet.getSequence()) == null,
                    "packet sequence already has been received");
            packetReceipt.set(packet.getSequence(), true);
        } else if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceDestinationPort = nextSequenceReceives
                    .at(packet.getDestinationPort());
            BigInteger nextSequenceRecv = nextSequenceDestinationPort.getOrDefault(packet.getDestinationChannel(),
                    BigInteger.ZERO);
            Context.require(
                    nextSequenceRecv.equals(packet.getSequence()),
                    "packet sequence != next receive sequence");
            nextSequenceDestinationPort.set(packet.getDestinationChannel(), nextSequenceRecv.add(BigInteger.ONE));
        } else {
            Context.revert("unknown ordering type");
        }
    }

    public void _writeAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
                                      byte[] acknowledgement) {
        Context.require(acknowledgement.length > 0, "acknowledgement cannot be empty");

        Channel channel = Channel.decode(channels.at(destinationPortId).get(destinationChannel));
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        byte[] ackCommitmentKey = IBCCommitment.packetAcknowledgementCommitmentKey(destinationPortId,
                destinationChannel, sequence);
        Context.require(commitments.get(ackCommitmentKey) == null, "acknowledgement for packet already exists");
        byte[] ackCommitment = IBCCommitment.keccak256(acknowledgement);
        commitments.set(ackCommitmentKey, ackCommitment);
        sendBTPMessage(connection.getClientId(), ByteUtil.join(ackCommitmentKey, ackCommitment));

    }

    public void _acknowledgePacket(Packet packet, byte[] acknowledgement, byte[] proof, byte[] proofHeight) {
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
        byte[] commitment = createPacketCommitment(packet);

        Context.require(IBCCommitment.equals(packetCommitment, commitment), "commitment byte[] are not equal");

        byte[] packetAckPath = IBCCommitment.packetAcknowledgementCommitmentPath(packet.getDestinationPort(),
                packet.getDestinationChannel(), packet.getSequence());
        verifyPacketAcknowledgement(
                connection,
                proofHeight,
                proof,
                packetAckPath,
                acknowledgement);

        if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceAckSourcePort = nextSequenceAcknowledgements
                    .at(packet.getSourcePort());
            BigInteger nextSequenceAck = nextSequenceAckSourcePort.get(packet.getSourceChannel());
            Context.require(
                    nextSequenceAck.equals(packet.getSequence()),
                    "packet sequence != next ack sequence");
            nextSequenceAckSourcePort.set(packet.getSourceChannel(), nextSequenceAck.add(BigInteger.ONE));
        }

        commitments.set(packetCommitmentKey, null);
    }

        public void _requestTimeout(Packet packet, byte[] proofHeight, byte[] proof) {

                byte[] packet_hash = IBCCommitment.keccak256(packet.encode());
                Context.require(!getRequestTimeout(packet_hash), "timeout packet request already exist");

        // TODO limit what packets can be timedout to limit spam creating of btp blocks.
                Channel channel = Channel
                                .decode(channels.at(packet.getDestinationPort()).get(packet.getDestinationChannel()));
        Context.require(
                packet.getSourcePort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getSourceChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);
        Context.require(connection.getState() == ConnectionEnd.State.STATE_OPEN,
                "connection state is not OPEN");

        boolean heightTimeout = packet.getTimeoutHeight().getRevisionHeight().compareTo(BigInteger.ZERO) > 0
                && BigInteger.valueOf(Context.getBlockHeight())
                .compareTo(packet.getTimeoutHeight().getRevisionHeight()) >= 0;
        boolean timeTimeout = packet.getTimeoutTimestamp().compareTo(BigInteger.ZERO) > 0
                && BigInteger.valueOf(Context.getBlockTimestamp())
                .compareTo(packet.getTimeoutTimestamp()) < 0;
        Context.require(heightTimeout || timeTimeout, "Packet has not yet timed out");

                byte[] commitmentPath = IBCCommitment.packetCommitmentPath(packet.getSourcePort(),
                                packet.getSourceChannel(), packet.getSequence());
                byte[] commitmentBytes = createPacketCommitmentBytes(packet);
                verifyPacketCommitment(
                                connection,
                                proofHeight,
                                proof,
                                commitmentPath,
                                commitmentBytes);

        if (channel.getOrdering() == Channel.Order.ORDER_UNORDERED) {
            DictDB<BigInteger, Boolean> packetReceipt = packetReceipts.at(packet.getDestinationPort())
                    .at(packet.getDestinationChannel());
            Context.require(
                    packetReceipt.get(packet.getSequence()) == null,
                    "packet sequence already has been received");

            sendBTPMessage(connection.getClientId(), IBCCommitment.packetReceiptCommitmentKey(
                                        packet.getDestinationPort(), packet.getDestinationChannel(),
                                        packet.getSequence()));
        } else if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            DictDB<String, BigInteger> nextSequenceDestinationPort = nextSequenceReceives
                    .at(packet.getDestinationPort());
                        BigInteger nextSequenceRecv = nextSequenceDestinationPort.getOrDefault(
                                        packet.getDestinationChannel(),
                    BigInteger.ZERO);
            Context.require(
                    nextSequenceRecv.equals(packet.getSequence()),
                    "packet sequence != next receive sequence");

                        byte[] recvCommitmentKey = IBCCommitment.nextSequenceRecvCommitmentKey(
                                        packet.getDestinationPort(),
                    packet.getDestinationChannel());
            byte[] recvCommitment = Proto.encodeFixed64(packet.getSequence(), false);
                        // ordered channel: check that the recv sequence is as claimed
            sendBTPMessage(connection.getClientId(), ByteUtil.join(recvCommitmentKey, recvCommitment));
        } else {
            Context.revert("unknown ordering type");
        }
                setRequestTimeout(packet_hash);

    }

    public void _timeoutPacket(Packet packet, byte[] proofHeight, byte[] proof, BigInteger nextSequenceRecv) {
        Channel channel = Channel.decode(channels.at(packet.getSourcePort()).get(packet.getSourceChannel()));

        // abortTransactionUnless(authenticateCapability(channelCapabilityPath(packet.getSourcePort(),
        // packet.getSourceChannel())))
        Context.require(packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()));
        Context.require(packet.getDestinationPort().equals(channel.getCounterparty().getPortId()));

        // note: the connection may have been closed
        byte[] connectionPb = connections.get(channel.getConnectionHops().get(0));
        Context.require(connectionPb != null, "connection does not exist");
        ConnectionEnd connection = ConnectionEnd.decode(connectionPb);

        // check that timeout height or timeout timestamp has passed on the other end
        Height height = Height.decode(proofHeight);
        boolean heightTimeout = packet.getTimeoutHeight().getRevisionHeight().compareTo(BigInteger.ZERO) > 0
                && height.getRevisionHeight().compareTo(packet.getTimeoutHeight().getRevisionHeight()) >= 0;
        Context.require(heightTimeout, "Packet has not yet timed out");

        // verify we actually sent this packet, check the store
        byte[] packetCommitmentKey = IBCCommitment.packetCommitmentKey(packet.getSourcePort(),
                packet.getSourceChannel(), packet.getSequence());
        byte[] packetCommitment = commitments.get(packetCommitmentKey);
        Context.require(packetCommitment != null, "packet commitment not found");
        byte[] commitment = createPacketCommitment(packet);

        Context.require(IBCCommitment.equals(packetCommitment, commitment), "commitment byte[] are not equal");

        if (channel.getOrdering() == Channel.Order.ORDER_UNORDERED) {
            byte[] packetReceiptKey = IBCCommitment.packetReceiptCommitmentPath(
                    packet.getDestinationPort(),
                    packet.getDestinationChannel(),
                    packet.getSequence());

            verifyPacketReceiptAbsence(
                    connection,
                    proofHeight,
                    proof,
                    packetReceiptKey
            );
        } else if (channel.getOrdering() == Channel.Order.ORDER_ORDERED) {
            // ordered channel: check that packet has not been received
            // only allow timeout on next sequence so all sequences before the timed out
            // packet are processed (received/timed out)
            // before this packet times out
            Context.require(
                    nextSequenceRecv.equals(packet.getSequence()),
                    "packet sequence != next receive sequence");
            byte[] nextRecvKey = IBCCommitment.nextSequenceRecvCommitmentPath(
                    packet.getDestinationPort(),
                    packet.getDestinationChannel());
            // ordered channel: check that the recv sequence is as claimed
            verifyNextSequenceRecv(
                    connection,
                    proofHeight,
                    proof,
                    nextRecvKey,
                    Proto.encodeFixed64(nextSequenceRecv, false));
            channel.setState(Channel.State.STATE_CLOSED);

            byte[] encodedChannel = channel.encode();
            updateChannelCommitment(connection.getClientId(), packet.getSourcePort(), packet.getSourceChannel(), encodedChannel);
            channels.at(packet.getSourcePort()).set(packet.getSourceChannel(), encodedChannel);
        } else {
            Context.revert("unknown ordering type");
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
        client.verifyMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                commitmentBytes);
    }

    private void verifyPacketAcknowledgement(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            byte[] path,
            byte[] acknowledgementCommitmentBytes) {
        ILightClient client = getClient(connection.getClientId());
        client.verifyMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                acknowledgementCommitmentBytes);
    }

    private void verifyNextSequenceRecv(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            byte[] path,
            byte[] commitmentBytes) {
        ILightClient client = getClient(connection.getClientId());
        client.verifyMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path,
                commitmentBytes);
    }

    private void verifyPacketReceiptAbsence(
            ConnectionEnd connection,
            byte[] height,
            byte[] proof,
            byte[] path) {
        ILightClient client = getClient(connection.getClientId());
        client.verifyNonMembership(
                connection.getClientId(),
                height,
                connection.getDelayPeriod(),
                calcBlockDelay(connection.getDelayPeriod()),
                proof,
                connection.getCounterparty().getPrefix().getKeyPrefix(),
                path);
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
        return IBCCommitment.keccak256(createPacketCommitmentBytes(packet));
    }

    public static byte[] createPacketCommitmentBytes(Packet packet) {
        return ByteUtil.join(
                Proto.encodeFixed64(packet.getTimeoutTimestamp(), false),
                Proto.encodeFixed64(packet.getTimeoutHeight().getRevisionNumber(),
                        false),
                Proto.encodeFixed64(packet.getTimeoutHeight().getRevisionHeight(),
                        false),
                IBCCommitment.keccak256(packet.getData()));
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
