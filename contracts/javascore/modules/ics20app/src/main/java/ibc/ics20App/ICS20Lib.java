package ibc.ics20app;

import ibc.ics24.host.IBCCommitment;
import ibc.icon.score.util.StringUtil;
import java.math.BigInteger;

public class ICS20Lib {

    public static class PacketData {
        public String denom;
        public String sender;
        public String receiver;
        public BigInteger amount;
        public String memo;
    }


    public static final byte[] SUCCESSFUL_ACKNOWLEDGEMENT_JSON = "{\"result\":\"AQ==\"}".getBytes();
    public static final byte[] FAILED_ACKNOWLEDGEMENT_JSON = "{\"error\":\"failed\"}".getBytes();
    public static final byte[] KECCAK256_SUCCESSFUL_ACKNOWLEDGEMENT_JSON = IBCCommitment.keccak256(SUCCESSFUL_ACKNOWLEDGEMENT_JSON);
    public static final Integer CHAR_SLASH = 0x2f;
    public static final Integer CHAR_BACKSLASH = 0x5c;
    public static final Integer CHAR_F = 0x66;
    public static final Integer CHAR_R = 0x72;
    public static final Integer CHAR_N = 0x6e;
    public static final Integer CHAR_B = 0x62;
    public static final Integer CHAR_T = 0x74;
    public static final Integer CHAR_CLOSING_BRACE = 0x7d;
    public static final Integer CHAR_M = 0x6d;
    private static final char[] HEX_DIGITS = "0123456789abcdef".toCharArray();

    private static final int CHAR_DOUBLE_QUOTE = '"';

    static boolean isEscapeNeededString(byte[] bz) {
        for (byte b : bz) {
            int c = b & 0xFF;
            if (c == CHAR_DOUBLE_QUOTE) {
                return true;
            }
        }
        return false;
    }

    public byte[] marshalUnsafeJSON(PacketData data) {
        if (data.memo.isEmpty()) {
            return marshalJson(data.denom, data.amount, data.sender, data.receiver);
        } else {
            return marshalJson(data.denom, data.amount, data.sender, data.receiver, data.memo);
        }
    }


    public static byte[] marshalJson(String escapedDenom, BigInteger amount, String escapedSender, String escapedReceiver) {
        String jsonString = "{" +
                "\"amount\":\"" + amount.toString() + "\"," +
                "\"denom\":\"" + escapedDenom + "\"," +
                "\"receiver\":\"" + escapedReceiver + "\"," +
                "\"sender\":\"" + escapedSender + "\"" +
                "}";

        return jsonString.getBytes();
    }

    public static byte[] marshalJson(String escapedDenom, BigInteger amount, String escapedSender, String escapedReceiver, String escapedMemo) {
        String jsonString = "{" +
                "\"amount\":\"" + amount.toString() + "\"," +
                "\"denom\":\"" + escapedDenom + "\"," +
                "\"receiver\":\"" + escapedReceiver + "\"," +
                "\"sender\":\"" + escapedSender + "\"," +
                "\"memo\":\"" + escapedMemo + "\"" +
                "}";

        return jsonString.getBytes();
    }

    public static PacketData unmarshalJSON(byte[] packet) {
        StringBuilder sanitized = new StringBuilder();
        String jsonString = new String(packet);

        for (char c : jsonString.toCharArray()){
            if (c != '\\' && c != '\"' && c !='{' && c!='}'){
                sanitized.append(c);
            }
        }
        jsonString=sanitized.toString();

        String[] jsonParts = StringUtil.split(jsonString, ',');

        PacketData data = new PacketData();

        data.amount = new BigInteger(getValue(jsonParts[0]));

        data.denom = getValue(jsonParts[1]);
        data.receiver = getValue(jsonParts[2]);
        data.sender = getValue(jsonParts[3]);
        if (jsonParts.length > 4) {
            data.memo = getValue(jsonParts[4]);
        } else {
            data.memo = "";
        }

        return data;
    }

    private static String getValue(String keyValue) {
        return  StringUtil.split(keyValue, ':')[1].trim();
        
    }

}
