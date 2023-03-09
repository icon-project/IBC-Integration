package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.proto.core.channel.Packet;
import score.Context;
import score.annotation.EventLog;
import score.annotation.External;

import java.math.BigInteger;

public abstract class IBCHandlerPacket extends IBCHandlerChannel {
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
    public void sendPacket(Packet packet) {
        Context.require(
                authenticateCapability(channelCapabilityPath(packet.getSourcePort(), packet.getSourceChannel())),
                "failed to authenticate " + Context.getCaller() + " for port: " + packet.getSourcePort()
                        + "and channel: " + packet.getSourceChannel());
        super.sendPacket(packet);
        SendPacket(packet.toBytes());
    }

    @External
    public void recvPacket(MsgPacketRecv msg) {
        super.recvPacket(msg);

        IIBCModule module = lookupModuleByChannel(msg.packet.getDestinationPort(),
                msg.packet.getDestinationChannel());
        byte[] acknowledgement = module.onRecvPacket(msg.packet, Context.getCaller());
        if (acknowledgement.length > 0) {
            super.writeAcknowledgement(
                    msg.packet.getDestinationPort(),
                    msg.packet.getDestinationChannel(),
                    msg.packet.sequence,
                    acknowledgement);
            WriteAcknowledgement(msg.packet.getDestinationPort(),
                    msg.packet.getDestinationChannel(), msg.packet.sequence, acknowledgement);
        }

        RecvPacket(msg.packet.toBytes());
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
        super.acknowledgePacket(msg);
        IIBCModule module = lookupModuleByChannel(msg.packet.getSourcePort(),
                msg.packet.getSourceChannel());
        module.onAcknowledgementPacket(msg.packet, msg.acknowledgement,
                Context.getCaller());
        AcknowledgePacket(msg.packet.toBytes(), msg.acknowledgement);
    }
}
