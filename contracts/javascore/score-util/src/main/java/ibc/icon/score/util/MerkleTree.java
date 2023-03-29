package ibc.icon.score.util;

import score.Context;

public class MerkleTree {
    /**
     * returns empty hash
     */
    public static byte[] emptyHash() {
        return Context.hash("sha-256", new byte[0]);
    }

    /**
     * returns tmhash(0x00 || leaf)
     */
    public static byte[] leafHash(byte[] leaf) {
        byte leafPrefix = 0x00;
        byte[] packed = new byte[1 + leaf.length];
        packed[0] = leafPrefix;
        System.arraycopy(leaf, 0, packed, 1, leaf.length);
        return Context.hash("sha-256", packed);
    }

    /**
     * returns tmhash(0x01 || left || right)
     */
    public static byte[] innerHash(byte[] leaf, byte[] right) {
        byte innerPrefix = 0x01;
        byte[] packed = new byte[1 + leaf.length + right.length];
        packed[0] = innerPrefix;
        System.arraycopy(leaf, 0, packed, 1, leaf.length);
        System.arraycopy(right, 0, packed, 1 + leaf.length, right.length);
        return Context.hash("sha-256", packed);
    }

    /**
     * returns the largest power of 2 less than length
     * <p>
     *      TODO: This public static byte[] can be optimized with bit shifting
     *      approach:
     *      https://www.baeldung.com/java-largest-power-of-2-less-than-number
     */
    public static int getSplitPoint(int input) {
        // require(input > 1, "MerkleTree: invalid input");

        int result = 1;
        for (int i = input - 1; i > 1; i--) {
            if ((i & (i - 1)) == 0) {
                result = i;
                break;
            }
        }
        return result;
    }

    /**
     * computes a Merkle tree where the leaves are the byte slice in the provided order
     * Follows RFC-6962
     */
    public static byte[] merkleRootHash(byte[][] data, int start, int total) {
        if (total == 0) {
            return emptyHash();
        } else if (total == 1) {
            return leafHash(data[start]);
        } else {
            int k = getSplitPoint(total);
            byte[] left = merkleRootHash(data, start, k); // data[:k]
            byte[] right = merkleRootHash(data, start + k, total - k); // data[k:]
            return innerHash(left, right);
        }
    }
}
