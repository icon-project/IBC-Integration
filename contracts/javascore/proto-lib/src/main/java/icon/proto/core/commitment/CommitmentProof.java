package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class CommitmentProof extends ProtoMessage {
  private ExistenceProof exist = new ExistenceProof();

  private NonExistenceProof nonexist = new NonExistenceProof();

  private BatchProof batch = new BatchProof();

  private CompressedBatchProof compressed = new CompressedBatchProof();

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

  public BatchProof getBatch() {
    return this.batch;
  }

  public void setBatch(BatchProof batch) {
    this.batch = batch;
  }

  public CompressedBatchProof getCompressed() {
    return this.compressed;
  }

  public void setCompressed(CompressedBatchProof compressed) {
    this.compressed = compressed;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.exist),
      Proto.encode(2, this.nonexist),
      Proto.encode(3, this.batch),
      Proto.encode(4, this.compressed));
  }

  public static CommitmentProof decode(byte[] data) {
    CommitmentProof obj = new CommitmentProof();
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
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.batch = BatchProof.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.compressed = CompressedBatchProof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
