package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class VersionParams extends ProtoMessage {
  private BigInteger app = BigInteger.ZERO;

  public BigInteger getApp() {
    return this.app;
  }

  public void setApp(BigInteger app) {
    this.app = app;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.app));
  }

  public static VersionParams decode(byte[] data) {
    VersionParams obj = new VersionParams();
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
            obj.app = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
