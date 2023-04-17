package ibc.icon.interfaces;

import ibc.icon.structs.messages.MsgPacketAcknowledgement;
import ibc.icon.structs.messages.MsgPacketRecv;
import ibc.icon.structs.messages.MsgPacketTimeout;
import score.annotation.EventLog;


import foundation.icon.score.client.ScoreClient;

@ScoreClient
public interface IIBCPacket {

    @EventLog(indexed = 1)
    public void SendPacket(byte[] packet);

    @EventLog(indexed = 1)
    public void RecvPacket(byte[] packet);

    @EventLog(indexed = 1)
    public void WriteAcknowledgement(byte[] packet, byte[] acknowledgement);

    @EventLog(indexed = 1)
    public void AcknowledgePacket(byte[] packet, byte[] acknowledgement);

    @EventLog(indexed = 1)
    public void TimeoutRequest(byte[] packet);

    @EventLog(indexed = 1)
    public void PacketTimeout(byte[] packet);

    /**
     * {@code @dev} sendPacket is called by a module in order to send an IBC packet on a
     * channel.
     * The packet sequence generated for the packet to be sent is returned. An
     * error is returned if one occurs.
     */
    void sendPacket(byte[] packetPb);

    /**
     * {@code @dev} recvPacket is called by a module in order to receive & process an IBC
     * packet sent on the corresponding channel end on the counterparty chain.
     */
    void recvPacket(MsgPacketRecv msg);

    /**
     * {@code @dev} writeAcknowledgement writes the packet execution acknowledgement to the
     * state,which will be verified by the counterparty chain using AcknowledgePacket.
     */
    void writeAcknowledgement(byte[] packet, byte[] acknowledgement);

    /**
     * {@code @dev} AcknowledgePacket is called by a module to process the acknowledgement
     * of a packet previously sent by the calling module on a channel to a counterparty module on the counterparty
     * chain. Its intended usage is within the ante handler. AcknowledgePacket will clean up the packet commitment,
     * which is no longer necessary since the packet has been received and acted upon. It will also increment
     * NextSequenceAck in case of ORDERED channels.
     */
    void acknowledgePacket(MsgPacketAcknowledgement msg);

    /**
     * {@code @dev} requestTimeout is called by a module in order to prove the
     * absence of a packet on a channel.
     */
    void requestTimeout(byte[] packetPb);


    /**
     * {@code @dev} requestTimeout is called by a module in order to prove the
     * absence of a packet on a channel.
     */
    void timeoutPacket(MsgPacketTimeout msg);
}
