package ibc.ics08.tendermint;

import java.math.BigInteger;
import java.util.Arrays;

import ibc.icon.score.util.MerkleTree;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ByteUtil;
import tendermint.types.Commit;
import tendermint.types.CommitSig;
import ibc.lightclients.tendermint.v1.ConsensusState;
import tendermint.types.Header;
import ibc.core.commitment.v1.MerkleRoot;
import tendermint.types.SignedHeader;
import tendermint.types.SignedMsgType;
import tendermint.types.SimpleValidator;
import tendermint.types.Validator;
import tendermint.types.ValidatorSet;
import ibc.core.client.v1.Height;

import google.protobuf.Timestamp;
import google.protobuf.Duration;
import score.Context;

public class TendermintHelper {
    public static final BigInteger MICRO_SECONDS_IN_A_SECOND = BigInteger.valueOf(1_000_000);

    public static byte[] toCanonicalVote(Commit commit, int valIdx, String chainId) {
        CommitSig commitSig = commit.getSignatures().get(valIdx);
        return ByteUtil.join(
            Proto.encode(1, SignedMsgType.SIGNED_MSG_TYPE_PRECOMMIT),
            Proto.encodeFixed64(2, commit.getHeight()),
            Proto.encodeFixed64(3, commit.getRound()),
            Proto.encode(4, commit.getBlockId()),
            Proto.encode(5, commitSig.getTimestamp()),
            Proto.encode(6, chainId));
    }

    public static ConsensusState toConsensusState(ibc.lightclients.tendermint.v1.Header header) {
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

    public static BigInteger getRevisionNumber(String chainId) {
        int id = chainId.lastIndexOf("-");
        if (id >= 0) {
            return new BigInteger(chainId.substring(id+1));
        }
        return BigInteger.ZERO;
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

    public static Height newHeight(BigInteger blockHeight, BigInteger revision) {
        Height height = new Height();
        height.setRevisionHeight(blockHeight);
        height.setRevisionNumber(revision);

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
        BigInteger microSeconds = timeInMicro.remainder(MICRO_SECONDS_IN_A_SECOND);
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

    public static byte[] hash(Header header) {
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
