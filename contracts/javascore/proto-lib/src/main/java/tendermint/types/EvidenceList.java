package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class EvidenceList extends ProtoMessage {
  private List<Evidence> evidence = new ArrayList<>();

  public List<Evidence> getEvidence() {
    return this.evidence;
  }

  public void setEvidence(List<Evidence> evidence) {
    this.evidence = evidence;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.evidence));
  }

  public static EvidenceList decode(byte[] data) {
    EvidenceList obj = new EvidenceList();
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
            obj.evidence.add(Evidence.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
