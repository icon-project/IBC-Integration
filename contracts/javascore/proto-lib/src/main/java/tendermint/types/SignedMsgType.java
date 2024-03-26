package tendermint.types;

public class SignedMsgType {
  public static final int SIGNED_MSG_TYPE_UNKNOWN = 0;

  public static final int SIGNED_MSG_TYPE_PREVOTE = 1;

  public static final int SIGNED_MSG_TYPE_PRECOMMIT = 2;

  public static final int SIGNED_MSG_TYPE_PROPOSAL = 32;
}
