package ibc.icon.structs.proto.lightclient.tendermint;

public class SignedMsgType {
    public final static int SIGNED_MSG_TYPE_UNKNOWN = 0;
    public final static int SIGNED_MSG_TYPE_PREVOTE = 1;
    public final static int SIGNED_MSG_TYPE_PRECOMMIT = 2;
    public final static int SIGNED_MSG_TYPE_PROPOSAL = 3;
}
