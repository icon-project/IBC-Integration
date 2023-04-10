package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;

public class LeafOp extends ProtoMessage {
  private int hash = 0;

  private int prehashKey = 0;

  private int prehashValue = 0;

  private int length = 0;

  private byte[] prefix = new byte[0];

  public int getHash() {
    return this.hash;
  }

  public void setHash(int hash) {
    this.hash = hash;
  }

  public int getPrehashKey() {
    return this.prehashKey;
  }

  public void setPrehashKey(int prehashKey) {
    this.prehashKey = prehashKey;
  }

  public int getPrehashValue() {
    return this.prehashValue;
  }

  public void setPrehashValue(int prehashValue) {
    this.prehashValue = prehashValue;
  }

  public int getLength() {
    return this.length;
  }

  public void setLength(int length) {
    this.length = length;
  }

  public byte[] getPrefix() {
    return this.prefix;
  }

  public void setPrefix(byte[] prefix) {
    this.prefix = prefix;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.hash),
      Proto.encode(2, this.prehashKey),
      Proto.encode(3, this.prehashValue),
      Proto.encode(4, this.length),
      Proto.encode(5, this.prefix));
  }

  public static LeafOp decode(byte[] data) {
    LeafOp obj = new LeafOp();
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
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.prehashKey = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.prehashValue = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.length = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.prefix = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
