package ibc.tendermint;

import java.math.BigInteger;
import java.util.Arrays;

import ibc.icon.score.util.Proto;
import icon.proto.clients.tendermint.*;
import score.Context;

import static ibc.tendermint.TendermintHelper.*;

public abstract class Tendermint {
    protected boolean verify(
            Duration trustingPeriod,
            Duration maxClockDrift,
            Fraction trustLevel,
            SignedHeader trustedHeader,
            ValidatorSet trustedVals,
            SignedHeader untrustedHeader,
            ValidatorSet untrustedVals,
            Duration currentTime) {
        if (!untrustedHeader.getHeader().getHeight()
                .equals(trustedHeader.getHeader().getHeight().add(BigInteger.ONE))) {
            return verifyNonAdjacent(
                    trustedHeader,
                    trustedVals,
                    untrustedHeader,
                    untrustedVals,
                    trustingPeriod,
                    currentTime,
                    maxClockDrift,
                    trustLevel);
        }

        return verifyAdjacent(trustedHeader, untrustedHeader, untrustedVals, trustingPeriod, currentTime,
                maxClockDrift);
    }

    protected boolean verifyAdjacent(
            SignedHeader trustedHeader,
            SignedHeader untrustedHeader,
            ValidatorSet untrustedVals,
            Duration trustingPeriod,
            Duration currentTime,
            Duration maxClockDrift) {
        Context.require(
                untrustedHeader.getHeader().getHeight()
                        .equals(trustedHeader.getHeader().getHeight().add(BigInteger.ONE)),
                "headers must be adjacent in height");

        Context.require(!isExpired(trustedHeader, trustingPeriod, currentTime), "header can't be expired");

        verifyNewHeaderAndVals(untrustedHeader, untrustedVals, trustedHeader, currentTime, maxClockDrift);

        // Check the validator hashes are the same
        Context.require(
                Arrays.equals(untrustedHeader.getHeader().getValidatorsHash(),
                        trustedHeader.getHeader().getNextValidatorsHash()),
                "expected old header next validators to match those from new header");

        // Ensure that +2/3 of new validators signed correctly.
        return verifyCommitLight(
                untrustedVals,
                trustedHeader.getHeader().getChainId(),
                untrustedHeader.getCommit().getBlockId(),
                untrustedHeader.getHeader().getHeight(),
                untrustedHeader.getCommit());

    }

    protected boolean verifyNonAdjacent(
            SignedHeader trustedHeader,
            ValidatorSet trustedVals,
            SignedHeader untrustedHeader,
            ValidatorSet untrustedVals,
            Duration trustingPeriod,
            Duration currentTime,
            Duration maxClockDrift,
            Fraction trustLevel) {
        Context.require(
                !untrustedHeader.getHeader().getHeight()
                        .equals(trustedHeader.getHeader().getHeight().add(BigInteger.ONE)),
                "LC: headers must be non adjacent in height");

        // assert that trustedVals is NextValidators of last trusted header
        // to do this, we check that trustedVals.Hash() == consState.NextValidatorsHash
        Context.require(Arrays.equals(hash(trustedVals), trustedHeader.getHeader().getNextValidatorsHash()),
                "LC: headers trusted validators does not hash to latest trusted validators");

        Context.require(!isExpired(trustedHeader, trustingPeriod, currentTime), "header can't be expired");

        verifyNewHeaderAndVals(untrustedHeader, untrustedVals, trustedHeader, currentTime, maxClockDrift);

        // Ensure that +`trustLevel` (default 1/3) or more of last trusted validators
        // signed correctly.
        verifyCommitLightTrusting(trustedVals, trustedHeader.getHeader().getChainId(), untrustedHeader.getCommit(),
                trustLevel);

        // Ensure that +2/3 of new validators signed correctly.
        return verifyCommitLight(
                untrustedVals,
                trustedHeader.getHeader().getChainId(),
                untrustedHeader.getCommit().getBlockId(),
                untrustedHeader.getHeader().getHeight(),
                untrustedHeader.getCommit());

    }

    protected void verifyNewHeaderAndVals(
            SignedHeader untrustedHeader,
            ValidatorSet untrustedVals,
            SignedHeader trustedHeader,
            Duration currentTime,
            Duration maxClockDrift) {
        // SignedHeader validate basic
        Context.require(untrustedHeader.getHeader().getChainId().equals(trustedHeader.getHeader().getChainId()),
                "header belongs to another chain");
        Context.require(untrustedHeader.getCommit().getHeight().equals(untrustedHeader.getHeader().getHeight()),
                "header and commit height mismatch");

        byte[] untrustedHeaderBlockHash = hash(untrustedHeader.getHeader());
        Context.require(Arrays.equals(untrustedHeaderBlockHash, untrustedHeader.getCommit().getBlockId().getHash()),
                "commit signs signs block failed");

        Context.require(untrustedHeader.getHeader().getHeight().compareTo(trustedHeader.getHeader().getHeight()) > 0,
                "expected new header height to be greater than one of old header");
        Context.require(
                gt(untrustedHeader.getHeader().getTime(), trustedHeader.getHeader().getTime()),
                "expected new header time to be after old header time");

        Timestamp curentTimestamp = new Timestamp();
        curentTimestamp.setSeconds(currentTime.getSeconds().add(maxClockDrift.getSeconds()));
        curentTimestamp.setNanos(currentTime.getNanos().add(maxClockDrift.getNanos()));
        Context.require(gt(curentTimestamp, untrustedHeader.getHeader().getTime()),
                "new header has time from the future");

        byte[] validatorsHash = hash(untrustedVals);
        Context.require(Arrays.equals(untrustedHeader.getHeader().getValidatorsHash(), validatorsHash),
                "expected new header validators to match those that were supplied at height XX");
    }

    protected boolean verifyCommitLightTrusting(
            ValidatorSet trustedVals,
            String chainID,
            Commit commit,
            Fraction trustLevel) {
        // sanity check
        Context.require(!trustLevel.getDenominator().equals(BigInteger.ZERO), "trustLevel has zero Denominator");

        BigInteger talliedVotingPower = BigInteger.ZERO;
        boolean[] seenVals = new boolean[trustedVals.getValidators().size()];

        CommitSig commitSig;
        BigInteger totalVotingPowerMulByNumerator = trustedVals.getTotalVotingPower()
                .multiply(trustLevel.getNumerator());
        BigInteger votingPowerNeeded = totalVotingPowerMulByNumerator.divide(trustLevel.getDenominator());

        int signaturesLength = commit.getSignatures().size();
        for (int idx = 0; idx < signaturesLength; idx++) {
            commitSig = commit.getSignatures().get(idx);

            // no need to verify absent or nil votes.
            if (commitSig.getBlockIdFlag() != BlockIDFlag.BLOCK_ID_FLAG_COMMIT) {
                continue;
            }

            // We don't know the validators that committed this block, so we have to
            // check for each vote if its validator is already known.
            int valIdx = getByAddress(trustedVals, commitSig.getValidatorAddress());
            if (valIdx == -1) {
                continue;
            }

            // check for double vote of validator on the same commit
            Context.require(!seenVals[valIdx], "double vote of validator on the same commit");
            seenVals[valIdx] = true;

            Validator val = trustedVals.getValidators().get(valIdx);

            // validate signature
            byte[] message = voteSignBytesDelim(commit, chainID, idx);
            byte[] sig = commitSig.getSignature();

            if (!verifySig(val, message, sig)) {
                return false;
            }

            talliedVotingPower = talliedVotingPower.add(val.getVotingPower());

            if (talliedVotingPower.compareTo(votingPowerNeeded) > 0) {
                return true;
            }

        }

        return false;
    }

    // VerifyCommitLight verifies +2/3 of the set had signed the given commit.
    //
    // This method is primarily used by the light client and does not check all the
    // signatures.
    protected boolean verifyCommitLight(
            ValidatorSet validators,
            String chainID,
            BlockID blockID,
            BigInteger height,
            Commit commit) {
        Context.require(validators.getValidators().size() == commit.getSignatures().size(),
                "invalid commmit signatures");

        Context.require(height.equals(commit.getHeight()), "invalid commit height");

        Context.require(commit.getBlockId().equals(blockID), "invalid commit -- wrong block ID");
        Validator val;
        CommitSig commitSig;

        BigInteger talliedVotingPower = BigInteger.ZERO;
        BigInteger votingPowerNeeded = validators.getTotalVotingPower().multiply(BigInteger.TWO)
                .divide(BigInteger.valueOf(3));

        int signaturesLength = commit.getSignatures().size();
        for (int i = 0; i < signaturesLength; i++) {
            commitSig = commit.getSignatures().get(i);

            // no need to verify absent or nil votes.
            if (commitSig.getBlockIdFlag() != BlockIDFlag.BLOCK_ID_FLAG_COMMIT) {
                continue;
            }

            val = validators.getValidators().get(i);

            byte[] message = voteSignBytesDelim(commit, chainID, i);
            byte[] sig = commitSig.getSignature();

            if (!verifySig(val, message, sig)) {
                return false;
            }

            talliedVotingPower = talliedVotingPower.add(val.getVotingPower());

            if (talliedVotingPower.compareTo(votingPowerNeeded) > 0) {
                return true;
            }
        }

        return false;
    }

    public boolean verifySig(
            Validator val,
            byte[] message,
            byte[] sig) {
        if (val.getPubKey().getEd25519() != null) {
            return verifySig("ed25519", message, sig, val.getPubKey().getEd25519());
        } else {
            return verifySig("ecdsa-secp256k1", message, sig, val.getPubKey().getSecp256k1());
        }

    }

    public boolean verifySig(
            String alg,
            byte[] message,
            byte[] sig,
            byte[] pubKey) {
        return Context.verifySignature(alg, message, sig, pubKey);
    }

    protected byte[] voteSignBytes(
            Commit commit,
            String chainID,
            int idx) {

        return toCanonicalVote(commit, idx, chainID).encode();
    }

    protected byte[] voteSignBytesDelim(
            Commit commit,
            String chainID,
            int idx) {
        return Proto.encodeDelim(voteSignBytes(commit, chainID, idx));
    }
}
