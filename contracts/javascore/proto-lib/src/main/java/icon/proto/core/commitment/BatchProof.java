package icon.proto.core.commitment;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class BatchProof extends ProtoMessage {
  private List<BatchEntry> entries = new ArrayList<>();

  public List<BatchEntry> getEntries() {
    return this.entries;
  }

  public void setEntries(List<BatchEntry> entries) {
    this.entries = entries;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.entries));
  }

  public static BatchProof decode(byte[] data) {
    BatchProof obj = new BatchProof();
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
            obj.entries.add(BatchEntry.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
