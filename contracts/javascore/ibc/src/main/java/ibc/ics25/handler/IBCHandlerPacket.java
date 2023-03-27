package ibc.ics25.handler;

import java.math.BigInteger;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.messages.MsgPacketTimeout;
import icon.proto.core.channel.Packet;
import score.Context;
import score.annotation.EventLog;
import score.annotation.External;

public abstract class IBCHandlerPacket extends IBCHandlerChannel implements IIBCPacket {
    @EventLog
    public void SendPacket(byte[] packet) {
    }

    @EventLog
    public void RecvPacket(byte[] packet) {
    }

    @EventLog
    public void WriteAcknowledgement(String destinationPortId, String destinationChannel, BigInteger sequence,
            byte[] acknowledgement) {
    }

    @EventLog
    public void AcknowledgePacket(byte[] packet, byte[] acknowledgement) {
    }

    @EventLog
    public void TimeoutRequest(byte[] packet) {
    }

    @EventLog
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
        Packet packet = msg.getPacket();
        IIBCModule module = lookupModuleByChannel(packet.getDestinationPort(),
                packet.getDestinationChannel());

        byte[] acknowledgement = module.onRecvPacket(msg.getPacketRaw(), Context.getCaller());
        _recvPacket(packet, msg.getProof(), msg.getProofHeightRaw());

        if (acknowledgement.length > 0) {
            _writeAcknowledgement(
                    packet.getDestinationPort(),
                    packet.getDestinationChannel(),
                    packet.getSequence(),
                    acknowledgement);
            WriteAcknowledgement(packet.getDestinationPort(),
                    packet.getDestinationChannel(), packet.getSequence(), acknowledgement);
        }

        RecvPacket(msg.getPacketRaw());
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
        Packet packet = msg.getPacket();
        IIBCModule module = lookupModuleByChannel(packet.getSourcePort(), packet.getSourceChannel());

        module.onAcknowledgementPacket(msg.getPacketRaw(), msg.getAcknowledgement(),
                Context.getCaller());
        _acknowledgePacket(packet, msg.getAcknowledgement(), msg.getProof(), msg.getProofHeightRaw());

        AcknowledgePacket(msg.getPacketRaw(), msg.getAcknowledgement());
    }

    @External
    public void requestTimeout(byte[] packetPb) {
        Packet packet = Packet.decode(packetPb);
        _requestTimeout(packet);

        TimeoutRequest(packetPb);
    }

    @External
    public void timeoutPacket(MsgPacketTimeout msg) {
        Packet packet = msg.getPacket();
        IIBCModule module = lookupModuleByChannel(packet.getSourcePort(), packet.getSourceChannel());
        module.onTimeoutPacket(msg.getPacketRaw(), Context.getCaller());
        _timeoutPacket(packet, msg.getProofHeightRaw(), msg.getProof(), msg.getNextSequenceRecv());

        PacketTimeout(msg.getPacketRaw());
    }

}
