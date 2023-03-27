package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class ExistenceProof extends ProtoMessage {
  private byte[] key = new byte[0];

  private byte[] value = new byte[0];

  private LeafOp leaf;

  private List<InnerOp> path = new ArrayList<>();

  public byte[] getKey() {
    return this.key;
  }

  public void setKey(byte[] key) {
    this.key = key;
  }

  public byte[] getValue() {
    return this.value;
  }

  public void setValue(byte[] value) {
    this.value = value;
  }

  public LeafOp getLeaf() {
    return this.leaf;
  }

  public void setLeaf(LeafOp leaf) {
    this.leaf = leaf;
  }

  public List<InnerOp> getPath() {
    return this.path;
  }

  public void setPath(List<InnerOp> path) {
    this.path = path;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.key),
      Proto.encode(2, this.value),
      Proto.encode(3, this.leaf),
      Proto.encodeMessageArray(4, this.path));
  }

  public static ExistenceProof decode(byte[] data) {
    ExistenceProof obj = new ExistenceProof();
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
            obj.key = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.value = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.leaf = LeafOp.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.path.add(InnerOp.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
