package ibc.ics20.app;

import java.math.BigInteger;

public class ICS20Lib {

    public static class PacketData {
        public String denom;
        public String sender;
        public String receiver;
        public BigInteger amount;
        public String memo;
    }


//    public static final String SUCCESSFUL_ACKNOWLEDGEMENT_JSON = ('{\"result\":\"AQ==\"}');

//    bytes public constant FAILED_ACKNOWLEDGEMENT_JSON = bytes('{"error":"failed"}');
//    bytes32 public constant KECCAK256_SUCCESSFUL_ACKNOWLEDGEMENT_JSON = keccak256(SUCCESSFUL_ACKNOWLEDGEMENT_JSON);

    public static final Integer CHAR_DOUBLE_QUOTE = 0x22;
    public static final Integer CHAR_SLASH = 0x2f;
    public static final Integer CHAR_BACKSLASH = 0x5c;
    public static final Integer CHAR_F = 0x66;
    public static final Integer CHAR_R = 0x72;
    public static final Integer CHAR_N = 0x6e;
    public static final Integer CHAR_B = 0x62;
    public static final Integer CHAR_T = 0x74;
    public static final Integer CHAR_CLOSING_BRACE = 0x7d;
    public static final Integer CHAR_M = 0x6d;
    public static final String HEX_DIGITS = "0123456789abcdef";





}
