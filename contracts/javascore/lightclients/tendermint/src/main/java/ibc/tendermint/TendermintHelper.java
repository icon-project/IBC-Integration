package ibc.tendermint;

import java.math.BigInteger;
import java.util.Arrays;

import ibc.icon.score.util.MerkleTree;
import ibc.icon.score.util.Proto;
import icon.proto.clients.tendermint.CanonicalVote;
import icon.proto.clients.tendermint.Commit;
import icon.proto.clients.tendermint.CommitSig;
import icon.proto.clients.tendermint.ConsensusState;
import icon.proto.clients.tendermint.Duration;
import icon.proto.clients.tendermint.LightHeader;
import icon.proto.clients.tendermint.MerkleRoot;
import icon.proto.clients.tendermint.SignedHeader;
import icon.proto.clients.tendermint.SignedMsgType;
import icon.proto.clients.tendermint.SimpleValidator;
import icon.proto.clients.tendermint.Timestamp;
import icon.proto.clients.tendermint.TmHeader;
import icon.proto.clients.tendermint.Validator;
import icon.proto.clients.tendermint.ValidatorSet;
import icon.proto.core.client.Height;
import score.Context;

public class TendermintHelper {
    public static final BigInteger MICRO_SECONDS_IN_A_SECOND = BigInteger.valueOf(1_000_000);

    public static CanonicalVote toCanonicalVote(Commit commit, int valIdx, String chainId) {
        CommitSig commitSig = commit.getSignatures().get(valIdx);
        CanonicalVote vote = new CanonicalVote();

        vote.setType(SignedMsgType.SIGNED_MSG_TYPE_PRECOMMIT);
        vote.setHeight(commit.getHeight());
        vote.setRound(commit.getRound());
        vote.setBlockId(commit.getBlockId());
        vote.setTimestamp(commitSig.getTimestamp());
        vote.setChainId(chainId);
        return vote;
    }

    public static ConsensusState toConsensusState(TmHeader header) {
        ConsensusState state = new ConsensusState();
        state.setNextValidatorsHash(header.getSignedHeader().getHeader().getNextValidatorsHash());
        state.setTimestamp(header.getSignedHeader().getHeader().getTime());
        MerkleRoot merkleRoot = new MerkleRoot();
        merkleRoot.setHash(header.getSignedHeader().getHeader().getAppHash());
        state.setRoot(merkleRoot);

        return state;
    }

    public static BigInteger getTotalVotingPower(ValidatorSet validatorSet) {
        BigInteger sum = BigInteger.ZERO;
        for (Validator validator : validatorSet.getValidators()) {
            sum = sum.add(validator.getVotingPower());
        }

        validatorSet.setTotalVotingPower(sum);
        return validatorSet.getTotalVotingPower();
    }

    public static int getByAddress(ValidatorSet validatorSet, byte[] addr) {
        int size = validatorSet.getValidators().size();
        for (int idx = 0; idx < size; idx++) {
            if (Arrays.equals(validatorSet.getValidators().get(idx).getAddress(), addr)) {
                return idx;
            }
        }

        return -1;
    }

    public static Height newHeight(BigInteger blockHeight) {
        Height height = new Height();
        height.setRevisionHeight(blockHeight);
        height.setRevisionNumber(BigInteger.ZERO);

        return height;
    }

    public static SimpleValidator toSimpleValidator(Validator validator) {
        SimpleValidator simpleValidator = new SimpleValidator();
        simpleValidator.setPubKey(validator.getPubKey());
        simpleValidator.setVotingPower(validator.getVotingPower());

        return simpleValidator;
    }

    public static boolean gt(Timestamp t1, Timestamp t2) {
        if (t1.getSeconds().compareTo(t2.getSeconds()) > 0) {
            return true;
        }

        if (t1.getSeconds().equals(t2.getSeconds()) && t1.getNanos().compareTo(t2.getNanos()) > 0) {
            return true;
        }

        return false;
    }

    public static boolean isExpired(SignedHeader header, Duration trustingPeriod, Timestamp currentTime) {
        Timestamp expirationTime = new Timestamp();
        expirationTime.setSeconds(header.getHeader().getTime().getSeconds().add(trustingPeriod.getSeconds()));
        expirationTime.setNanos(header.getHeader().getTime().getNanos());

        return gt(currentTime, expirationTime);
    }

    public static Timestamp getCurrentTime() {
        BigInteger timeInMicro = BigInteger.valueOf(Context.getBlockTimestamp());
        BigInteger seconds = timeInMicro.divide(MICRO_SECONDS_IN_A_SECOND);
        BigInteger microSeconds = timeInMicro.subtract(seconds.multiply(MICRO_SECONDS_IN_A_SECOND));
        BigInteger nanoSeconds = microSeconds.multiply(BigInteger.valueOf(1000));

        Timestamp currentTime = new Timestamp();
        currentTime.setSeconds(seconds);
        currentTime.setNanos(nanoSeconds);

        return currentTime;
    }

    public static byte[] hash(ValidatorSet validatorSet) {
        int size = validatorSet.getValidators().size();
        byte[][] data = new byte[size][];
        for (int i = 0; i < size; i++) {
            data[i] = toSimpleValidator(validatorSet.getValidators().get(i)).encode();
        }

        return MerkleTree.merkleRootHash(data, 0, size);
    }

    public static byte[] hash(LightHeader header) {
        byte[] hbz = Proto.encode(1, header.getVersion().getBlock());
        byte[] pbt = header.getTime().encode();
        byte[] bzbi = header.getLastBlockId().encode();

        byte[][] all = new byte[][] {
                hbz,
                Proto.encode(1, header.getChainId()),
                Proto.encode(1, header.getHeight()),
                pbt,
                bzbi,
                Proto.encode(1, header.getLastCommitHash()),
                Proto.encode(1, header.getDataHash()),
                Proto.encode(1, header.getValidatorsHash()),
                Proto.encode(1, header.getNextValidatorsHash()),
                Proto.encode(1, header.getConsensusHash()),
                Proto.encode(1, header.getAppHash()),
                Proto.encode(1, header.getLastResultsHash()),
                Proto.encode(1, header.getEvidenceHash()),
                Proto.encode(1, header.getProposerAddress())
        };

        return MerkleTree.merkleRootHash(all, 0, all.length);
    }
}
