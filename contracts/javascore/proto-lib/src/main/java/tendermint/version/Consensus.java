package tendermint.version;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class Consensus extends ProtoMessage {
  private BigInteger block = BigInteger.ZERO;

  private BigInteger app = BigInteger.ZERO;

  public BigInteger getBlock() {
    return this.block;
  }

  public void setBlock(BigInteger block) {
    this.block = block;
  }

  public BigInteger getApp() {
    return this.app;
  }

  public void setApp(BigInteger app) {
    this.app = app;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.block),
      Proto.encode(2, this.app));
  }

  public static Consensus decode(byte[] data) {
    Consensus obj = new Consensus();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.block = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.app = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
