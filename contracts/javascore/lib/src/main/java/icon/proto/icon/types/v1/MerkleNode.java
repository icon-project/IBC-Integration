package icon.proto.icon.types.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class MerkleNode extends ProtoMessage {
  private BigInteger dir = BigInteger.ZERO;

  private byte[] value = new byte[0];

  public BigInteger getDir() {
    return this.dir;
  }

  public void setDir(BigInteger dir) {
    this.dir = dir;
  }

  public byte[] getValue() {
    return this.value;
  }

  public void setValue(byte[] value) {
    this.value = value;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.dir),
      Proto.encode(2, this.value));
  }

  public static MerkleNode decode(byte[] data) {
    MerkleNode obj = new MerkleNode();
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
            obj.dir = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.value = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
