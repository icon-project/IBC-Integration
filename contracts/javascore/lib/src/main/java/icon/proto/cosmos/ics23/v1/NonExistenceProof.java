package cosmos.ics23.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class NonExistenceProof extends ProtoMessage {
  private byte[] key = new byte[0];

  private ExistenceProof left = new ExistenceProof();

  private ExistenceProof right = new ExistenceProof();

  public byte[] getKey() {
    return this.key;
  }

  public void setKey(byte[] key) {
    this.key = key;
  }

  public ExistenceProof getLeft() {
    return this.left;
  }

  public void setLeft(ExistenceProof left) {
    this.left = left;
  }

  public ExistenceProof getRight() {
    return this.right;
  }

  public void setRight(ExistenceProof right) {
    this.right = right;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.key),
      Proto.encode(2, this.left),
      Proto.encode(3, this.right));
  }

  public static NonExistenceProof decode(byte[] data) {
    NonExistenceProof obj = new NonExistenceProof();
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
            obj.left = ExistenceProof.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.right = ExistenceProof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
