package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.UserRevertedException;
import scorex.util.ArrayList;

import java.util.Arrays;
import java.util.List;

import static ibc.ics23.commitment.Compress.*;
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
        var nonProof = getNonExistProofForKey(spec, decoProof, key);
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
                if (!isExistenceProofEmpty(existenceProof) && Arrays.equals(existenceProof.getKey(), key)) {
                    return existenceProof;
                }
            }
        }
        return null;
    }

    private static NonExistenceProof getNonExistProofForKey(ProofSpec spec, CommitmentProof proof, byte[] key) {
        NonExistenceProof nonExistenceProof = proof.getNonexist();
        if (!isNonExistenceProofEmpty(nonExistenceProof)) {
            if (isLeft(spec, nonExistenceProof.getLeft(), key) && isRight(spec, nonExistenceProof.getRight(), key)) {
                return nonExistenceProof;
            }
        } else if (!isBatchProofEmpty(proof.getBatch())) {
            List<BatchEntry> proofBatchEntries = proof.getBatch().getEntries();
            for (BatchEntry proofBatchEntry : proofBatchEntries) {
                NonExistenceProof batchNonExistenceProof = proofBatchEntry.getNonexist();
                if (!isNonExistenceProofEmpty(batchNonExistenceProof)
                        && isLeft(spec, batchNonExistenceProof.getLeft(), key)
                        && isRight(spec, batchNonExistenceProof.getRight(), key)) {
                    return batchNonExistenceProof;
                }
            }
        }
        return null;
    }

    private static boolean isLeft(ProofSpec spec, ExistenceProof left, byte[] key) {
        return isExistenceProofEmpty(left) || Ops.compare(keyForComparison(spec, left.getKey()), keyForComparison(spec, key)) < 0;
    }

    private static boolean isRight(ProofSpec spec, ExistenceProof right, byte[] key) {
        return isExistenceProofEmpty(right) || Ops.compare(keyForComparison(spec, right.getKey()), keyForComparison(spec, key)) > 0;
    }

    public static CommitmentProof combineProofs(List<CommitmentProof> proofs) {
        List<BatchEntry> entries = new ArrayList<>();

        for (CommitmentProof proof : proofs) {
            var exist = proof.getExist();
            var nonExist = proof.getNonexist();
            var batch = proof.getBatch();
            var comp = proof.getCompressed();

            if (!isExistenceProofEmpty(exist)) {
                var entry = new BatchEntry();
                entry.setExist(exist);
                entries.add(entry);
            } else if (!isNonExistenceProofEmpty(nonExist)) {
                var entry = new BatchEntry();
                entry.setNonexist(nonExist);
                entries.add(entry);
            } else if (!isBatchProofEmpty(batch)) {
                entries.addAll(batch.getEntries());
            } else if (!isCompressedBatchProofEmpty(comp)) {
                var decompressedProof = decompress(proof);
                entries.addAll(decompressedProof.getBatch().getEntries());
            } else {
                throw new UserRevertedException("Proof neither exist or non-exist");
            }
        }
        var batch = new CommitmentProof();
        var batchProof = new BatchProof();
        batchProof.setEntries(entries);
        batch.setBatch(batchProof);
        return compress(batch);
    }
}
