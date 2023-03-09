package ibc.tendermint;

import ibc.icon.score.util.Proto;
import ibc.icon.structs.proto.lightclient.tendermint.*;
import score.Context;

import java.math.BigInteger;
import java.util.Arrays;

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
        if (!untrustedHeader.header.height.equals(trustedHeader.header.height.add(BigInteger.ONE))) {
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
        Context.require(untrustedHeader.header.height.equals(trustedHeader.header.height.add(BigInteger.ONE)),
                "headers must be adjacent in height");

        Context.require(!trustedHeader.isExpired(trustingPeriod, currentTime), "header can't be expired");

        verifyNewHeaderAndVals(untrustedHeader, untrustedVals, trustedHeader, currentTime, maxClockDrift);

        // Check the validator hashes are the same
        Context.require(
                Arrays.equals(untrustedHeader.header.validatorsHash, trustedHeader.header.nextValidatorsHash),
                "expected old header next validators to match those from new header");

        // Ensure that +2/3 of new validators signed correctly.
        return verifyCommitLight(
                untrustedVals,
                trustedHeader.header.chainId,
                untrustedHeader.commit.blockId,
                untrustedHeader.header.height,
                untrustedHeader.commit);

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
                !untrustedHeader.header.height.equals(trustedHeader.header.height.add(BigInteger.ONE)),
                "LC: headers must be non adjacent in height");

        // assert that trustedVals is NextValidators of last trusted header
        // to do this, we check that trustedVals.Hash() == consState.NextValidatorsHash
        Context.require(Arrays.equals(trustedVals.hash(), trustedHeader.header.nextValidatorsHash),
                "LC: headers trusted validators does not hash to latest trusted validators");

        Context.require(!trustedHeader.isExpired(trustingPeriod, currentTime), "header can't be expired");

        verifyNewHeaderAndVals(untrustedHeader, untrustedVals, trustedHeader, currentTime, maxClockDrift);

        // Ensure that +`trustLevel` (default 1/3) or more of last trusted validators
        // signed correctly.
        verifyCommitLightTrusting(trustedVals, trustedHeader.header.chainId, untrustedHeader.commit, trustLevel);

        // Ensure that +2/3 of new validators signed correctly.
        return verifyCommitLight(
                untrustedVals,
                trustedHeader.header.chainId,
                untrustedHeader.commit.blockId,
                untrustedHeader.header.height,
                untrustedHeader.commit);

    }

    protected void verifyNewHeaderAndVals(
            SignedHeader untrustedHeader,
            ValidatorSet untrustedVals,
            SignedHeader trustedHeader,
            Duration currentTime,
            Duration maxClockDrift) {
        // SignedHeader validate basic
        Context.require(untrustedHeader.header.chainId.equals(trustedHeader.header.chainId),
                "header belongs to another chain");
        Context.require(untrustedHeader.commit.height.equals(untrustedHeader.header.height),
                "header and commit height mismatch");

        byte[] untrustedHeaderBlockHash = untrustedHeader.header.hash();
        Context.require(Arrays.equals(untrustedHeaderBlockHash, untrustedHeader.commit.blockId.hash),
                "commit signs signs block failed");

        Context.require(untrustedHeader.header.height.compareTo(trustedHeader.header.height) > 0,
                "expected new header height to be greater than one of old header");
        Context.require(
                untrustedHeader.header.time.gt(trustedHeader.header.time),
                "expected new header time to be after old header time");

        Timestamp curentTimestamp = new Timestamp(currentTime.seconds.add(maxClockDrift.seconds),
                currentTime.nanos.add(maxClockDrift.nanos));
        Context.require(curentTimestamp.gt(untrustedHeader.header.time),
                "new header has time from the future");

        byte[] validatorsHash = untrustedVals.hash();
        Context.require(Arrays.equals(untrustedHeader.header.validatorsHash, validatorsHash),
                "expected new header validators to match those that were supplied at height XX");
    }

    protected boolean verifyCommitLightTrusting(
            ValidatorSet trustedVals,
            String chainID,
            Commit commit,
            Fraction trustLevel) {
        // sanity check
        Context.require(!trustLevel.denominator.equals(BigInteger.ZERO), "trustLevel has zero Denominator");

        BigInteger talliedVotingPower = BigInteger.ZERO;
        boolean[] seenVals = new boolean[trustedVals.validators.length];

        CommitSig commitSig;
        BigInteger totalVotingPowerMulByNumerator = trustedVals.getTotalVotingPower().multiply(trustLevel.numerator);
        BigInteger votingPowerNeeded = totalVotingPowerMulByNumerator.divide(trustLevel.denominator);

        for (int idx = 0; idx < commit.signatures.length; idx++) {
            commitSig = commit.signatures[idx];

            // no need to verify absent or nil votes.
            if (commitSig.blockIdFlag != BlockIDFlag.BLOCK_ID_FLAG_COMMIT) {
                continue;
            }

            // We don't know the validators that committed this block, so we have to
            // check for each vote if its validator is already known.
            int valIdx = trustedVals.getByAddress(commitSig.validatorAddress);
            if (valIdx == -1) {
                continue;
            }

            // check for double vote of validator on the same commit
            Context.require(!seenVals[valIdx], "double vote of validator on the same commit");
            seenVals[valIdx] = true;

            Validator val = trustedVals.validators[valIdx];

            // validate signature
            byte[] message = voteSignBytesDelim(commit, chainID, idx);
            byte[] sig = commitSig.signature;

            if (!verifySig(val, message, sig)) {
                return false;
            }

            talliedVotingPower = talliedVotingPower.add(val.votingPower);

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
        Context.require(validators.validators.length == commit.signatures.length, "invalid commmit signatures");

        Context.require(height.equals(commit.height), "invalid commit height");

        Context.require(commit.blockId.equals(blockID), "invalid commit -- wrong block ID");
        Validator val;
        CommitSig commitSig;

        BigInteger talliedVotingPower = BigInteger.ZERO;
        BigInteger votingPowerNeeded = validators.getTotalVotingPower().multiply(BigInteger.TWO)
                .divide(BigInteger.valueOf(3));

        for (int i = 0; i < commit.signatures.length; i++) {
            commitSig = commit.signatures[i];

            // no need to verify absent or nil votes.
            if (commitSig.blockIdFlag != BlockIDFlag.BLOCK_ID_FLAG_COMMIT) {
                continue;
            }

            val = validators.validators[i];

            byte[] message = voteSignBytesDelim(commit, chainID, i);
            byte[] sig = commitSig.signature;

            if (!verifySig(val, message, sig)) {
                return false;
            }

            talliedVotingPower = talliedVotingPower.add(val.votingPower);

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
        if (val.pubKey.ed25519 != null) {
            return verifySig("ed25519", message, sig, val.pubKey.ed25519);
        } else {
            return verifySig("ecdsa-secp256k1", message, sig, val.pubKey.secp256k1);
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

        return commit.toCanonicalVote(idx, chainID).encode();
    }

    protected byte[] voteSignBytesDelim(
            Commit commit,
            String chainID,
            int idx) {
        return Proto.encodeDelim(voteSignBytes(commit, chainID, idx));
    }
}
