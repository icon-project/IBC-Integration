package ibc.icon.score.util;

import org.junit.jupiter.api.Test;

import java.math.BigInteger;

import static ibc.icon.score.util.StringUtil.bytesToHex;
import static org.junit.jupiter.api.Assertions.assertEquals;

class ProtoTest {

    @Test
    void encodeFixed64() {
        BigInteger num = BigInteger.valueOf(858);
        assertEquals("5a03000000000000", bytesToHex(Proto.encodeFixed64(num, true)));
        assertEquals("000000000000035a", bytesToHex(Proto.encodeFixed64(num, false)));
    }
}