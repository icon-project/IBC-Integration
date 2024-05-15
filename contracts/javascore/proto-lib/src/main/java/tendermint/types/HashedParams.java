package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class HashedParams extends ProtoMessage {
  private BigInteger blockMaxBytes = BigInteger.ZERO;

  private BigInteger blockMaxGas = BigInteger.ZERO;

  public BigInteger getBlockMaxBytes() {
    return this.blockMaxBytes;
  }

  public void setBlockMaxBytes(BigInteger blockMaxBytes) {
    this.blockMaxBytes = blockMaxBytes;
  }

  public BigInteger getBlockMaxGas() {
    return this.blockMaxGas;
  }

  public void setBlockMaxGas(BigInteger blockMaxGas) {
    this.blockMaxGas = blockMaxGas;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.blockMaxBytes),
      Proto.encode(2, this.blockMaxGas));
  }

  public static HashedParams decode(byte[] data) {
    HashedParams obj = new HashedParams();
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
            obj.blockMaxBytes = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.blockMaxGas = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
