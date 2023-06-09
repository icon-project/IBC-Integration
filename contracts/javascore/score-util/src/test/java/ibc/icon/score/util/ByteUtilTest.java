package ibc.icon.score.util;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class ByteUtilTest {

    @Test
    void convertBytesToStringBytes() {
        var input = "7315ba98a01f9bbf1a74be1de1e439c10c2c5336bb8dabe3f48283812f57b8e2";
        var output = ByteUtil.convertBytesToStringBytes(input);
        var expectedOutput = "5b3131352c32312c3138362c3135322c3136302c33312c3135352c3139312c32362c3131362c3139302c32392c3232352c3232382c35372c3139332c31322c34342c38332c35342c3138372c3134312c3137312c3232372c3234342c3133302c3133312c3132392c34372c38372c3138342c3232365d";
        assertEquals(expectedOutput, StringUtil.bytesToHex(output));
    }
}