package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class ProofSpec extends ProtoMessage {
  private LeafOp leafSpec;

  private InnerSpec innerSpec;

  private BigInteger maxDepth = BigInteger.ZERO;

  private BigInteger minDepth = BigInteger.ZERO;

  public LeafOp getLeafSpec() {
    return this.leafSpec;
  }

  public void setLeafSpec(LeafOp leafSpec) {
    this.leafSpec = leafSpec;
  }

  public InnerSpec getInnerSpec() {
    return this.innerSpec;
  }

  public void setInnerSpec(InnerSpec innerSpec) {
    this.innerSpec = innerSpec;
  }

  public BigInteger getMaxDepth() {
    return this.maxDepth;
  }

  public void setMaxDepth(BigInteger maxDepth) {
    this.maxDepth = maxDepth;
  }

  public BigInteger getMinDepth() {
    return this.minDepth;
  }

  public void setMinDepth(BigInteger minDepth) {
    this.minDepth = minDepth;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.leafSpec),
      Proto.encode(2, this.innerSpec),
      Proto.encode(3, this.maxDepth),
      Proto.encode(4, this.minDepth));
  }

  public static ProofSpec decode(byte[] data) {
    ProofSpec obj = new ProofSpec();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.leafSpec = LeafOp.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.innerSpec = InnerSpec.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.maxDepth = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.minDepth = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
