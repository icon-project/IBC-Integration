package ibc.icon.score.util;

import scorex.util.ArrayList;

import java.math.BigInteger;
import java.util.List;

public class Proto {

    public static class DecodeResponse<T> {
        public T res;
        public int index;
    }

    public static DecodeResponse<Boolean> decodeBoolean(byte[] data, int index) {
        DecodeResponse<Boolean> resp = new DecodeResponse<>();
        resp.index = index + 1;
        resp.res = data[index] == 1;

        return resp;
    }

    public static DecodeResponse<String> decodeString(byte[] data, int index) {
        DecodeResponse<String> resp = new DecodeResponse<>();
        DecodeResponse<byte[]> bytesResp = decodeBytes(data, index);
        resp.index = bytesResp.index;
        resp.res = new String(bytesResp.res);

        return resp;

    }

    public static DecodeResponse<byte[]> decodeBytes(byte[] data, int index) {
        DecodeResponse<byte[]> resp = new DecodeResponse<>();

        DataSize dataSize = getDataSize(data, index);
        byte[] res = new byte[dataSize.length];

        System.arraycopy(data, dataSize.index, res, 0, dataSize.length);
        resp.index = dataSize.index + dataSize.length;
        resp.res = res;

        return resp;
    }

    public static DecodeResponse<Integer> decodeEnum(byte[] data, int index) {
        int result = 0;
        DecodeResponse<Integer> resp = new DecodeResponse<>();

        for (int shift = 0; shift < 64; shift += 7) {
            final byte b = data[index];
            index++;
            result |= (b & 0x7F) << shift;
            if ((b & 0x80) == 0) {
                break;
            }
        }

        resp.index = index;
        resp.res = result;
        return resp;
    }

    public static DecodeResponse<BigInteger> decodeVarInt(byte[] data, int index) {
        DecodeResponse<BigInteger> resp = new DecodeResponse<>();

        long value = 0;
        for (int shift = 0; shift < 64; shift += 7) {
            final byte b = data[index];
            index++;
            value |= (long) (b & 0x7F) << shift;
            if ((b & 0x80) == 0) {
                break;
            }
        }

        resp.index = index;
        resp.res = BigInteger.valueOf(value);
        return resp;
    }

    public static DecodeResponse<BigInteger> decodeFixed64(byte[] data, int index) {
        DecodeResponse<BigInteger> resp = new DecodeResponse<>();
        long res = (((data[index] & 0xffL))
                | ((data[index + 1] & 0xffL) << 8)
                | ((data[index + 2] & 0xffL) << 16)
                | ((data[index + 3] & 0xffL) << 24)
                | ((data[index + 4] & 0xffL) << 32)
                | ((data[index + 5] & 0xffL) << 40)
                | ((data[index + 6] & 0xffL) << 48)
                | ((data[index + 7] & 0xffL) << 56));

        resp.index = index + 8;
        resp.res = BigInteger.valueOf(res);
        return resp;
    }

    public static byte[] encodeMessageArray(int order, List<? extends ProtoMessage> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length][];
        for (int i = 0; i < length; i++) {
            encodedItems[i] = encode(order, items.get(i));
        }

        return ByteUtil.join(encodedItems);
    }

    public static byte[] encode(int order, ProtoMessage item) {
        if (item == null) {
            return new byte[0];
        }

        byte[] encodedItem =  item.encode();
        if (encodedItem.length == 0){
            return new byte[] {(byte)(order << 3 | 2), 0};
        }

        return encode(order, encodedItem);
    }

    public static byte[] encodeStringArray(int order, List<String> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length][];
        for (int i = 0; i < length; i++) {
            encodedItems[i] = encode(order, items.get(i));
        }

        return ByteUtil.join(encodedItems);
    }

    public static byte[] encode(int order, String item) {
        return encode(order, item.getBytes());
    }

    public static byte[] encodeBytesArray(int order, List<byte[]> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length][];
        for (int i = 0; i < length; i++) {
            encodedItems[i] = encode(order, items.get(i));
        }

        return ByteUtil.join(encodedItems);
    }

    public static byte[] encode(int order, byte[] item) {
        if (item.length == 0) {
            return new byte[0];
        }

        byte[] length = encodeVarInt(BigInteger.valueOf(item.length));

        byte[] bs = new byte[item.length + 1 + length.length];

        bs[0] = (byte) (order << 3 | 2);

        System.arraycopy(length, 0, bs, 1, length.length);
        System.arraycopy(item, 0, bs, 1 + length.length, item.length);

        return bs;
    }

    public static byte[] encodeBooleanArray(int order, List<Boolean> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length][];
        for (int i = 0; i < length; i++) {
            encodedItems[i] = encode(order, items.get(i));
        }

        return ByteUtil.join(encodedItems);
    }

    public static byte[] encode(int order, Boolean item) {
        if (item == null) {
            return new byte[0];
        }

        byte[] bs = new byte[2];
        bs[0] = (byte) (order << 3);
        bs[1] = (byte) (item ? 1 : 0);

        return bs;
    }

    public static byte[] encodeVarIntArray(int order, List<BigInteger> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length + 2][];
        encodedItems[0] = new byte[]{(byte) (order << 3 | 2)};
        int size = 0;
        for (int i = 0; i < length; i++) {
            byte[] val = encodeVarInt(items.get(i));
            encodedItems[i + 2] = val;
            size = size + val.length;
        }
        encodedItems[1] = encodeVarInt(BigInteger.valueOf(size));

        return ByteUtil.join(encodedItems);
    }

    public static DecodeResponse<List<BigInteger>> decodeVarIntArray(byte[] data, int index) {
        DecodeResponse<List<BigInteger>> response = new DecodeResponse<>();
        response.res = new ArrayList<>();
        DataSize dataSize = getDataSize(data, index);
        index = dataSize.index;
        int startIndex = index;

        while (dataSize.length > index - startIndex) {
            DecodeResponse<BigInteger> valResp = decodeVarInt(data, index);
            response.res.add(valResp.res);
            index = valResp.index;
        }

        response.index = index;
        return response;
    }

    public static byte[] encode(int order, int item) {
        return encode(order, BigInteger.valueOf(item));
    }

    public static byte[] encode(int order, BigInteger item) {
        if (item.equals(BigInteger.ZERO)) {
            return new byte[0];
        }

        byte[] varInt = encodeVarInt(item);
        byte[] bs = new byte[varInt.length + 1];

        bs[0] = (byte) (order << 3);
        System.arraycopy(varInt, 0, bs, 1, varInt.length);

        return bs;
    }

    public static byte[] encodeVarInt(BigInteger item) {
        if (item.equals(BigInteger.ZERO)) {
            return new byte[0];
        }

        int size = estimateVarIntSize(item);

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

    public static int estimateVarIntSize(BigInteger item) {
        int size = 0;
        long value = item.longValue();

        while (true) {
            size++;
            if ((value & ~0x7FL) == 0) {
                return size;
            } else {
                value >>>= 7;
            }
        }
    }

    public static byte[] encodeFixed64Array(int order, List<BigInteger> items) {
        int length = items.size();
        byte[][] encodedItems = new byte[length][];
        for (int i = 0; i < length; i++) {
            encodedItems[i] = encodeFixed64(order, items.get(i));
        }

        return ByteUtil.join(encodedItems);
    }

    public static byte[] encodeFixed64(int order, BigInteger item) {
        if (item.equals(BigInteger.ZERO)) {
            return new byte[0];
        }
        byte[] bs = new byte[9];
        bs[0] = (byte) (order << 3 | 1);
        byte[] num = encodeFixed64(item, true);
        System.arraycopy(num, 0, bs, 1, num.length);
        return bs;
    }

    public static byte[] encodeFixed64(BigInteger item, boolean littleEndian) {
        byte[] bs = new byte[8];
        if (item.equals(BigInteger.ZERO)) {
            return bs;
        }
        long l = item.longValue();
        if (littleEndian) {
            for (int i = 0; i < 8; i++) {
                bs[i] = (byte) (l & 0xFF);
                l >>= 8;
            }
        } else {
            for (int i = 7; i >= 0; i--) {
                bs[i] = (byte) (l & 0xFF);
                l >>= 8;
            }
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


    private static DataSize getDataSize(byte[] data, int index) {
        int length = 0;
        for (int shift = 0; shift < 64; shift += 7) {
            final byte b = data[index];
            index++;
            length |= (long) (b & 0x7F) << shift;
            if ((b & 0x80) == 0) {
                break;
            }
        }
        return new DataSize(index, length);
    }

    private static class DataSize {

        public final int index;
        public final int length;

        public DataSize(int index, int length) {
            this.index = index;
            this.length = length;
        }
    }
}
