package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class PartSetHeader extends ProtoMessage {
  private BigInteger total = BigInteger.ZERO;

  private byte[] hash = new byte[0];

  public BigInteger getTotal() {
    return this.total;
  }

  public void setTotal(BigInteger total) {
    this.total = total;
  }

  public byte[] getHash() {
    return this.hash;
  }

  public void setHash(byte[] hash) {
    this.hash = hash;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.total),
      Proto.encode(2, this.hash));
  }

  public static PartSetHeader decode(byte[] data) {
    PartSetHeader obj = new PartSetHeader();
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
            obj.total = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.hash = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
