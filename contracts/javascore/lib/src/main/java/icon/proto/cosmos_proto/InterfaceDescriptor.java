package cosmos_proto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class InterfaceDescriptor extends ProtoMessage {
  private String name = "";

  private String description = "";

  public String getName() {
    return this.name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public String getDescription() {
    return this.description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.name),
      Proto.encode(2, this.description));
  }

  public static InterfaceDescriptor decode(byte[] data) {
    InterfaceDescriptor obj = new InterfaceDescriptor();
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
            obj.name = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.description = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
