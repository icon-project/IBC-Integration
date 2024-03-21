package ibc.ics08.tendermint;

import icon.ibc.interfaces.ILightClient;
import ibc.icon.score.util.NullChecker;
import ibc.icon.score.util.StringUtil;
import ibc.ics23.commitment.types.Merkle;
import ibc.ics24.host.IBCCommitment;
import google.protobuf.*;
import tendermint.types.*;
import ibc.core.commitment.v1.*;
import ibc.lightclients.tendermint.v1.*;
import ibc.core.client.v1.Height;
import score.Address;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.Map;

import static ibc.ics08.tendermint.TendermintHelper.*;
import static ibc.ics23.commitment.types.Merkle.applyPrefix;
import static score.Context.require;

public class ICS08TendermintLightClient extends Tendermint implements ILightClient {
    public final Address ibcHandler;

    public static final String CLIENT_STATES = "CLIENT_STATES";
    public static final String CONSENSUS_STATES = "CONSENSUS_STATES";
    public static final String PROCESSED_TIMES = "PROCESSED_TIMES";
    public static final String PROCESSED_HEIGHTS = "PROCESSED_HEIGHTS";

    public static final DictDB<String, byte[]> clientStates = Context.newDictDB(CLIENT_STATES, byte[].class);
    public static final BranchDB<String, DictDB<BigInteger, byte[]>> consensusStates = Context.newBranchDB(
            CONSENSUS_STATES, byte[].class);
    public static final BranchDB<String, DictDB<BigInteger, BigInteger>> processedTimes = Context.newBranchDB(
            PROCESSED_TIMES, BigInteger.class);
    public static final BranchDB<String, DictDB<BigInteger, BigInteger>> processedHeights = Context.newBranchDB(
            PROCESSED_HEIGHTS, BigInteger.class);

    public ICS08TendermintLightClient(Address ibcHandler) {
        this.ibcHandler = ibcHandler;
    }

    private void onlyHandler() {
        Address caller = Context.getCaller();
        Context.require(caller.equals(ibcHandler), "Only the IBC handler: " + ibcHandler + " is allowed");
    }

    /**
     * @dev getTimestampAtHeight returns the timestamp of the consensus state at the
     *      given height.
     */
    @External(readonly = true)
    public BigInteger getTimestampAtHeight(
            String clientId,
            byte[] height) {
        Height decodedHeight = Height.decode(height);
        byte[] encodedConsensusState = consensusStates.at(clientId).get(decodedHeight.getRevisionHeight());
        NullChecker.requireNotNull(encodedConsensusState,
                "height: " + decodedHeight.getRevisionHeight() + " does not have a consensus state");
        ConsensusState consensusState = ConsensusState.decode(encodedConsensusState);
        return consensusState.getTimestamp().getSeconds();
    }

    /**
     * @dev getLatestHeight returs latest height stored in the given client state
     */
    @External(readonly = true)
    public byte[] getLatestHeight(String clientId) {
        byte[] encodedClientState = clientStates.get(clientId);
        NullChecker.requireNotNull(encodedClientState, "Client does not exist");
        ClientState clientState = ClientState.decode(encodedClientState);
        return clientState.getLatestHeight().encode();
    }

    @External(readonly = true)
    public byte[] getConsensusState(
            String clientId,
            byte[] height) {
        Height decodedHeight = Height.decode(height);
        return consensusStates.at(clientId).get(decodedHeight.getRevisionHeight());
    }

    @External(readonly = true)
    public byte[] getClientState(String clientId) {
        return clientStates.get(clientId);
    }

    /**
     * @dev createClient creates a new client with the given state
     */
    @External
    public Map<String, byte[]> createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes) {
        onlyHandler();
        Context.require(clientStates.get(clientId) == null, "Client already exists");
        ClientState clientState = ClientState.decode(clientStateBytes);

        validateTrustLevel(clientState.getTrustLevel());

        clientStates.set(clientId, clientStateBytes);
        consensusStates.at(clientId).set(clientState.getLatestHeight().getRevisionHeight(), consensusStateBytes);

        return Map.of(
                "clientStateCommitment", IBCCommitment.keccak256(clientStateBytes),
                "consensusStateCommitment", IBCCommitment.keccak256(consensusStateBytes),
                "height", clientState.getLatestHeight().encode());
    }

    /**
     * @dev checkHeaderAndUpdateState validates the header
     */
    @External
    public Map<String, byte[]> updateClient(String clientId, byte[] clientMessageBytes) {
        onlyHandler();
        ibc.lightclients.tendermint.v1.Header tmHeader = ibc.lightclients.tendermint.v1.Header
                .decode(clientMessageBytes);
        boolean conflictingHeader = checkForDuplicateHeader(clientId, tmHeader);

        byte[] encodedClientState = clientStates.get(clientId);
        require(encodedClientState != null, "LC: client state is invalid");
        ClientState clientState = ClientState.decode(encodedClientState);

        byte[] encodedTrustedConsensusState = consensusStates.at(clientId)
                .get(tmHeader.getTrustedHeight().getRevisionHeight());
        require(encodedTrustedConsensusState != null, "LC: consensusState not found at trusted height");
        ConsensusState trustedConsensusState = ConsensusState.decode(encodedTrustedConsensusState);

        checkValidity(clientState, trustedConsensusState, tmHeader);

        // Header is different from existing consensus state and also valid, so freeze
        // the client and return
        if (conflictingHeader) {
            return handleConflict(clientId, tmHeader, clientState);
        }
        // update the consensus state from a new header and set processed time metadata
        return updateHeader(clientId, tmHeader, clientState, encodedClientState);

    }

    @External(readonly = true)
    public void verifyMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path,
            byte[] value) {
        Height height = Height.decode(heightBytes);
        ClientState clientState = ClientState.decode(mustGetClientState(clientId));
        validateArgs(clientState, height.getRevisionHeight(), prefix, proof);
        validateDelayPeriod(clientId, height, delayTimePeriod, delayBlockPeriod);

        ConsensusState consensusState = ConsensusState
                .decode(mustGetConsensusState(clientId, height.getRevisionHeight()));

        var root = consensusState.getRoot();
        var merkleProof = MerkleProof.decode(proof);
        var merklePath = applyPrefix(StringUtil.bytesToHex(prefix), StringUtil.bytesToHex(path));

        Merkle.verifyMembership(merkleProof, Merkle.SDK_SPEC, root, merklePath, value);
    }

    @External(readonly = true)
    public void verifyNonMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path) {
        Height height = Height.decode(heightBytes);
        ClientState clientState = ClientState.decode(mustGetClientState(clientId));
        validateArgs(clientState, height.getRevisionHeight(), prefix, proof);
        validateDelayPeriod(clientId, height, delayTimePeriod, delayBlockPeriod);

        ConsensusState consensusState = ConsensusState
                .decode(mustGetConsensusState(clientId, height.getRevisionHeight()));

        var root = consensusState.getRoot();
        var merkleProof = MerkleProof.decode(proof);
        var merklePath = applyPrefix(StringUtil.bytesToHex(prefix), StringUtil.bytesToHex(path));

        Merkle.verifyNonMembership(merkleProof, Merkle.SDK_SPEC, root, merklePath);
    }

    private Map<String, byte[]> updateHeader(String clientId, ibc.lightclients.tendermint.v1.Header tmHeader,
            ClientState clientState, byte[] encodedClientState) {
        if (tmHeader.getSignedHeader().getHeader().getHeight()
                .compareTo(clientState.getLatestHeight().getRevisionHeight()) > 0) {
            BigInteger revision = getRevisionNumber(tmHeader.getSignedHeader().getHeader().getChainId());
            clientState.setLatestHeight(newHeight(tmHeader.getSignedHeader().getHeader().getHeight(), revision));
            encodedClientState = clientState.encode();
            clientStates.set(clientId, encodedClientState);
        }

        byte[] encodedConsensusState = toConsensusState(tmHeader).encode();
        consensusStates.at(clientId).set(tmHeader.getSignedHeader().getHeader().getHeight(),
                encodedConsensusState);
        processedHeights.at(clientId).set(tmHeader.getSignedHeader().getHeader().getHeight(),
                BigInteger.valueOf(Context.getBlockHeight()));
        processedTimes.at(clientId).set(tmHeader.getSignedHeader().getHeader().getHeight(),
                BigInteger.valueOf(Context.getBlockTimestamp()));

        return Map.of(
                "clientStateCommitment", IBCCommitment.keccak256(encodedClientState),
                "consensusStateCommitment", IBCCommitment.keccak256(encodedConsensusState),
                "height", clientState.getLatestHeight().encode());
    }

    private Map<String, byte[]> handleConflict(String clientId, ibc.lightclients.tendermint.v1.Header tmHeader,
            ClientState clientState) {
        BigInteger revision = getRevisionNumber(tmHeader.getSignedHeader().getHeader().getChainId());
        clientState.setFrozenHeight(newHeight(tmHeader.getSignedHeader().getHeader().getHeight(), revision));
        byte[] encodedClientState = clientState.encode();
        clientStates.set(clientId, encodedClientState);

        byte[] encodedConsensusState = toConsensusState(tmHeader).encode();
        consensusStates.at(clientId).set(clientState.getLatestHeight().getRevisionHeight(), encodedConsensusState);
        processedHeights.at(clientId).set(tmHeader.getSignedHeader().getHeader().getHeight(),
                BigInteger.valueOf(Context.getBlockHeight()));
        processedTimes.at(clientId).set(tmHeader.getSignedHeader().getHeader().getHeight(),
                BigInteger.valueOf(Context.getBlockTimestamp()));

        return Map.of(
                "clientStateCommitment", IBCCommitment.keccak256(encodedClientState),
                "consensusStateCommitment", IBCCommitment.keccak256(encodedConsensusState),
                "height",
                newHeight(tmHeader.getSignedHeader().getHeader().getHeight(), revision).encode());
    }

    private boolean checkForDuplicateHeader(String clientId, ibc.lightclients.tendermint.v1.Header tmHeader) {
        // Check if the Client store already has a consensus state for the header's
        // height
        // If the consensus state exists, and it matches the header then we return early
        // since header has already been submitted in a previous UpdateClient.
        byte[] prevConsState = consensusStates.at(clientId)
                .get(tmHeader.getSignedHeader().getHeader().getHeight());
        if (prevConsState == null) {
            return false;
        }

        // This header has already been submitted and the necessary state is already
        // stored
        Context.require(!Arrays.equals(prevConsState, toConsensusState(tmHeader).encode()),
                "LC: This header has already been submitted");

        // A consensus state already exists for this height, but it does not match the
        // provided header.
        // Thus, we must check that this header is valid, and if so we will freeze the
        // client.
        return true;
    };

    // checkValidity checks if the Tendermint header is valid.
    public void checkValidity(
            ClientState clientState,
            ConsensusState trustedConsensusState,
            ibc.lightclients.tendermint.v1.Header tmHeader) {
        // assert header height is newer than consensus state
        require(
                tmHeader.getSignedHeader().getHeader().getHeight()
                        .compareTo(tmHeader.getTrustedHeight().getRevisionHeight()) > 0,
                "LC: Trusted height is higher than untrusted header height");

        tendermint.types.Header lc = new tendermint.types.Header();
        lc.setChainId(clientState.getChainId());
        lc.setHeight(tmHeader.getTrustedHeight().getRevisionHeight());
        lc.setTime(trustedConsensusState.getTimestamp());
        lc.setNextValidatorsHash(trustedConsensusState.getNextValidatorsHash());

        ValidatorSet trustedVals = tmHeader.getTrustedValidators();
        SignedHeader trustedHeader = new SignedHeader();
        trustedHeader.setHeader(lc);

        SignedHeader untrustedHeader = tmHeader.getSignedHeader();
        ValidatorSet untrustedVals = tmHeader.getValidatorSet();

        Timestamp currentTime = getCurrentTime();
        Context.require(!isExpired(trustedHeader, clientState.getTrustingPeriod(), currentTime),
                "header can't be expired");

        boolean ok = verify(
                clientState.getMaxClockDrift(),
                clientState.getTrustLevel(),
                trustedHeader,
                trustedVals,
                untrustedHeader,
                untrustedVals,
                currentTime);

        require(ok, "LC: failed to verify header");
    }

    private void validateArgs(ClientState cs, BigInteger height, byte[] prefix, byte[] proof) {
        Context.require(cs.getLatestHeight().getRevisionHeight().compareTo(height) >= 0,
                "Latest height must be greater or equal to proof height");
        Context.require(cs.getFrozenHeight().getRevisionHeight().equals(BigInteger.ZERO) ||
                cs.getFrozenHeight().getRevisionHeight().compareTo(height) >= 0,
                "Client is Frozen");
        Context.require(prefix.length > 0, "Prefix cant be empty");
        Context.require(proof.length > 0, "Proof cant be empty");
    }

    private void validateDelayPeriod(String clientId, Height height,
            BigInteger delayPeriodTime,
            BigInteger delayPeriodBlocks) {
        BigInteger currentTime = BigInteger.valueOf(Context.getBlockTimestamp());
        BigInteger validTime = mustGetProcessedTime(clientId,
                height.getRevisionHeight()).add(delayPeriodTime);

        BigInteger currentHeight = BigInteger.valueOf(Context.getBlockHeight());
        BigInteger validHeight = mustGetProcessedHeight(clientId,
                height.getRevisionHeight()).add(delayPeriodBlocks);

        Context.require(currentTime.compareTo(validTime) >= 0, "Delay Time period has not yet passed");
        Context.require(currentHeight.compareTo(validHeight) >= 0, "Delay Height has not yet passed");
    }

    private byte[] mustGetClientState(String clientId) {
        byte[] clientState = clientStates.get(clientId);
        require(clientState != null, "LC: client state not found");
        return clientState;
    }

    private byte[] mustGetConsensusState(String clientId, BigInteger height) {
        byte[] consensusState = consensusStates.at(clientId).get(height);
        require(consensusState != null, "LC: consensus state not found");
        return consensusState;
    }

    private BigInteger mustGetProcessedTime(String clientId, BigInteger height) {
        BigInteger processedTime = processedTimes.at(clientId).get(height);
        require(processedTime != null, "LC: processed time not found");
        return processedTime;
    }

    private BigInteger mustGetProcessedHeight(String clientId, BigInteger height) {
        BigInteger processedHeight = processedHeights.at(clientId).get(height);
        require(processedHeight != null, "LC: processed height not found");
        return processedHeight;
    }
}
