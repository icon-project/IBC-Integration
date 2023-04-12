package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.UserRevertedException;
import scorex.util.ArrayList;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

import static ibc.ics23.commitment.Compress.isCompressedBatchProofEmpty;

public class Proof {

    public static void verify(ExistenceProof proof, ProofSpec spec, byte[] commitmentRoot, byte[] key, byte[] value) {
        if (!Arrays.equals(proof.getKey(), key)) {
            throw new UserRevertedException("Provided key doesn't match proof");
        }
        if (!Arrays.equals(proof.getValue(), value)) {
            throw new UserRevertedException("Provided value doesn't match proof");
        }
        checkAgainstSpec(proof, spec);
        byte[] root = calculateRoot(proof);
        boolean rootMatch = Arrays.equals(root, commitmentRoot);
        if (!rootMatch) {
            throw new UserRevertedException("Calculated Root doesn't match provided root");
        }
    }

    public static byte[] calculateRoot(ExistenceProof proof) {
        if (isLeafOpEmpty(proof.getLeaf())) {
            throw new UserRevertedException("Existence Proof needs defined LeafOp");
        }
        byte[] root = Ops.applyOp(proof.getLeaf(), proof.getKey(), proof.getValue());
        List<InnerOp> proofPath = proof.getPath();
        for (InnerOp innerOp : proofPath) {
            root = Ops.applyOp(innerOp, root);
        }
        return root;
    }

    public static void checkAgainstSpec(ExistenceProof proof, ProofSpec spec) {
        if (isLeafOpEmpty(proof.getLeaf())) {
            throw new UserRevertedException("Existence Proof needs defined LeafOp");
        }
        Ops.checkAgainstSpec(proof.getLeaf(), spec);
        int proofPathSize = proof.getPath().size();
        if (spec.getMinDepth().compareTo(BigInteger.ZERO) > 0) {
            boolean innerOpsDepthTooShort =
                    BigInteger.valueOf(proofPathSize).compareTo(spec.getMinDepth()) < 0;
            if (innerOpsDepthTooShort) {
                throw new UserRevertedException("InnerOps depth too short");
            }
        }

        if (spec.getMaxDepth().compareTo(BigInteger.ZERO) > 0) {
            boolean innerOpsDepthTooLong = BigInteger.valueOf(proofPathSize).compareTo(spec.getMaxDepth()) > 0;
            if (innerOpsDepthTooLong) {
                throw new UserRevertedException("InnerOps depth too long");
            }
        }

        for (int i = 0; i < proofPathSize; i++) {
            Ops.checkAgainstSpec(proof.getPath().get(i), spec);
        }
    }

    public static void verify(NonExistenceProof proof, ProofSpec spec, byte[] commitmentRoot, byte[] key) {
        byte[] leftKey = new byte[0];
        byte[] rightKey = new byte[0];

        ExistenceProof proofLeft = proof.getLeft();
        if (!isExistenceProofEmpty(proofLeft)) {
            byte[] proofLeftKey = proofLeft.getKey();
            verify(proofLeft, spec, commitmentRoot, proofLeftKey, proofLeft.getValue());
            leftKey = proofLeftKey;
        }

        ExistenceProof proofRight = proof.getRight();
        if (!isExistenceProofEmpty(proofRight)) {
            byte[] proofRightKey = proofRight.getKey();
            verify(proofRight, spec, commitmentRoot, proofRightKey, proofRight.getValue());
            rightKey = proofRightKey;
        }

        if (leftKey.length == 0 && rightKey.length == 0) {
            throw new UserRevertedException("Both left and right proofs missing");
        }

        if (rightKey.length > 0 && Ops.compare(key, rightKey) >= 0) {
            throw new UserRevertedException("Key is not left of right proof");
        }

        if (leftKey.length > 0 && Ops.compare(key, leftKey) <= 0) {
            throw new UserRevertedException("Key is not right of left proof");
        }

        if (leftKey.length == 0) {
            if (!isLeftMost(spec.getInnerSpec(), proof.getRight().getPath())) {
                throw new UserRevertedException("Left proof missing, right proof must be left-most");
            }
        } else if (rightKey.length == 0) {
            if (!isRightMost(spec.getInnerSpec(), proof.getLeft().getPath())) {
                throw new UserRevertedException("isRightMost: right proof missing, left proof must be right-most");
            }
        } else {
            boolean isLeftNeigh = isLeftNeighbour(spec.getInnerSpec(), proof.getLeft().getPath(),
                    proof.getRight().getPath());
            if (!isLeftNeigh) {
                throw new UserRevertedException("isLeftNeighbour: right proof missing, left proof must be right-most");
            }
        }
    }

    public static byte[] calculateRoot(NonExistenceProof proof) {
        if (!isExistenceProofEmpty(proof.getLeft())) {
            return calculateRoot(proof.getLeft());
        }
        if (!isExistenceProofEmpty(proof.getRight())) {
            return calculateRoot(proof.getRight());
        }
        throw new UserRevertedException("Nonexistence proof has empty Left and Right Proof");
    }

    public static byte[] calculateRoot(CommitmentProof proof) {
        if (!isExistenceProofEmpty(proof.getExist())) {
            return calculateRoot(proof.getExist());
        }
        if (!isNonExistenceProofEmpty(proof.getNonexist())) {
            return calculateRoot(proof.getNonexist());
        }

        if (!isBatchProofEmpty(proof.getBatch())) {
            if (proof.getBatch().getEntries().size() == 0) {
                throw new UserRevertedException("Batch Proof has no entry");
            }
            if (isBatchEntryEmpty(proof.getBatch().getEntries().get(0))) {
                throw new UserRevertedException("Batch proof has empty entry");
            }
            if (!isExistenceProofEmpty(proof.getBatch().getEntries().get(0).getExist())) {
                return calculateRoot(proof.getBatch().getEntries().get(0).getExist());
            }
            if (!isNonExistenceProofEmpty(proof.getBatch().getEntries().get(0).getNonexist())) {
                return calculateRoot(proof.getBatch().getEntries().get(0).getNonexist());
            }
        }
        if (!isCompressedBatchProofEmpty(proof.getCompressed())) {
            return calculateRoot(Compress.decompress(proof));
        }
        throw new UserRevertedException("Empty proof");
    }

    private static boolean isLeftMost(InnerSpec spec, List<InnerOp> path) {
        BigInteger[] padding = getPadding(spec, 0);
        BigInteger minPrefix = padding[0];
        BigInteger maxPrefix = padding[1];
        BigInteger suffix = padding[2];

        for (InnerOp innerOp : path) {
            if (!hasPadding(innerOp, minPrefix, maxPrefix, suffix)) {
                return false;
            }
        }
        return true;
    }

    private static boolean isRightMost(InnerSpec spec, List<InnerOp> path) {
        int last = spec.getChildOrder().size() - 1;
        BigInteger[] padding = getPadding(spec, last);
        BigInteger minPrefix = padding[0];
        BigInteger maxPrefix = padding[1];
        BigInteger suffix = padding[2];

        for (InnerOp innerOp : path) {
            if (!hasPadding(innerOp, minPrefix, maxPrefix, suffix)) {
                return false;
            }
        }
        return true;
    }

    private static boolean isLeftStep(InnerSpec spec, InnerOp left, InnerOp right) {
        int leftIdx = orderFromPadding(spec, left);
        int rightIdx = orderFromPadding(spec, right);
        return rightIdx == leftIdx + 1;
    }

    private static boolean isLeftNeighbour(InnerSpec spec, List<InnerOp> left, List<InnerOp> right) {
        int leftIdx = left.size() - 1;
        int rightIdx = right.size() - 1;
        while (leftIdx >= 0 && rightIdx >= 0) {
            InnerOp leftInnerOp = left.get(leftIdx);
            InnerOp rightInnerOp = right.get(rightIdx);
            if (Arrays.equals(leftInnerOp.getPrefix(), rightInnerOp.getPrefix()) &&
                    Arrays.equals(leftInnerOp.getSuffix(), rightInnerOp.getSuffix())) {
                leftIdx = leftIdx - 1;
                rightIdx = rightIdx - 1;
                continue;
            }
            break;
        }

        if (!isLeftStep(spec, left.get(leftIdx), right.get(rightIdx))) {
            return false;
        }

        if (!isRightMost(spec, sliceInnerOps(left, 0, leftIdx))) {
            return false;
        }

        if (!isLeftMost(spec, sliceInnerOps(right, 0, rightIdx))) {
            return false;
        }
        return true;
    }

    private static int orderFromPadding(InnerSpec spec, InnerOp op) {
        int maxBranch = spec.getChildOrder().size();
        for (int branch = 0; branch < maxBranch; branch++) {
            BigInteger[] padding = getPadding(spec, branch);
            BigInteger minPrefix = padding[0];
            BigInteger maxPrefix = padding[1];
            BigInteger suffix = padding[2];

            if (hasPadding(op, minPrefix, maxPrefix, suffix)) {
                return branch;
            }
        }
        throw new UserRevertedException("Cannot find any valid spacing for this node");
    }

    private static BigInteger[] getPadding(InnerSpec spec, int branch) {
        BigInteger childSize = spec.getChildSize();
        int idx = getPosition(spec.getChildOrder(), branch);
        BigInteger prefix = BigInteger.valueOf(idx).multiply(childSize);
        BigInteger minPrefix = prefix.add(spec.getMinPrefixLength());
        BigInteger maxPrefix = prefix.add(spec.getMaxPrefixLength());
        BigInteger suffix = BigInteger.valueOf(spec.getChildOrder().size() - 1 - idx).multiply(childSize);

        return new BigInteger[]{minPrefix, maxPrefix, suffix};
    }

    private static int getPosition(List<BigInteger> order, int branch) {
        int orderLength = order.size();
        if (branch >= orderLength) {
            throw new UserRevertedException("Invalid branch");
        }

        for (int i = 0; i < orderLength; i++) {
            if (order.get(i).equals(BigInteger.valueOf(branch))) {
                return i;
            }
        }
        return 0;
    }

    private static boolean hasPadding(InnerOp op, BigInteger minPrefix, BigInteger maxPrefix, BigInteger suffix) {
        int opPrefixLength = op.getPrefix().length;
        if (opPrefixLength < minPrefix.intValue()) return false;
        if (opPrefixLength > maxPrefix.intValue()) return false;
        return op.getSuffix().length == suffix.intValue();
    }

    private static List<InnerOp> sliceInnerOps(List<InnerOp> array, int start, int end) {
        List<InnerOp> slice = new ArrayList<>(end - start);
        for (int i = start; i < end; i++) {
            slice.set(i, array.get(i));
        }
        return slice;
    }

    public static boolean isLeafOpEmpty(LeafOp leafOp) {
        if (leafOp == null) {
            return true;
        }
        if (leafOp.getHash() != 0) {
            return false;
        }
        if (leafOp.getPrehashKey() != 0) {
            return false;
        }
        if (leafOp.getPrehashValue() != 0) {
            return false;
        }
        if (leafOp.getLength() != 0) {
            return false;
        }
        if (leafOp.getPrefix().length != 0) {
            return false;
        }
        return true;
    }

    public static boolean isExistenceProofEmpty(ExistenceProof existenceProof) {
        if (existenceProof == null) {
            return true;
        }
        return isProofEmpty(existenceProof.getKey(), existenceProof.getValue(), existenceProof.getPath().size());
    }

    public static boolean isNonExistenceProofEmpty(NonExistenceProof nonExistenceProof) {
        if (nonExistenceProof == null) {
            return true;
        }
        if (nonExistenceProof.getKey().length != 0) {
            return false;
        }
        return true;
    }

    public static boolean isProofEmpty(byte[] key, byte[] value, int size) {
        if (key.length != 0) {
            return false;
        }
        if (value.length != 0) {
            return false;
        }
        if (size != 0) {
            return false;
        }
        return true;
    }

    public static boolean isBatchProofEmpty(BatchProof batchProof) {
        if (batchProof == null) {
            return true;
        }
        if (batchProof.getEntries().size() != 0) {
            return false;
        }
        return true;
    }

    public static boolean isBatchEntryEmpty(BatchEntry batchEntry) {
        if (batchEntry == null) {
            return true;
        }
        if (!isExistenceProofEmpty(batchEntry.getExist())) {
            return false;
        }
        if (!isNonExistenceProofEmpty(batchEntry.getNonexist())) {
            return false;
        }
        return true;
    }
}
