package ibc.icon.score.util;

import java.math.BigInteger;

public class Proto {

    public static byte[] encode(int order, String item) {
        return encode(order, item.getBytes());
    }

    public static byte[] encode(int order, byte[] item) {
        if (item == null) {
            return new byte[0];
        }

        byte[] bs = new byte[item.length + 2];

        bs[0] = (byte) (order << 3 | 2);
        bs[1] = (byte) item.length;

        System.arraycopy(item, 0, bs, 2, item.length);

        return bs;
    }

    public static byte[] encode(int order, Boolean item) {
        if (item == null) {
            return new byte[0];
        }

        byte[] bs = new byte[2];
        bs[0] = (byte) (order << 3 | 0);
        bs[1] = (byte) (item ? 1 : 0);

        return bs;
    }

    public static byte[] encode(int order, BigInteger item) {
        if (item == null) {
            return new byte[0];
        }

        byte[] varInt = encodeVarInt(item);
        byte[] bs = new byte[varInt.length + 1];

        bs[0] = (byte) (order << 3 | 0);
        System.arraycopy(varInt, 0, bs, 1, varInt.length);

        return bs;
    }

    public static byte[] encodeVarInt(BigInteger item) {
        if (item == null) {
            return new byte[0];
        }

        int itemBytes = item.bitLength();
        int size = itemBytes / 7;
        if (itemBytes % 7 != 0) {
            size++;
        }
        byte[] res = new byte[size];
        int index = 0;
        long value = item.longValue();

        while (true) {
            if ((value & ~0x7FL) == 0) {
                res[index] = (byte) value;
                break;
            } else {
                res[index] = (byte) (((int) value & 0x7F) | 0x80);
                value >>>= 7;
                index++;
            }
        }
        return res;
    }

    public static byte[] encodeFixed64(int order, BigInteger item) {
        if (item == null) {
            return new byte[0];
        }
        long l = item.longValue();
        byte[] bs = new byte[9];
        bs[0] = (byte) (order << 3 | 1);
        for (int i = 1; i < 9; i++) {
            bs[i] = (byte) (l & 0xFF);
            l >>= 8;
        }

        return bs;
    }

    public static byte[] encodeDelim(byte[] input) {
        // Context.require(input.length < _MAX_UINT64, "Encoder: out of bounds");
        BigInteger length = BigInteger.valueOf(input.length);
        byte[] prefix = encodeVarInt(length);
        byte[] bs = new byte[input.length + prefix.length];

        System.arraycopy(prefix, 0, bs, 0, prefix.length);
        System.arraycopy(input, 0, bs, prefix.length, input.length);

        return bs;
    }

}
