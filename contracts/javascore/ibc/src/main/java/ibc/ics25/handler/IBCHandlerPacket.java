package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.messages.MsgPacketTimeout;
import icon.proto.core.channel.Packet;
import score.Context;
import score.annotation.EventLog;
import score.annotation.External;

import java.math.BigInteger;

public class IBCHandlerPacket extends IBCHandlerChannel implements IIBCPacket {
    @EventLog(indexed = 1)
    public void SendPacket(byte[] packet) {
    }

    @EventLog(indexed = 1)
    public void RecvPacket(byte[] packet) {
    }

    @EventLog(indexed = 3)
    public void WriteAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
                                     byte[] acknowledgement) {
    }

    @EventLog(indexed = 1)
    public void AcknowledgePacket(byte[] packet, byte[] acknowledgement) {
    }

    @EventLog(indexed = 1)
    public void TimeoutRequest(byte[] packet) {
    }

    @EventLog(indexed = 1)
    public void PacketTimeout(byte[] packet) {
    }

    @External
    public void sendPacket(byte[] packetPb) {
        Packet packet = Packet.decode(packetPb);
        Context.require(
                authenticateCapability(channelCapabilityPath(packet.getSourcePort(), packet.getSourceChannel())),
                "failed to authenticate " + Context.getCaller() + " for port: " + packet.getSourcePort()
                        + "and channel: " + packet.getSourceChannel());
        _sendPacket(packet);
        SendPacket(packetPb);
    }

    @External
    public void recvPacket(MsgPacketRecv msg) {
        Packet packet = Packet.decode(msg.getPacket());
        IIBCModule module = lookupModuleByChannel(packet.getDestinationPort(),
                packet.getDestinationChannel());

        byte[] acknowledgement = module.onRecvPacket(msg.getPacket(), Context.getCaller());
        _recvPacket(packet, msg.getProof(), msg.getProofHeight());

        if (acknowledgement.length > 0) {
            _writeAcknowledgement(
                    packet.getDestinationPort(),
                    packet.getDestinationChannel(),
                    packet.getSequence(),
                    acknowledgement);
            WriteAcknowledgement(packet.getDestinationPort(),
                    packet.getDestinationChannel(), packet.getSequence(), acknowledgement);
        }

        RecvPacket(msg.getPacket());
    }

    @External
    public void writeAcknowledgement(
            String destinationPortId,
            String destinationChannel,
            BigInteger sequence,
            byte[] acknowledgement) {
        Context.require(authenticateCapability(channelCapabilityPath(destinationPortId, destinationChannel)),
                "failed to authenticate " + Context.getCaller() + " for port: " + destinationPortId + "and channel: "
                        + destinationChannel);
        _writeAcknowledgement(
                destinationPortId,
                destinationChannel,
                sequence,
                acknowledgement);
        WriteAcknowledgement(destinationPortId, destinationChannel, sequence, acknowledgement);
    }

    @External
    public void acknowledgePacket(MsgPacketAcknowledgement msg) {
        Packet packet = Packet.decode(msg.getPacket());
        IIBCModule module = lookupModuleByChannel(packet.getSourcePort(), packet.getSourceChannel());

        module.onAcknowledgementPacket(msg.getPacket(), msg.getAcknowledgement(),
                Context.getCaller());
        _acknowledgePacket(packet, msg.getAcknowledgement(), msg.getProof(), msg.getProofHeight());

        AcknowledgePacket(msg.getPacket(), msg.getAcknowledgement());
    }

    @External
    public void requestTimeout(byte[] packetPb) {
        Packet packet = Packet.decode(packetPb);
        _requestTimeout(packet);

        TimeoutRequest(packetPb);
    }

    @External
    public void timeoutPacket(MsgPacketTimeout msg) {
        Packet packet = Packet.decode(msg.getPacket());
        IIBCModule module = lookupModuleByChannel(packet.getSourcePort(), packet.getSourceChannel());
        module.onTimeoutPacket(msg.getPacket(), Context.getCaller());
        _timeoutPacket(packet, msg.getProofHeight(), msg.getProof(), msg.getNextSequenceRecv());

        PacketTimeout(msg.getPacket());
    }

}
