package google.protobuf;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class Any extends ProtoMessage {
  private String typeUrl = "";

  private byte[] value = new byte[0];

  public String getTypeUrl() {
    return this.typeUrl;
  }

  public void setTypeUrl(String typeUrl) {
    this.typeUrl = typeUrl;
  }

  public byte[] getValue() {
    return this.value;
  }

  public void setValue(byte[] value) {
    this.value = value;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.typeUrl),
      Proto.encode(2, this.value));
  }

  public static Any decode(byte[] data) {
    Any obj = new Any();
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
            obj.typeUrl = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.value = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
