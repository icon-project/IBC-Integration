package cosmos.ics23.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;

public class InnerOp extends ProtoMessage {
  private int hash = 0;

  private byte[] prefix = new byte[0];

  private byte[] suffix = new byte[0];

  public int getHash() {
    return this.hash;
  }

  public void setHash(int hash) {
    this.hash = hash;
  }

  public byte[] getPrefix() {
    return this.prefix;
  }

  public void setPrefix(byte[] prefix) {
    this.prefix = prefix;
  }

  public byte[] getSuffix() {
    return this.suffix;
  }

  public void setSuffix(byte[] suffix) {
    this.suffix = suffix;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.hash),
      Proto.encode(2, this.prefix),
      Proto.encode(3, this.suffix));
  }

  public static InnerOp decode(byte[] data) {
    InnerOp obj = new InnerOp();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.hash = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.prefix = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.suffix = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
