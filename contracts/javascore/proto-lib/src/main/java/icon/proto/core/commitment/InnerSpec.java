package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class InnerSpec extends ProtoMessage {
  private List<BigInteger> childOrder = new ArrayList<>();

  private BigInteger childSize = BigInteger.ZERO;

  private BigInteger minPrefixLength = BigInteger.ZERO;

  private BigInteger maxPrefixLength = BigInteger.ZERO;

  private byte[] emptyChild = new byte[0];

  private int hash = 0;

  public List<BigInteger> getChildOrder() {
    return this.childOrder;
  }

  public void setChildOrder(List<BigInteger> childOrder) {
    this.childOrder = childOrder;
  }

  public BigInteger getChildSize() {
    return this.childSize;
  }

  public void setChildSize(BigInteger childSize) {
    this.childSize = childSize;
  }

  public BigInteger getMinPrefixLength() {
    return this.minPrefixLength;
  }

  public void setMinPrefixLength(BigInteger minPrefixLength) {
    this.minPrefixLength = minPrefixLength;
  }

  public BigInteger getMaxPrefixLength() {
    return this.maxPrefixLength;
  }

  public void setMaxPrefixLength(BigInteger maxPrefixLength) {
    this.maxPrefixLength = maxPrefixLength;
  }

  public byte[] getEmptyChild() {
    return this.emptyChild;
  }

  public void setEmptyChild(byte[] emptyChild) {
    this.emptyChild = emptyChild;
  }

  public int getHash() {
    return this.hash;
  }

  public void setHash(int hash) {
    this.hash = hash;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeVarIntArray(1, this.childOrder),
      Proto.encode(2, this.childSize),
      Proto.encode(3, this.minPrefixLength),
      Proto.encode(4, this.maxPrefixLength),
      Proto.encode(5, this.emptyChild),
      Proto.encode(6, this.hash));
  }

  public static InnerSpec decode(byte[] data) {
    InnerSpec obj = new InnerSpec();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<List<BigInteger>> resp = Proto.decodeVarIntArray(data, index);
            index = resp.index;
            obj.childOrder.addAll(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.childSize = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.minPrefixLength = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.maxPrefixLength = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.emptyChild = resp.res;
            break;
        }
        case 6: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.hash = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
