package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.Context;
import scorex.util.ArrayList;
import scorex.util.HashMap;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

import static ibc.ics23.commitment.Proof.isBatchProofEmpty;
import static ibc.ics23.commitment.Proof.isExistenceProofEmpty;

public class Compress {

    public static CommitmentProof compress(CommitmentProof proof) {
        var batch = proof.getBatch();
        if (isBatchProofEmpty(batch)) {
            return proof;
        }
        var commitmentProof = new CommitmentProof();
        var operation = new CompressOperations();
        var compressedBatchProof = operation.compress(batch);
        commitmentProof.setCompressed(compressedBatchProof);
        return commitmentProof;
    }

    private static class CompressOperations {
        List<InnerOp> lookup = new ArrayList<>();
        Map<String, BigInteger> registry = new HashMap<>();

        public CompressedBatchProof compress(BatchProof batch) {
            List<CompressedBatchEntry> compressedEntries = new ArrayList<>();

            for (BatchEntry entry : batch.getEntries()) {
                var compressedEntry = compressEntry(entry);
                compressedEntries.add(compressedEntry);
            }
            var compressedBatchProof = new CompressedBatchProof();
            compressedBatchProof.setEntries(compressedEntries);
            compressedBatchProof.setLookupInners(lookup);
            return compressedBatchProof;
        }

        private CompressedBatchEntry compressEntry(BatchEntry entry) {
            var exist = entry.getExist();
            if (!isExistenceProofEmpty(exist)) {
                var compressedExistenceProof = compressExist(exist);
                var compressedBatchEntry = new CompressedBatchEntry();
                compressedBatchEntry.setExist(compressedExistenceProof);
                return compressedBatchEntry;
            }

            var nonExist = entry.getNonexist();
            var compressedNonExist = new CompressedNonExistenceProof();
            compressedNonExist.setKey(nonExist.getKey());
            compressedNonExist.setLeft(compressExist(nonExist.getLeft()));
            compressedNonExist.setRight(compressExist(nonExist.getRight()));
            var compressedBatchEntry = new CompressedBatchEntry();
            compressedBatchEntry.setNonexist(compressedNonExist);
            return compressedBatchEntry;
        }

        private CompressedExistenceProof compressExist(ExistenceProof exist) {
            if (isExistenceProofEmpty(exist)) {
                return null;
            }

            var res = new CompressedExistenceProof();
            res.setKey(exist.getKey());
            res.setValue(exist.getValue());
            res.setLeaf(exist.getLeaf());

            List<InnerOp> existenceProofPath = exist.getPath();
            int pathSize = existenceProofPath.size();
            List<BigInteger> path = new ArrayList<>();
            for (int i = 0; i < pathSize; i++) {
                var step = existenceProofPath.get(i);
                path.add(i, compressStep(step));
            }
            res.setPath(path);
            return res;
        }

        private BigInteger compressStep(InnerOp step) {
            var bz = step.encode();
            var sig = new String(bz);

            var num = registry.get(sig);
            if (num != null) {
                return num;
            }

            num = BigInteger.valueOf(lookup.size());
            lookup.add(step);
            registry.put(sig, num);
            return num;
        }
    }

    public static CommitmentProof decompress(CommitmentProof proof) {
        var compressedBatchProof = proof.getCompressed();
        if (isCompressedBatchProofEmpty(compressedBatchProof)) {
            return proof;
        }

        var batchProof = new BatchProof();
        batchProof.setEntries(decompress(compressedBatchProof));

        var commitmentProof = new CommitmentProof();
        commitmentProof.setBatch(batchProof);
        return commitmentProof;
    }

    public static List<BatchEntry> decompress(CompressedBatchProof proof) {
        int proofEntriesSize = proof.getEntries().size();
        List<BatchEntry> entries = new ArrayList<>(proofEntriesSize);
        for (int i = 0; i < proofEntriesSize; i++) {
            entries.add(i, decompressEntry(proof.getEntries().get(i), proof.getLookupInners()));
        }
        return entries;
    }

    public static BatchEntry decompressEntry(CompressedBatchEntry entry, List<InnerOp> lookup) {
        var batchEntry = new BatchEntry();
        if (!isCompressedExistenceProofEmpty(entry.getExist())) {
            batchEntry.setExist(decompressExist(entry.getExist(), lookup));
            return batchEntry;
        }

        var nonExist = new NonExistenceProof();
        nonExist.setKey(entry.getNonexist().getKey());
        nonExist.setLeft(decompressExist(entry.getNonexist().getLeft(), lookup));
        nonExist.setRight(decompressExist(entry.getNonexist().getRight(), lookup));

        batchEntry.setNonexist(nonExist);
        return batchEntry;
    }

    public static ExistenceProof decompressExist(CompressedExistenceProof proof, List<InnerOp> lookup) {
        var decoProof = new ExistenceProof();
        if (isCompressedExistenceProofEmpty(proof)) {
            return decoProof;
        }
        decoProof.setKey(proof.getKey());
        decoProof.setValue(proof.getValue());
        decoProof.setLeaf(proof.getLeaf());

        int proofPathLength = proof.getPath().size();
        List<InnerOp> path = new ArrayList<>(proofPathLength);
        for (int i = 0; i < proofPathLength; i++) {
            BigInteger step = proof.getPath().get(i);
            Context.require(step.compareTo(BigInteger.ZERO) >= 0);
            Context.require(step.compareTo(BigInteger.valueOf(lookup.size())) < 0);
            path.add(i, lookup.get(step.intValue()));
        }
        decoProof.setPath(path);
        return decoProof;
    }

    public static boolean isCompressedBatchProofEmpty(CompressedBatchProof proof) {
        if (proof != null && !proof.getEntries().isEmpty()) {
            return false;
        }

        if (proof != null && !proof.getLookupInners().isEmpty()) {
            return false;
        }
        return true;
    }

    public static boolean isCompressedExistenceProofEmpty(CompressedExistenceProof proof) {
        if (proof == null) {
            return true;
        }
        return Proof.isProofEmpty(proof.getKey(), proof.getValue(), proof.getPath().size());
    }
}
