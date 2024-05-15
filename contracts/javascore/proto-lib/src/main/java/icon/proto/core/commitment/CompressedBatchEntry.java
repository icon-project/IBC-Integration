package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class CompressedBatchEntry extends ProtoMessage {
  private CompressedExistenceProof exist = new CompressedExistenceProof();

  private CompressedNonExistenceProof nonexist = new CompressedNonExistenceProof();

  public CompressedExistenceProof getExist() {
    return this.exist;
  }

  public void setExist(CompressedExistenceProof exist) {
    this.exist = exist;
  }

  public CompressedNonExistenceProof getNonexist() {
    return this.nonexist;
  }

  public void setNonexist(CompressedNonExistenceProof nonexist) {
    this.nonexist = nonexist;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.exist),
      Proto.encode(2, this.nonexist));
  }

  public static CompressedBatchEntry decode(byte[] data) {
    CompressedBatchEntry obj = new CompressedBatchEntry();
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
            obj.exist = CompressedExistenceProof.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nonexist = CompressedNonExistenceProof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
