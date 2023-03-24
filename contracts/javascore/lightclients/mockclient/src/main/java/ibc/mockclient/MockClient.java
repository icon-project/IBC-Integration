package ibc.mockclient;

import java.math.BigInteger;
import score.Context;
import score.annotation.External;
import ibc.icon.structs.messages.ConsensusStateUpdate;
import ibc.icon.structs.messages.UpdateClientResponse;
import icon.proto.core.client.Height;

import ibc.ics24.host.IBCCommitment;

public class MockClient {

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
        return null;
    }

    @External(readonly = true)
    public byte[] getClientState(String clientId) {
        return null;
    }

    @External
    public UpdateClientResponse createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes) {
        ConsensusStateUpdate update = new ConsensusStateUpdate(IBCCommitment.keccak256(consensusStateBytes),
                new Height().encode());
        UpdateClientResponse response = new UpdateClientResponse(IBCCommitment.keccak256(clientStateBytes), update,
                true);

        return response;
    }

    @External(readonly = true)
    public UpdateClientResponse updateClient(String clientId, byte[] clientMessageBytes) {
        ConsensusStateUpdate update = new ConsensusStateUpdate(IBCCommitment.keccak256(new byte[1]),
                new Height().encode());
        UpdateClientResponse response = new UpdateClientResponse(IBCCommitment.keccak256(clientMessageBytes), update,
                true);

        return response;
    }

    @External
    public boolean verifyMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path,
            byte[] value) {

        return true;
    }

    @External
    public boolean verifyNonMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path) {

        return true;
    }
}
