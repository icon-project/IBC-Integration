package icon.proto.core.connection;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class Version extends ProtoMessage {
  private String identifier = "";

  private List<String> features = new ArrayList<>();

  public String getIdentifier() {
    return this.identifier;
  }

  public void setIdentifier(String identifier) {
    this.identifier = identifier;
  }

  public List<String> getFeatures() {
    return this.features;
  }

  public void setFeatures(List<String> features) {
    this.features = features;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.identifier),
      Proto.encodeStringArray(2, this.features));
  }

  public static Version decode(byte[] data) {
    Version obj = new Version();
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
            obj.identifier = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.features.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
