package cosmos.ics23.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class BatchEntry extends ProtoMessage {
  private ExistenceProof exist = new ExistenceProof();

  private NonExistenceProof nonexist = new NonExistenceProof();

  public ExistenceProof getExist() {
    return this.exist;
  }

  public void setExist(ExistenceProof exist) {
    this.exist = exist;
  }

  public NonExistenceProof getNonexist() {
    return this.nonexist;
  }

  public void setNonexist(NonExistenceProof nonexist) {
    this.nonexist = nonexist;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.exist),
      Proto.encode(2, this.nonexist));
  }

  public static BatchEntry decode(byte[] data) {
    BatchEntry obj = new BatchEntry();
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
            obj.exist = ExistenceProof.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nonexist = NonExistenceProof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
