package ibc.lightclients.tendermint.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class Misbehaviour extends ProtoMessage {
  private String clientId = "";

  private Header header1 = new Header();

  private Header header2 = new Header();

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public Header getHeader1() {
    return this.header1;
  }

  public void setHeader1(Header header1) {
    this.header1 = header1;
  }

  public Header getHeader2() {
    return this.header2;
  }

  public void setHeader2(Header header2) {
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
            obj.header1 = Header.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.header2 = Header.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
