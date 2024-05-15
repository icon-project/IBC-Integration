package ibc.core.client.v1;

import google.protobuf.Any;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class IdentifiedClientState extends ProtoMessage {
  private String clientId = "";

  private Any clientState = new google.protobuf.Any();

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public Any getClientState() {
    return this.clientState;
  }

  public void setClientState(Any clientState) {
    this.clientState = clientState;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.clientId),
      Proto.encode(2, this.clientState));
  }

  public static IdentifiedClientState decode(byte[] data) {
    IdentifiedClientState obj = new IdentifiedClientState();
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
            obj.clientId = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.clientState = google.protobuf.Any.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
