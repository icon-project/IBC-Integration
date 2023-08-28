package ibc.ics23.commitment.types;

import ibc.icon.score.util.StringUtil;
import ibc.ics23.commitment.Ics23;
import ibc.ics23.commitment.Proof;
import icon.proto.clients.tendermint.MerkleRoot;
import icon.proto.core.commitment.*;
import score.UserRevertedException;
import scorex.util.ArrayList;

import java.util.Arrays;
import java.util.List;

import static ibc.icon.score.util.StringUtil.bytesToHex;
import static ibc.ics23.commitment.Proof.*;

public class Merkle {
    public static List<ProofSpec> SDK_SPEC = List.of(
        Proof.getIavlSpec(),
        Proof.getTendermintSpec()
    );

    public static MerklePath applyPrefix(String path) {
        var mpath = new MerklePath();
        List<String> keyPath = new ArrayList<>();
        keyPath.add(StringUtil.bytesToHex("wasm".getBytes()));
        keyPath.add(path);
        mpath.setKeyPath(keyPath);
        return mpath;
    }

    /**
     * verifyMembership verifies the membership of a merkle proof against the given
     * root, path, and value. Note that the path is expected as String[]{<store key of module}, <key corresponding to
     * requested value>}.
     */
    public static void verifyMembership(MerkleProof proof, List<ProofSpec> specs, MerkleRoot root, MerklePath path,
                                        byte[] value) {
        validateVerificationArgs(proof, specs, root);

        // VerifyMembership specific argument validation
        if (path.getKeyPath().size() != specs.size()) {
            throw new UserRevertedException("Path length " + path.getKeyPath().size() + " not same as proof " + specs.size());
        }
        if (value.length == 0) {
            throw new UserRevertedException("Empty value in membership proof");
        }
        verifyChainedMembershipProof(root.getHash(), specs, proof.getProofs(), path, value, 0);
    }

    /**
     * verifyNonMembership verifies the absence of a merkle proof against the given root and path.
     * verifyNonMembership verifies a chained proof where the absence of a given path is proven at the lowest subtree
     * and then each subtree's inclusion is proved up to the final root.
     */
    public static void verifyNonMembership(MerkleProof proof, List<ProofSpec> specs, MerkleRoot root, MerklePath path) {
        validateVerificationArgs(proof, specs, root);

        // verifyNonMembership specific argument validation
        if (path.getKeyPath().size() != specs.size()) {
            throw new UserRevertedException("Path length " + path.getKeyPath().size() + " not same as proof " + specs.size());
        }

        var firstProof = proof.getProofs().get(0);
        if (!isNonExistenceProofEmpty(firstProof.getNonexist())) {
            // verifyNonMembership wil verify the absence of key in the lowest subtree, and then chain inclusion proofs
            // of all subroots up to final root
            var subroot = calculateRoot(firstProof.getNonexist());
            var key = getKey(path, path.getKeyPath().size() - 1);
            Ics23.verifyNonMembership(specs.get(0), subroot, firstProof, key);
            verifyChainedMembershipProof(root.getHash(), specs, proof.getProofs(), path, subroot, 1);
        } else if (!isExistenceProofEmpty(firstProof.getExist())) {
            throw new UserRevertedException("got existence proof in verifyNonMembership. If this is unexpected, please" +
                    "ensure that proof was queried with the correct key.");
        } else {
            throw new UserRevertedException("Expected proof type: NonExistenceProof, got: BatchProof or Compressed " +
                    "Batch Proof");
        }
    }

    /**
     * Initialize sub-root to value since the proofs list may be empty.
     * This may happen if this call is verifying intermediate proofs after the lowest proof has been executed.
     * In this case, there may be no intermediate proofs to verify, and we just check that lowest proof root equals
     * final root
     */
    private static void verifyChainedMembershipProof(byte[] root, List<ProofSpec> specs, List<CommitmentProof> proofs,
                                                     MerklePath keys, byte[] value, int index) {
        var subroot = value;
        for (int i = index; i < proofs.size(); i++) {
            var proof = proofs.get(i);

            if (!isExistenceProofEmpty(proof.getExist())) {
                subroot = calculateRoot(proof.getExist(), null);

                // Since keys are passed in from highest to lowest, we must grab their indices in reverse order from the
                // proofs and spec which are lowest to highest
                var key = getKey(keys, keys.getKeyPath().size() - 1 - i);
                Ics23.verifyMembership(specs.get(i), subroot, proofs.get(i), key, value);
                value = subroot;
            } else if (!isNonExistenceProofEmpty(proof.getNonexist())) {
                throw new UserRevertedException("Chained membership proof contains non-existence proof at index " + i +
                        ". If this is unexpected, please ensure that proof was queried from a height that contained the " +
                        "value in store and was queried with the correct key. The key used: " + keys);
            } else {
                throw new UserRevertedException("Expected proof type: Existence Proof, got: Batch proof or Compressed " +
                        "Batch proof");
            }
        }

        if (!Arrays.equals(root, subroot)) {
            throw new UserRevertedException("proof did not commit to expected root: " + bytesToHex(root) + ", got: " +
                    bytesToHex(subroot) + ". Please ensure proof was submitted with correct proof height and to the " +
                    "correct chain.");
        }
    }

    /**
     * getKey() will return a byte representation of the key after URl escaping the key element
     *
     * @return byte representation of key
     */
    private static byte[] getKey(MerklePath mp, int i) {
        if (i >= mp.getKeyPath().size()) {
            throw new UserRevertedException("index out of range. " + i + " (index) >= " + mp.getKeyPath().size() + " (len)");
        }
        String key = mp.getKeyPath().get(i);
        return StringUtil.hexToBytes(key);
    }

    private static void validateVerificationArgs(MerkleProof proof, List<ProofSpec> specs, MerkleRoot root) {
        if (isMerkleProofEmpty(proof)) {
            throw new UserRevertedException("Proof cannot be empty");
        }

        if (isMerkleRootEmpty(root)) {
            throw new UserRevertedException("Root cannot be empty");
        }

        if (specs.size() != proof.getProofs().size()) {
            throw new UserRevertedException("InvalidMerkleProof: length of specs: " +
                    specs.size() + " not equal to length of proof: " + proof.getProofs().size());
        }

        for (int i = 0; i < specs.size(); i++) {
            var spec = specs.get(i);
            if (isProofSpecEmpty(spec)) {
                throw new UserRevertedException("spec at position " + i + " is nil");
            }
        }
    }

    public static boolean isMerkleProofEmpty(MerkleProof proof) {
        if (proof == null) {
            return true;
        }

        return proof.getProofs().isEmpty();
    }

    private static boolean isMerkleRootEmpty(MerkleRoot root) {
        if (root == null) {
            return true;
        }

        return root.getHash().length == 0;
    }

    private static boolean isProofSpecEmpty(ProofSpec spec) {
        if (spec == null) {
            return true;
        }
        if (!isLeafOpEmpty(spec.getLeafSpec())) {
            return false;
        }
        if (!isInnerSpecEmpty(spec.getInnerSpec())) {
            return false;
        }
        return true;
    }

    public static byte[] prefixLengthInBigEndian(byte[] input) {
        // calculate the length of the input array
        int length = input.length;

        // manually convert the length to a 2-byte array in big endian format
        byte[] lengthPrefix = new byte[2];
        lengthPrefix[0] = (byte) ((length >> 8) & 0xFF);
        lengthPrefix[1] = (byte) (length & 0xFF);

        // prefix the length to the input array
        byte[] result = new byte[lengthPrefix.length + input.length];

        System.arraycopy(lengthPrefix, 0, result, 0, lengthPrefix.length);
        System.arraycopy(input, 0, result, lengthPrefix.length, input.length);

        return result;
    }

    private static boolean isInnerSpecEmpty(InnerSpec spec) {
        return spec == null;
    }
}
