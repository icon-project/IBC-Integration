package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class BlockParams extends ProtoMessage {
  private BigInteger maxBytes = BigInteger.ZERO;

  private BigInteger maxGas = BigInteger.ZERO;

  public BigInteger getMaxBytes() {
    return this.maxBytes;
  }

  public void setMaxBytes(BigInteger maxBytes) {
    this.maxBytes = maxBytes;
  }

  public BigInteger getMaxGas() {
    return this.maxGas;
  }

  public void setMaxGas(BigInteger maxGas) {
    this.maxGas = maxGas;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.maxBytes),
      Proto.encode(2, this.maxGas));
  }

  public static BlockParams decode(byte[] data) {
    BlockParams obj = new BlockParams();
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
            obj.maxBytes = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.maxGas = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
