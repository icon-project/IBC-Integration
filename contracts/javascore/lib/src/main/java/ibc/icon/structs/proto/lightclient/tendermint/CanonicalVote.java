package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;

import java.math.BigInteger;

public class CanonicalVote {
    public int type;
    public BigInteger height;
    public BigInteger round;
    public BlockID blockId;
    public Timestamp timestamp;
    public String chainId;

    public byte[] encode() {
        byte[] type = Proto.encode(1, BigInteger.valueOf(this.type));
        byte[] height = Proto.encodeFixed64(2, this.height);
        byte[] round = Proto.encodeFixed64(3, this.round);
        byte[] blockId = Proto.encode(4, this.blockId.encode());
        byte[] timestamp = Proto.encode(5, this.timestamp.encode());
        byte[] chainId = Proto.encode(6, this.chainId);

        return ByteUtil.join(type, height, round, blockId, timestamp, chainId);
    }

}