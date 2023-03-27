package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class CompressedBatchProof extends ProtoMessage {
  private List<CompressedBatchEntry> entries = new ArrayList<>();

  private List<InnerOp> lookupInners = new ArrayList<>();

  public List<CompressedBatchEntry> getEntries() {
    return this.entries;
  }

  public void setEntries(List<CompressedBatchEntry> entries) {
    this.entries = entries;
  }

  public List<InnerOp> getLookupInners() {
    return this.lookupInners;
  }

  public void setLookupInners(List<InnerOp> lookupInners) {
    this.lookupInners = lookupInners;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.entries),
      Proto.encodeMessageArray(2, this.lookupInners));
  }

  public static CompressedBatchProof decode(byte[] data) {
    CompressedBatchProof obj = new CompressedBatchProof();
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
            obj.entries.add(CompressedBatchEntry.decode(resp.res));
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lookupInners.add(InnerOp.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
