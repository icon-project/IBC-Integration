package ibc.ics04.channel;

import java.math.BigInteger;

import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import score.Context;
import score.DictDB;

public class IBCPacket extends IBCChannelHandshake implements IIBCPacket {
    public void sendPacket(Packet packet) {
        Channel channel = store.channels.at(packet.getSourcePort()).get(packet.getSourceChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");
        Context.require(
                packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = store.connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        ILightClient client = new ILightClientScoreInterface(store.clientImpls.get(connection.getClientId()));
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

        BigInteger nextSequenceSend = store.nextSequenceSends.at(packet.getSourcePort())
                .getOrDefault(packet.getSourceChannel(), BigInteger.ZERO);
        Context.require(
                packet.getSequence().equals(nextSequenceSend),
                "packet sequence != next send sequence");

        store.nextSequenceSends.at(packet.getSourcePort())
                .set(packet.getSourceChannel(), nextSequenceSend.add(BigInteger.ONE));

        // TODO: IBC-Store
        // commitments[IBCCommitment.packetCommitmentKey(packet.source_port,
        // packet.source_channel,
        // packet.sequence)] = keccak256(
        // abi.encodePacked(
        // sha256(
        // abi.encodePacked(
        // packet.timeout_timestamp,
        // packet.timeout_height.revision_number,
        // packet.timeout_height.revision_height,
        // sha256(packet.data)))));
    }

    public void recvPacket(MsgPacketRecv msg) {
        Channel channel = store.channels.at(msg.packet.getSourcePort()).get(msg.packet.getSourceChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        // TODO
        // Authenticate capability to ensure caller has authority to receive packet on
        // this channel

        Context.require(
                msg.packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                msg.packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = store.connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState().equals(ConnectionEnd.State.STATE_OPEN), "connection state is not OPEN");

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

        byte[] commitmentPath = new byte[0]; // IBCCommitment.packetCommitmentPath(
        // msg.packet.source_port, msg.packet.source_channel, msg.packet.sequence),
        byte[] commitmentBytes = new byte[0]; // sha256(
        // abi.encodePacked(
        // msg.packet.timeout_timestamp,
        // msg.packet.timeout_height.revision_number,
        // msg.packet.timeout_height.revision_height,
        // sha256(msg.packet.data)))),
        verifyPacketCommitment(
                connection,
                msg.proofHeight,
                msg.proof,
                commitmentPath,
                commitmentBytes);

        if (channel.getOrdering().equals(Channel.Order.ORDER_UNORDERED)) {
            DictDB<BigInteger, BigInteger> packetReceipt = store.packetReceipts.at(msg.packet.getDestinationPort())
                    .at(msg.packet.getDestinationChannel());
            Context.require(
                    packetReceipt.get(msg.packet.getSequence()) == null,
                    "packet sequence already has been received");
            packetReceipt.set(msg.packet.getSequence(), BigInteger.ONE);
        } else if (channel.getOrdering().equals(Channel.Order.ORDER_ORDERED)) {
            BigInteger nextSequenceRecv = store.nextSequenceRecvs.at(msg.packet.getDestinationPort())
                    .getOrDefault(msg.packet.getDestinationChannel(), BigInteger.ZERO);
            Context.require(
                    nextSequenceRecv.equals(msg.packet.sequence),
                    "packet sequence != next receive sequence");
            store.nextSequenceRecvs.at(msg.packet.getDestinationPort()).set(msg.packet.getDestinationChannel(),
                    nextSequenceRecv.add(BigInteger.ONE));
        } else {
            Context.revert("unknown ordering type");
        }
    }

    public void writeAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
            byte[] acknowledgement) {
        Context.require(acknowledgement.length > 0, "acknowledgement cannot be empty");

        Channel channel = store.channels.at(destinationPortId).get(destinationChannel);
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        // bytes32 ackCommitmentKey =
        // IBCCommitment.packetAcknowledgementCommitmentKey(destinationPortId,
        // destinationChannel, sequence);
        // bytes32 ackCommitment = commitments[ackCommitmentKey];
        // Context.require(ackCommitment == bytes32(0), "acknowledgement for packet
        // already exists");
        // commitments[ackCommitmentKey] =
        // keccak256(abi.encodePacked(sha256(acknowledgement)));

    }

    public void acknowledgePacket(MsgPacketAcknowledgement msg) {
        Channel channel = store.channels.at(msg.packet.getSourcePort()).get(msg.packet.getSourceChannel());
        Context.require(channel.getState() == Channel.State.STATE_OPEN, "channel state must be OPEN");

        Context.require(
                msg.packet.getDestinationPort().equals(channel.getCounterparty().getPortId()),
                "packet destination port doesn't match the counterparty's port");
        Context.require(
                msg.packet.getDestinationChannel().equals(channel.getCounterparty().getChannelId()),
                "packet destination channel doesn't match the counterparty's channel");

        ConnectionEnd connection = store.connections.get(channel.getConnectionHops()[0]);
        Context.require(connection != null, "connection does not exist");
        Context.require(connection.getState().equals(ConnectionEnd.State.STATE_OPEN), "connection state is not OPEN");

        // bytes32 packetCommitmentKey =
        // IBCCommitment.packetCommitmentKey(msg.packet.source_port,
        // msg.packet.source_channel, msg.packet.sequence);
        // bytes32 packetCommitment = commitments[packetCommitmentKey];
        // Context.require(packetCommitment != bytes32(0), "packet commitment not
        // found");
        // Context.require(
        // packetCommitment
        // == keccak256(
        // abi.encodePacked(
        // sha256(
        // abi.encodePacked(
        // msg.packet.timeout_timestamp,
        // msg.packet.timeout_height.revision_number,
        // msg.packet.timeout_height.revision_height,
        // sha256(msg.packet.data)
        // )
        // )
        // )
        // ),
        // "commitment byte[] are not equal"
        // );

        // verifyPacketAcknowledgement(
        // connection,
        // msg.proofHeight,
        // msg.proof,
        // IBCCommitment.packetAcknowledgementCommitmentPath(
        // msg.packet.destination_port, msg.packet.destination_channel,
        // msg.packet.sequence),
        // sha256(msg.acknowledgement))

        if (channel.getOrdering().equals(Channel.Order.ORDER_ORDERED)) {
            BigInteger nextSequenceAck = store.nextSequenceAcks.at(msg.packet.getSourcePort())
                    .getOrDefault(msg.packet.getSourceChannel(), BigInteger.ZERO);
            Context.require(
                    nextSequenceAck.equals(msg.packet.sequence),
                    "packet sequence != next ack sequence");
            store.nextSequenceAcks.at(msg.packet.getSourcePort()).set(msg.packet.getSourceChannel(),
                    nextSequenceAck.add(BigInteger.ONE));
        }

        // delete commitments[packetCommitmentKey];
    }

    /* Verification functions */

    private void verifyPacketCommitment(
            ConnectionEnd connection,
            Height height,
            byte[] proof,
            byte[] path,
            byte[] commitmentBytes) {
        ILightClient client = new ILightClientScoreInterface(store.clientImpls.get(connection.getClientId()));
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
        ILightClient client = new ILightClientScoreInterface(store.clientImpls.get(connection.getClientId()));
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
        // if (expectedTimePerBlock != 0) {
        // blockDelay = (timeDelay + expectedTimePerBlock - 1) / expectedTimePerBlock;
        // }
        return blockDelay;
    }
}
