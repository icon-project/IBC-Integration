package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.Context;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.List;

public class Compress {
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
            entries.set(i, decompressEntry(proof.getEntries().get(i), proof.getLookupInners()));
        }
        return entries;
    }

    public static BatchEntry decompressEntry(CompressedBatchEntry entry, List<InnerOp> lookup) {
        var batchEntry = new BatchEntry();
        if (!isCompressedExistenceProofEmpty(entry.getExist())) {
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
            path.set(i, lookup.get(step.intValue()));
        }
        decoProof.setPath(path);
        return decoProof;
    }

    public static boolean isCompressedBatchProofEmpty(CompressedBatchProof proof) {
        if (proof != null && proof.getEntries().size() != 0) {
            return false;
        }

        if (proof != null && proof.getLookupInners().size() != 0) {
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
