/*
 * Copyright 2021 ICON Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package ibc.icon.score.util;

import scorex.util.ArrayList;
import java.util.List;

public class ByteUtil {

    public static byte[] join(byte[]... data) {
        int length = 0;
        for (byte[] bs : data) {
            length += bs.length;
        }

        byte[] result = new byte[length];
        int index = 0;
        for (byte[] bs : data) {
            System.arraycopy(bs, 0, result, index, bs.length);
            index += bs.length;
        }

        return result;
    }

    public static byte[] convertBytesToStringBytes(String hexString) {
        var hexStringInBytes = StringUtil.hexToBytes(hexString);
        List<Integer> intArr = new ArrayList<>();
        for (byte b : hexStringInBytes) {
            intArr.add(Byte.toUnsignedInt(b));
        }
        StringBuilder result = new StringBuilder();
        result.append("[");
        for (Integer integer : intArr) {
            result.append(integer).append(",");
        }
        result.deleteCharAt(result.length() - 1);
        result.append("]");
        return result.toString().getBytes();
    }

    public static byte[] convertBytesToStringBytes(byte[] inputByte) {
        return convertBytesToStringBytes(StringUtil.bytesToHex(inputByte));
    }
}
