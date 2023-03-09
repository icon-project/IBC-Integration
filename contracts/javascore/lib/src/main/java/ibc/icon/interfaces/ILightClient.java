package ibc.icon.interfaces;

import foundation.icon.score.client.ScoreInterface;
import ibc.icon.structs.messages.CreateClientResponse;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.icon.structs.proto.core.client.Height;

import java.math.BigInteger;

/**
 * {@code @dev} This defines an interface for Light Client contract can be integrated
 * with ibc-solidity.
 * You can register the Light Client contract that implements this through
 * `registerClient` on IBCHandler.
 */
@ScoreInterface
public interface ILightClient {
    /**
     * {@code @dev} createClient creates a new client with the given state.
     * If succeeded, it returns a commitment for the initial state.
     */
    CreateClientResponse createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes);

    /**
     * {@code @dev} getTimestampAtHeight returns the timestamp of the consensus state at the
     * given height.
     */
    BigInteger getTimestampAtHeight(String clientId, Height height);

    /**
     * {@code @dev} getLatestHeight returns the latest height of the client state
     * corresponding to `clientId`.
     */
    Height getLatestHeight(String clientId);

    /**
     * {@code @dev} updateClient updates the client corresponding to `clientId`.
     * If succeeded, it returns a commitment for the updated state.
     * If there are no updates for consensus state, this public void should return an empty array as `updates`.
     * <p>
     * NOTE: updateClient is intended to perform the followings:
     * 1. verify a given client message(e.g. header)
     * 2. check misbehaviour such like duplicate block height
     * 3. if misbehaviour is found, update state accordingly and return
     * 4. update state(s) with the client message
     * 5. persist the state(s) on the host
     */
    UpdateClientResponse updateClient(String clientId, byte[] clientMessageBytes);

    /**
     * {@code @dev} verifyMembership is a generic proof verification method which verifies a
     * proof of the existence of a value at a given CommitmentPath at the
     * specified height.
     * The caller is expected to construct the full CommitmentPath from a
     * CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    Boolean verifyMembership(
            String clientId,
            Height height,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            String prefix,
            byte[] path,
            byte[] value);

    /**
     * {@code @dev} verifyNonMembership is a generic proof verification method which
     * verifies the absence of a given CommitmentPath at a specified height.
     * The caller is expected to construct the full CommitmentPath from a
     * CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    Boolean verifyNonMembership(
            String clientId,
            Height height,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            String prefix,
            byte[] path);

    /**
     * {@code @dev} getClientState returns the clientState corresponding to `clientId`.
     * If it's not found, the public void returns false.
     */
    byte[] getClientState(String clientId);

    /**
     * {@code @dev} getConsensusState returns the consensusState corresponding to `clientId`
     * and `height`.
     * If it's not found, the public void returns false.
     */
    byte[] getConsensusState(String clientId, Height height);
}