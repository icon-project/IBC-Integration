package ibc.icon.structs.proto.lightclient.tendermint;

import java.math.BigInteger;

public class Vote {
    public SignedMsgType type;
    public BigInteger height;
    public BigInteger round;
    public BlockID blockId;
    public Timestamp timestamp;
    public byte[] validatorAddress;
    public BigInteger validatorIndex;
    public byte[] signature;
}