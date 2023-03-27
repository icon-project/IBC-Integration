package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class CompressedNonExistenceProof extends ProtoMessage {
  private byte[] key = new byte[0];

  private CompressedExistenceProof left;

  private CompressedExistenceProof right;

  public byte[] getKey() {
    return this.key;
  }

  public void setKey(byte[] key) {
    this.key = key;
  }

  public CompressedExistenceProof getLeft() {
    return this.left;
  }

  public void setLeft(CompressedExistenceProof left) {
    this.left = left;
  }

  public CompressedExistenceProof getRight() {
    return this.right;
  }

  public void setRight(CompressedExistenceProof right) {
    this.right = right;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.key),
      Proto.encode(2, this.left),
      Proto.encode(3, this.right));
  }

  public static CompressedNonExistenceProof decode(byte[] data) {
    CompressedNonExistenceProof obj = new CompressedNonExistenceProof();
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
            obj.left = CompressedExistenceProof.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.right = CompressedExistenceProof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
