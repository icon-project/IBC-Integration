package ibc.mockclient;

import java.math.BigInteger;
import score.Context;
import score.annotation.External;
import ibc.icon.structs.messages.UpdateClientResponse;
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
        return null;
    }

    @External(readonly = true)
    public byte[] getClientState(String clientId) {
        return null;
    }

    @External
    public UpdateClientResponse createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes) {
        UpdateClientResponse response = new UpdateClientResponse(
            IBCCommitment.keccak256(clientStateBytes),
            IBCCommitment.keccak256(consensusStateBytes),  
            new Height().encode()
        );

        return response;
    }

    @External(readonly = true)
    public UpdateClientResponse updateClient(String clientId, byte[] clientMessageBytes) {
        UpdateClientResponse response = new UpdateClientResponse(
            IBCCommitment.keccak256(clientMessageBytes),
            IBCCommitment.keccak256(clientMessageBytes),
            new Height().encode()
        );

        return response;
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
