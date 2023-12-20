package ibc.ics20.app;

import ibc.ics24.host.IBCCommitment;
import score.Context;

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

    // Function to check if escape is needed for a byte array in Java
    static boolean isEscapeNeededString(byte[] bz) {
        for (int i = 0; i < bz.length; i++) {
            int c = bz[i] & 0xFF; // Convert to unsigned int
            if (c == CHAR_DOUBLE_QUOTE) {
                return true;
            }
        }
        return false;
    }



    public static byte[] marshalJson(String escapedDenom, BigInteger amount, String escapedSender, String escapedReceiver) {
        String jsonString = "{\"amount\":\"" +
                amount.toString() +
                "\",\"denom\":\"" +
                escapedDenom +
                "\",\"receiver\":\"" +
                escapedReceiver +
                "\",\"sender\":\"" +
                escapedSender +
                "\"}";

        return jsonString.getBytes();
    }

    // write a code do unmarshalJSON function in java with above params
    public static PacketData unmarshalJSON(byte[] packet) {
        String jsonString = new String(packet);
        PacketData data = new PacketData();
        data.amount = new BigInteger(jsonString.substring(jsonString.indexOf("amount") + 9, jsonString.indexOf("receiver") - 3));
        data.denom = jsonString.substring(jsonString.indexOf("denom") + 8, jsonString.indexOf("amount") - 3);
        data.memo = jsonString.substring(jsonString.indexOf("memo") + 8, jsonString.indexOf("memo") - 3);
        data.receiver = jsonString.substring(jsonString.indexOf("receiver") + 11, jsonString.indexOf("sender") - 3);
        data.sender = jsonString.substring(jsonString.indexOf("sender") + 9, jsonString.indexOf("}") - 1);
        return data;
    }


    static String addressToHexString(String addr) {
        StringBuilder hexString = new StringBuilder("0x");
        long localValue = Long.parseLong(addr.substring(2), 16);

        for (int i = 39; i >= 0; --i) {
            hexString.append(HEX_DIGITS[(int) (localValue & 0xf)]);
            localValue >>= 4;
        }

        if (localValue != 0) {
            Context.revert("Insufficient hex length");
        }

        return hexString.toString();
    }





}
