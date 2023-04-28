package icon.proto.icon.lightclient.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class Misbehaviour extends ProtoMessage {
  private String clientId = "";

  private BlockUpdate header1 = new BlockUpdate();

  private BlockUpdate header2 = new BlockUpdate();

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public BlockUpdate getHeader1() {
    return this.header1;
  }

  public void setHeader1(BlockUpdate header1) {
    this.header1 = header1;
  }

  public BlockUpdate getHeader2() {
    return this.header2;
  }

  public void setHeader2(BlockUpdate header2) {
    this.header2 = header2;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.clientId),
      Proto.encode(2, this.header1),
      Proto.encode(3, this.header2));
  }

  public static Misbehaviour decode(byte[] data) {
    Misbehaviour obj = new Misbehaviour();
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
            obj.header1 = BlockUpdate.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.header2 = BlockUpdate.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
