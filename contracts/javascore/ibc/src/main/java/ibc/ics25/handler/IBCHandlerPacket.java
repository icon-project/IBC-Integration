package ibc.ics25.handler;

import java.math.BigInteger;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.interfaces.IIBCPacket;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
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

    @External
    public void sendPacket(byte[] packetPb) {
        Packet packet = Packet.decode(packetPb);
        Context.require(
                authenticateCapability(channelCapabilityPath(packet.getSourcePort(), packet.getSourceChannel())),
                "failed to authenticate " + Context.getCaller() + " for port: " + packet.getSourcePort()
                        + "and channel: " + packet.getSourceChannel());
        super.sendPacket(packet);
        SendPacket(packetPb);
    }

    @External
    public void recvPacket(MsgPacketRecv msg) {
        Packet packet = msg.getPacket();
        super.recvPacket(packet, msg.getProof(), msg.getProofHeightRaw());

        IIBCModule module = lookupModuleByChannel(packet.getDestinationPort(),
                packet.getDestinationChannel());
        byte[] acknowledgement = module.onRecvPacket(msg.getPacketRaw(), Context.getCaller());
        if (acknowledgement.length > 0) {
            super.writeAcknowledgement(
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
        Context.require(authenticateCapability(channelCapabilityPath(destinationPortId,
                destinationChannel)),
                "failed to authenticate " + Context.getCaller() + " for port: " + destinationPortId
                        + "and channel: " + destinationChannel);
        super.writeAcknowledgement(
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
        super.acknowledgePacket(packet, msg.getAcknowledgement(), msg.getProof(), msg.getProofHeightRaw());

        module.onAcknowledgementPacket(msg.getPacketRaw(), msg.getAcknowledgement(),
                Context.getCaller());
        AcknowledgePacket(msg.getPacketRaw(), msg.getAcknowledgement());
    }
}
