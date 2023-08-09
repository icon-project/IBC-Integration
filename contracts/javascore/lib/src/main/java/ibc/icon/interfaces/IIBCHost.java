package ibc.icon.interfaces;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

import foundation.icon.score.client.ScoreClient;
import score.Address;
import score.annotation.External;

@ScoreClient
public interface IIBCHost {
    @External(readonly = true)
    byte[] getCommitment(byte[] key);

    @External(readonly = true)
    Address getClientRegistry(String type);

    @External(readonly = true)
    String getClientType(String clientId);

    @External(readonly = true)
    Address getClientImplementation(String clientId);

    @External(readonly = true)
    byte[] getConnection(String connectionId);

    @External(readonly = true)
    byte[] getChannel(String portId, String channelId);

    @External(readonly = true)
    BigInteger getNextSequenceSend(String portId, String channelId);

    @External(readonly = true)
    BigInteger getNextSequenceReceive(String portId, String channelId);

    @External(readonly = true)
    BigInteger getNextSequenceAcknowledgement(String portId, String channelId);

    @External(readonly = true)
    boolean getPacketReceipt(String portId, String channelId, BigInteger sequence);

    @External(readonly = true)
    Address getCapability(byte[] name);

    @External(readonly = true)
    BigInteger getExpectedTimePerBlock();

    @External(readonly = true)
    BigInteger getNextClientSequence();

    @External(readonly = true)
    BigInteger getNextConnectionSequence();

    @External(readonly = true)
    BigInteger getNextChannelSequence();

    @External(readonly = true)
    byte[] getLatestHeight(String clientId);

    @External(readonly = true)
    byte[] getClientState(String clientId);

    @External(readonly = true)
    byte[] getConsensusState(String clientId, byte[] height);

    @External(readonly = true)
    byte[] getPacketCommitment(String portId, String channelId, BigInteger sequence);

    @External(readonly = true)
    Map<String, Long> getPacketHeights(String portId, String channelId, int startSequence, int endSequence);

    @External(readonly = true)
    byte[] getPacketAcknowledgementCommitment(String portId, String channelId, BigInteger sequence);

    @External(readonly = true)
    boolean hasPacketReceipt(String portId, String channelId, BigInteger sequence);

    @External(readonly = true)
    List<Integer> getMissingPacketReceipts(String portId, String channelId, int startSequence, int endSequence);

    @External(readonly = true)
    int getBTPNetworkId(String clientId);
}
