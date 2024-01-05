package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class ABCIParams extends ProtoMessage {
  private BigInteger voteExtensionsEnableHeight = BigInteger.ZERO;

  public BigInteger getVoteExtensionsEnableHeight() {
    return this.voteExtensionsEnableHeight;
  }

  public void setVoteExtensionsEnableHeight(BigInteger voteExtensionsEnableHeight) {
    this.voteExtensionsEnableHeight = voteExtensionsEnableHeight;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.voteExtensionsEnableHeight));
  }

  public static ABCIParams decode(byte[] data) {
    ABCIParams obj = new ABCIParams();
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
            obj.voteExtensionsEnableHeight = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
