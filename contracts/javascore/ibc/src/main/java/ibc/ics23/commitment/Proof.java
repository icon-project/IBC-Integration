package ibc.ics23.commitment;

import icon.proto.core.commitment.*;
import score.UserRevertedException;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

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
}
