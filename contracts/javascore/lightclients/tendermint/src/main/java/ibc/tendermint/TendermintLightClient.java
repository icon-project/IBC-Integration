package ibc.tendermint;

import java.math.BigInteger;

import score.Address;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.annotation.External;
import ibc.icon.structs.messages.ConsensusStateUpdate;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.icon.structs.proto.core.client.Height;

import static score.Context.require;

import ibc.icon.structs.proto.lightclient.tendermint.*;
import ibc.ics24.host.IBCCommitment;

public class TendermintLightClient extends Tendermint {
    public final Address ibcHandler;
    public static final BigInteger MICRO_SECONDS_IN_A_SECOND = BigInteger.valueOf(1_000_000);

    public static final String CLIENT_STATES = "CLIENT_STATES";
    public static final String CONSENSUS_STATES = "CONSENSUS_STATES";
    public static final String PROCESSED_TIMES = "PROCESSED_TIMES";
    public static final String PROCESSED_HEIGHTS = "PROCESSED_HEIGHTS";

    public static final DictDB<String, ClientState> clientStates = Context.newDictDB(CLIENT_STATES, ClientState.class);
    public static final BranchDB<String, DictDB<BigInteger, ConsensusState>> consensusStates = Context.newBranchDB(
            CONSENSUS_STATES, ConsensusState.class);
    public static final BranchDB<String, DictDB<BigInteger, BigInteger>> processedTimes = Context.newBranchDB(
            PROCESSED_TIMES, BigInteger.class);
    public static final BranchDB<String, DictDB<BigInteger, BigInteger>> processedHeights = Context.newBranchDB(
            PROCESSED_HEIGHTS, BigInteger.class);

    public TendermintLightClient(Address ibcHandler) {
        this.ibcHandler = ibcHandler;
    }

    /**
     * @dev getTimestampAtHeight returns the timestamp of the consensus state at the
     *      given height.
     */
    @External(readonly = true)
    public BigInteger getTimestampAtHeight(
            Address host,
            String clientId,
            Height height) {
        ConsensusState consensusState = consensusStates.at(clientId).get(height.revisionHeight);

        return consensusState.timestamp.seconds;
    }

    /**
     * @dev getLatestHeight returs latest height stored in the given client state
     */
    @External(readonly = true)
    public Height getLatestHeight(String clientId) {
        ClientState clientState = clientStates.get(clientId);
        // if (!found) {
        // return (Height(0, 0), false);
        // }
        return new Height(BigInteger.ZERO, clientState.latestHeight);
    }

    /**
     * @dev createClient creates a new client with the given state
     */
    @External
    public UpdateClientResponse createClient(String clientId, byte[] clientStateBytes, byte[] consensusStateBytes) {
        ClientState clientState = ClientState.fromBytes(clientStateBytes);
        ConsensusState consensusState = ConsensusState.fromBytes(consensusStateBytes);
        clientStates.set(clientId, clientState);
        consensusStates.at(clientId).set(clientState.latestHeight, consensusState);
        ConsensusStateUpdate update = new ConsensusStateUpdate(IBCCommitment.keccak256(consensusStateBytes),
                new Height(BigInteger.ZERO, clientState.latestHeight));
        UpdateClientResponse response = new UpdateClientResponse(IBCCommitment.keccak256(clientStateBytes), update,
                true);

        return response;
    }

    /**
     * @dev checkHeaderAndUpdateState validates the header
     */
    @External(readonly = true)
    public UpdateClientResponse updateClient(String clientId, byte[] clientMessageBytes) {
        TmHeader tmHeader = TmHeader.fromBytes(clientMessageBytes);
        boolean conflictingHeader = false;

        // Check if the Client store already has a consensus state for the header's
        // height
        // If the consensus state exists, and it matches the header then we return early
        // since header has already been submitted in a previous UpdateClient.
        // TODO: revision number?
        ConsensusState prevConsState = consensusStates.at(clientId).get(tmHeader.signedHeader.header.height);
        if (prevConsState != null) {
            // This header has already been submitted and the necessary state is already
            // stored
            Context.require(!prevConsState.isEqual(tmHeader.toConsensusState()),
                    "block already exists in consensus state");

            // A consensus state already exists for this height, but it does not match the
            // provided header.
            // Thus, we must check that this header is valid, and if so we will freeze the
            // client.
            conflictingHeader = true;
        }

        ConsensusState trustedConsensusState = consensusStates.at(clientId).get(tmHeader.trustedHeight);
        require(trustedConsensusState != null, "LC: consensusState not found at trusted height");

        ClientState clientState = clientStates.get(clientId);
        require(clientState != null, "LC: client state is invalid");
        Duration currentTime = new Duration(
                BigInteger.valueOf(Context.getBlockTimestamp() / MICRO_SECONDS_IN_A_SECOND.longValue()),
                BigInteger.ZERO);
        checkValidity(clientState, trustedConsensusState, tmHeader, currentTime);

        // Header is different from existing consensus state and also valid, so freeze
        // the client and return
        if (conflictingHeader) {
            clientState.frozenHeight = tmHeader.signedHeader.header.height;
            clientStates.set(clientId, clientState);
            consensusStates.at(clientId).set(clientState.latestHeight, tmHeader.toConsensusState());
            processedHeights.at(clientId).set(tmHeader.signedHeader.header.height,
                    BigInteger.valueOf(Context.getBlockHeight()));
            processedTimes.at(clientId).set(tmHeader.signedHeader.header.height,
                    BigInteger.valueOf(Context.getBlockTimestamp()));

            ConsensusStateUpdate consensusStateUpdate = new ConsensusStateUpdate(tmHeader.toConsensusState().toBytes(),
                    new Height(BigInteger.ZERO, tmHeader.signedHeader.header.height));
            UpdateClientResponse response = new UpdateClientResponse(clientState.toBytes(), consensusStateUpdate, true);

            return response;
        }

        // TODO: check consensus state monotonicity

        // update the consensus state from a new header and set processed time metadata
        if (tmHeader.signedHeader.header.height.compareTo(clientState.latestHeight) > 0) {
            clientState.latestHeight = tmHeader.signedHeader.header.height;
        }

        clientStates.set(clientId, clientState);
        consensusStates.at(clientId).set(clientState.latestHeight, tmHeader.toConsensusState());
        processedHeights.at(clientId).set(tmHeader.signedHeader.header.height,
                BigInteger.valueOf(Context.getBlockHeight()));
        processedTimes.at(clientId).set(tmHeader.signedHeader.header.height,
                BigInteger.valueOf(Context.getBlockTimestamp()));
        ConsensusStateUpdate consensusStateUpdate = new ConsensusStateUpdate(tmHeader.toConsensusState().toBytes(),
                new Height(BigInteger.ZERO, clientState.latestHeight));
        UpdateClientResponse response = new UpdateClientResponse(clientState.toBytes(), consensusStateUpdate, true);

        return response;
    }

    // checkValidity checks if the Tendermint header is valid.
    private void checkValidity(
            ClientState clientState,
            ConsensusState trustedConsensusState,
            TmHeader tmHeader,
            Duration currentTime) {
        // assert header height is newer than consensus state
        require(
                tmHeader.signedHeader.header.height.compareTo(tmHeader.trustedHeight) > 0,
                "LC: header height consensus state height");

        LightHeader lc = new LightHeader();
        lc.chainId = clientState.chainId;
        lc.height = tmHeader.trustedHeight;
        lc.time = trustedConsensusState.timestamp;
        lc.nextValidatorsHash = trustedConsensusState.nextValidatorsHash;

        ValidatorSet trustedVals = tmHeader.trustedValidators;
        SignedHeader trustedHeader = new SignedHeader();
        trustedHeader.header = lc;

        SignedHeader untrustedHeader = tmHeader.signedHeader;
        ValidatorSet untrustedVals = tmHeader.validatorSet;

        boolean ok = verify(
                clientState.trustingPeriod,
                clientState.maxClockDrift,
                clientState.trustLevel,
                trustedHeader,
                trustedVals,
                untrustedHeader,
                untrustedVals,
                currentTime);

       require(ok, "LC: failed to verify header");
    }

    public boolean verifyChannelState(
            String clientId,
            BigInteger height,
            byte[] prefix,
            byte[] proof,
            String portId,
            String channelId,
            byte[] channelBytes // serialized with pb
    ) {

        ClientState clientState = clientStates.get(clientId);
        if (clientState == null) {
            return false;
        }
        if (!validateArgs(clientState, height, prefix, proof)) {
            return false;
        }
        ConsensusState consensusState = consensusStates.at(clientId).get(height);
        if (consensusState == null) {
            return false;
        }
        return verifyMembership(proof, consensusState.root.hash, prefix,
                IBCCommitment.channelCommitmentKey(portId, channelId),
                IBCCommitment.keccak256(channelBytes));
    }

    public boolean verifyPacketCommitment(
            String clientId,
            BigInteger height,
            BigInteger delayPeriodTime,
            BigInteger delayPeriodBlocks,
            byte[] prefix,
            byte[] proof,
            String portId,
            String channelId,
            BigInteger sequence,
            byte[] commitmentBytes) {

        ClientState clientState = clientStates.get(clientId);
        if (clientState == null) {
            return false;
        }
        if (!validateArgs(clientState, height, prefix, proof)) {
            return false;
        }
        if (!validateDelayPeriod(clientId, height, delayPeriodTime, delayPeriodBlocks)) {
            return false;
        }
        ConsensusState consensusState = consensusStates.at(clientId).get(height);
        if (consensusState == null) {
            return false;
        }
        return verifyMembership(proof, consensusState.root.hash, prefix,
                IBCCommitment.packetCommitmentKey(portId, channelId, sequence), commitmentBytes);
    }

    public boolean verifyPacketAcknowledgement(
            String clientId,
            BigInteger height,
            BigInteger delayPeriodTime,
            BigInteger delayPeriodBlocks,
            byte[] prefix,
            byte[] proof,
            String portId,
            String channelId,
            BigInteger sequence,
            byte[] acknowledgement) {
        ClientState clientState = clientStates.get(clientId);
        require(clientState != null, "LC: client state not found");
        if (!validateArgs(clientState, height, prefix, proof)) {
            return false;
        }
        if (!validateDelayPeriod(clientId, height, delayPeriodTime, delayPeriodBlocks)) {
            return false;
        }

        byte[] stateRoot = mustGetConsensusState(clientId, height).root.hash;
        byte[] ackCommitmentSlot = IBCCommitment.packetAcknowledgementCommitmentKey(portId, channelId, sequence);
        byte[] ackCommitment = IBCCommitment.sha256(acknowledgement);
        return verifyMembership(proof, stateRoot, prefix, ackCommitmentSlot, ackCommitment);
    }

    public boolean verifyClientState(
            Address host,
            String clientId,
            BigInteger height,
            byte[] prefix,
            String counterpartyClientIdentifier,
            byte[] proof,
            byte[] clientStateBytes) {

        ClientState clientState = clientStates.get(clientId);
        if (clientState == null) {
            return false;
        }
        if (!validateArgs(clientState, height, prefix, proof)) {
            return false;
        }
        ConsensusState consensusState = consensusStates.at(clientId).get(height);
        if (consensusState == null) {
            return false;
        }
        return verifyMembership(proof, consensusState.root.hash, prefix,
                IBCCommitment.clientStateCommitmentKey(counterpartyClientIdentifier),
                IBCCommitment.keccak256(clientStateBytes));
    }

    public boolean verifyClientConsensusState(
            Address host,
            String clientId,
            BigInteger height,
            String counterpartyClientIdentifier,
            BigInteger consensusHeight,
            byte[] prefix,
            byte[] proof,
            byte[] consensusStateBytes // serialized with pb
    ) {
        ClientState clientState = clientStates.get(clientId);
        if (clientState == null) {
            return false;
        }
        if (!validateArgs(clientState, height, prefix, proof)) {
            return false;
        }
        ConsensusState consensusState = consensusStates.at(clientId).get(height);
        if (consensusState == null) {
            return false;
        }
        return verifyMembership(proof, consensusState.root.hash, prefix,
                IBCCommitment.consensusStateCommitmentKey(counterpartyClientIdentifier, BigInteger.ZERO,
                        consensusHeight),
                IBCCommitment.keccak256(consensusStateBytes));
    }

    public boolean validateArgs(ClientState cs, BigInteger height, byte[] prefix, byte[] proof) {
        if (cs.latestHeight.compareTo(height) < 0) {
            return false;
        } else if (prefix.length == 0) {
            return false;
        } else if (proof.length == 0) {
            return false;
        }
        return true;
    }

    public boolean validateDelayPeriod(String clientId, BigInteger height, BigInteger delayPeriodTime,
            BigInteger delayPeriodBlocks) {
        BigInteger currentTime = BigInteger.valueOf(Context.getBlockTimestamp() * 1000 * 1000 * 1000);
        BigInteger validTime = mustGetProcessedTime(clientId, height).add(delayPeriodTime);
        if (currentTime.compareTo(validTime) < 0) {
            return false;
        }
        BigInteger currentHeight = BigInteger.valueOf(Context.getBlockHeight());
        BigInteger validHeight = mustGetProcessedHeight(clientId, height).add(delayPeriodBlocks);
        if (currentHeight.compareTo(validHeight) < 0) {
            return false;
        }
        return true;
    }

    // NOTE: this is a workaround to avoid the error `Stack too deep` in caller side
    public ConsensusState mustGetConsensusState(String clientId, BigInteger height) {
        ConsensusState consensusState = consensusStates.at(clientId).get(height);
        require(consensusState != null, "LC: consensus state not found");
        return consensusState;
    }

    public BigInteger mustGetProcessedTime(String clientId, BigInteger height) {
        BigInteger processedTime = processedTimes.at(clientId).get(height);
        require(processedTime != null, "LC: processed time not found");
        return processedTime;
    }

    public BigInteger mustGetProcessedHeight(String clientId, BigInteger height) {
        BigInteger processedHeight = processedHeights.at(clientId).get(height);
        require(processedHeight != null, "LC: processed height not found");
        return processedHeight;
    }

    public boolean verifyMembership(
            byte[] proof,
            byte[] root,
            byte[] prefix,
            byte[] slot,
            byte[] expectedValue) {
        return true;
        // CommitmentProof commitmentProof = CommitmentProof.decode(proof);

        // Ics23.VerifyMembershipError vCode = Ics23.verifyMembership(_tmProofSpec,
        // root.toBytes(), commitmentProof, slot.toBytes(), expectedValue.toBytes());

        // return vCode == Ics23.VerifyMembershipError.None;
    }
}
