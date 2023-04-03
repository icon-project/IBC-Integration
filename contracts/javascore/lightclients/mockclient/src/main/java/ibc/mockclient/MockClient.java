package ibc.mockclient;

import java.math.BigInteger;
import java.util.Map;

import score.Context;
import score.annotation.External;
import ibc.icon.interfaces.ILightClient;
import icon.proto.core.client.Height;

import ibc.ics24.host.IBCCommitment;

public class MockClient implements ILightClient {

    public MockClient() {
    }

    @External(readonly = true)
    public BigInteger getTimestampAtHeight(
            String clientId,
            byte[] height) {
        return BigInteger.valueOf(Context.getBlockTimestamp());
    }

    @External(readonly = true)
    public byte[] getLatestHeight(String clientId) {
        Height height = new Height();
        height.setRevisionHeight(BigInteger.valueOf(Context.getBlockHeight()));
        return height.encode();
    }

    @External(readonly = true)
    public byte[] getConsensusState(
            String clientId,
            byte[] height) {
        return new byte[0];
    }

    @External(readonly = true)
    public byte[] getClientState(String clientId) {
        return new byte[0];
    }

    @External
    public Map<String, byte[]> createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes) {
        return Map.of(
            "clientStateCommitment", IBCCommitment.keccak256(clientStateBytes),
            "consensusStateCommitment", IBCCommitment.keccak256(consensusStateBytes),  
            "height", new Height().encode()
        );
    }

    @External(readonly = true)
    public Map<String, byte[]> updateClient(String clientId, byte[] clientMessageBytes) {
        return Map.of(
            "clientStateCommitment", IBCCommitment.keccak256(clientMessageBytes),
            "consensusStateCommitment", IBCCommitment.keccak256(clientMessageBytes),  
            "height", new Height().encode()
        );
    }

    @External
    public void verifyMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path,
            byte[] value) {
    }

    @External
    public void verifyNonMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path) {
    }
}
