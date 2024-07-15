package ibc.core.commitment.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class MerklePath extends ProtoMessage {
  private List<String> keyPath = new ArrayList<>();

  public List<String> getKeyPath() {
    return this.keyPath;
  }

  public void setKeyPath(List<String> keyPath) {
    this.keyPath = keyPath;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeStringArray(1, this.keyPath));
  }

  public static MerklePath decode(byte[] data) {
    MerklePath obj = new MerklePath();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.keyPath.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
