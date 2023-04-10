package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.UserRevertedException;

import java.util.Arrays;
import java.util.List;

import static ibc.ics23.commitment.Proof.*;

public class Ics23 {

    public static void verifyMembership(ProofSpec spec, byte[] commitmentRoot, CommitmentProof proof
            , byte[] key, byte[] value) {
        var decoProof = Compress.decompress(proof);
        var exiProof = getExistProofForKey(decoProof, key);
        if (exiProof == null) {
            throw new UserRevertedException("getExistProofForKey not available");
        }
        Proof.verify(exiProof, spec, commitmentRoot, key, value);
    }

    public static void verifyNonMembership(ProofSpec spec, byte[] commitmentRoot, CommitmentProof proof, byte[] key) {
        var decoProof = Compress.decompress(proof);
        var nonProof = getNonExistProofForKey(decoProof, key);
        if (nonProof == null) {
            throw new UserRevertedException("getNonExistProofForKey not available");
        }
        Proof.verify(nonProof, spec, commitmentRoot, key);
    }

    private static ExistenceProof getExistProofForKey(CommitmentProof proof, byte[] key) {
        if (!isExistenceProofEmpty(proof.getExist())) {
            if (Arrays.equals(proof.getExist().getKey(), key)) {
                return proof.getExist();
            }
        } else if (!isBatchProofEmpty(proof.getBatch())) {
            List<BatchEntry> proofBatchEntries = proof.getBatch().getEntries();
            for (BatchEntry proofBatchEntry : proofBatchEntries) {
                ExistenceProof existenceProof = proofBatchEntry.getExist();
                if (existenceProof != null && Arrays.equals(existenceProof.getKey(), key)) {
                    return existenceProof;
                }
            }
        }
        return null;
    }

    private static NonExistenceProof getNonExistProofForKey(CommitmentProof proof, byte[] key) {
        NonExistenceProof nonExistenceProof = proof.getNonexist();
        if (!isNonExistenceProofEmpty(nonExistenceProof)) {
            if (isLeft(nonExistenceProof.getLeft(), key) && isRight(nonExistenceProof.getRight(), key)) {
                return nonExistenceProof;
            }
        } else if (!isBatchProofEmpty(proof.getBatch())) {
            List<BatchEntry> proofBatchEntries = proof.getBatch().getEntries();
            for (BatchEntry proofBatchEntry : proofBatchEntries) {
                NonExistenceProof batchNonExistenceProof = proofBatchEntry.getNonexist();
                if (!isNonExistenceProofEmpty(batchNonExistenceProof)
                        && isLeft(batchNonExistenceProof.getLeft(), key)
                        && isRight(batchNonExistenceProof.getRight(), key)) {
                    return batchNonExistenceProof;
                }
            }
        }
        return null;
    }

    private static boolean isLeft(ExistenceProof left, byte[] key) {
        return isExistenceProofEmpty(left) || Ops.compare(left.getKey(), key) < 0;
    }

    private static boolean isRight(ExistenceProof right, byte[] key) {
        return isExistenceProofEmpty(right) || Ops.compare(right.getKey(), key) > 0;
    }
}
