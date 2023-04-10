package ibc.ics23.commitment;

import ibc.icon.score.util.ByteUtil;
import icon.proto.core.commitment.*;
import score.Context;
import score.UserRevertedException;

import java.math.BigInteger;
import java.util.Arrays;

public class Ops {

    public static final byte[] empty = new byte[0];

    //LeafOp operations
    public static byte[] applyOp(LeafOp leafOp, byte[] key, byte[] value) {
        if (key.length == 0) {
            throw new UserRevertedException("Leaf Op Needs key");
        }
        if (value.length == 0) {
            throw new UserRevertedException("Leaf Op Needs Value");
        }
        byte[] pKey = prepareLeafData(leafOp.getPrehashKey(), leafOp.getLength(), key);
        byte[] pValue = prepareLeafData(leafOp.getPrehashValue(), leafOp.getLength(), value);
        byte[] data = ByteUtil.join(leafOp.getPrefix(), pKey, pValue);
        return doHash(leafOp.getHash(), data);
    }

    public static byte[] prepareLeafData(int hashOp, int lenOp, byte[] data) {
        byte[] hashed = doHashOrNoop(hashOp, data);
        return doLengthOp(lenOp, hashed);
    }

    public static byte[] doHashOrNoop(int hashOp, byte[] preImage) {
        if (hashOp == HashOp.NO_HASH) {
            return preImage;
        }
        return doHash(hashOp, preImage);
    }

    public static byte[] doHash(int hashOp, byte[] preImage) {
        if (hashOp == HashOp.SHA256) {
            return Context.hash("sha-256", preImage);
        } else if (hashOp == HashOp.KECCAK) {
            return Context.hash("keccak-256", preImage);
        } else if (hashOp == HashOp.RIPEMD160) {
            throw new UserRevertedException("RIPEMD160 hash not supported");
        } else if (hashOp == HashOp.BITCOIN) {
            throw new UserRevertedException("Bitcoin hash not supported");
        } else if (hashOp == HashOp.SHA512) {
            throw new UserRevertedException("SHA512 hash not supported");
        } else if (hashOp == HashOp.SHA512_256) {
            throw new UserRevertedException("SHA512-256 hash not supported");
        } else {
            throw new UserRevertedException("Unsupported hash operation");
        }
    }

    public static byte[] doLengthOp(int lenOp, byte[] data) {
        if (lenOp == LengthOp.NO_PREFIX) {
            return data;
        } else if (lenOp == LengthOp.VAR_PROTO) {
            //TODO: Implement var proto operation
        } else if (lenOp == LengthOp.REQUIRE_32_BYTES) {
            if (data.length != 32) {
                throw new UserRevertedException("Length of data should be 32");
            }
        } else if (lenOp == LengthOp.REQUIRE_64_BYTES) {
            if (data.length != 64) {
                throw new UserRevertedException("Length of data should be 64");
            }
        } else if (lenOp == LengthOp.FIXED32_LITTLE) {
            BigInteger size = BigInteger.valueOf(data.length);
            byte[] sizeB = size.toByteArray();
            byte[] littleE = new byte[4];
            littleE[0] = sizeB[3];
            littleE[1] = sizeB[2];
            littleE[2] = sizeB[1];
            littleE[3] = sizeB[0];
            return ByteUtil.join(littleE, data);
        } else {
            throw new UserRevertedException("Unsupported lenOp");
        }
        return empty;
    }

    public static void checkAgainstSpec(LeafOp leafOp, ProofSpec spec) {
        if (leafOp.getHash() != spec.getLeafSpec().getHash()) {
            throw new UserRevertedException("checkAgainstSpec for LeafOp - Unexpected HashOp");
        }
        if (leafOp.getPrehashKey() != spec.getLeafSpec().getPrehashKey()) {
            throw new UserRevertedException("CheckAgainstSpec for LeafOp - Unexpected PreHashKey");
        }
        if (leafOp.getPrehashValue() != spec.getLeafSpec().getPrehashValue()) {
            throw new UserRevertedException("CheckAgainstSpec for LeafOp - Unexpected PrehashValue");
        }
        if (leafOp.getLength() != spec.getLeafSpec().getLength()) {
            throw new UserRevertedException("CheckAgainstSpec for LeafOp - Unexpected lengthOp");
        }
        boolean hasPrefix = hasPrefix(leafOp.getPrefix(), spec.getLeafSpec().getPrefix());
        if (!hasPrefix) {
            throw new UserRevertedException("CheckAgainstSpec for LeafOp - Leaf Prefix doesn't start with spec prefix");
        }
    }

    public static byte[] applyOp(InnerOp innerOp, byte[] child) {
        if (child.length == 0) {
            throw new UserRevertedException("InnerOp needs child value");
        }
        byte[] preImage = ByteUtil.join(innerOp.getPrefix(), child, innerOp.getSuffix());
        return doHash(innerOp.getHash(), preImage);
    }

    public static void checkAgainstSpec(InnerOp innerOp, ProofSpec spec) {
        if (innerOp.getHash() != spec.getInnerSpec().getHash()) {
            throw new UserRevertedException("CheckAgainstSpec for InnerOp - Unexpected HashOp");
        }
        int minPrefixLength = spec.getInnerSpec().getMinPrefixLength().intValue();
        if (innerOp.getPrefix().length < minPrefixLength) {
            throw new UserRevertedException("InnerOp prefix too short");
        }
        byte[] leafPrefix = spec.getLeafSpec().getPrefix();
        boolean hasPrefix = hasPrefix(innerOp.getPrefix(), leafPrefix);
        if (hasPrefix) {
            throw new UserRevertedException("Inner Prefix starts with wrong value");
        }

        BigInteger childSize = spec.getInnerSpec().getChildSize();
        BigInteger maxLeftChildBytes =
                BigInteger.valueOf(spec.getInnerSpec().getChildOrder().size() - 1).multiply(childSize);
        BigInteger maxPrefixLength = spec.getInnerSpec().getMaxPrefixLength();
        if (BigInteger.valueOf(innerOp.getPrefix().length).compareTo(maxPrefixLength.add(maxLeftChildBytes)) > 0) {
            throw new UserRevertedException("InnerOp prefix too long");
        }
    }

    public static boolean hasPrefix(byte[] element, byte[] prefix) {
        if (prefix.length == 0) {
            return true;
        }
        if (prefix.length > element.length) {
            return false;
        }
        byte[] slice = Arrays.copyOfRange(element, 0, prefix.length);
        return Arrays.equals(prefix, slice);
    }

    public static int compare(byte[] a, byte[] b) {
        int minLen = Math.min(a.length, b.length);
        for (int i = 0; i < minLen; i++) {
            int aInt = a[i] & 0xff;
            int bInt = b[i] & 0xff;
            if (aInt < bInt) {
                return -1;
            } else if (aInt > bInt) {
                return 1;
            }
        }

        if (a.length > minLen) {
            return 1;
        }
        if (b.length > minLen) {
            return -1;
        }
        return 0;
    }
}
