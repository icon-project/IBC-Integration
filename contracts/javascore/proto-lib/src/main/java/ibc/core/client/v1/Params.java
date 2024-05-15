package ibc.core.client.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class Params extends ProtoMessage {
  private List<String> allowedClients = new ArrayList<>();

  public List<String> getAllowedClients() {
    return this.allowedClients;
  }

  public void setAllowedClients(List<String> allowedClients) {
    this.allowedClients = allowedClients;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeStringArray(1, this.allowedClients));
  }

  public static Params decode(byte[] data) {
    Params obj = new Params();
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
            obj.allowedClients.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
