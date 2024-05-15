package cosmos.upgrade.v1beta1;

import google.protobuf.Any;
import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class Plan extends ProtoMessage {
  private String name = "";

  private Timestamp time = new google.protobuf.Timestamp();

  private BigInteger height = BigInteger.ZERO;

  private String info = "";

  private Any upgradedClientState = new google.protobuf.Any();

  public String getName() {
    return this.name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public Timestamp getTime() {
    return this.time;
  }

  public void setTime(Timestamp time) {
    this.time = time;
  }

  public BigInteger getHeight() {
    return this.height;
  }

  public void setHeight(BigInteger height) {
    this.height = height;
  }

  public String getInfo() {
    return this.info;
  }

  public void setInfo(String info) {
    this.info = info;
  }

  public Any getUpgradedClientState() {
    return this.upgradedClientState;
  }

  public void setUpgradedClientState(Any upgradedClientState) {
    this.upgradedClientState = upgradedClientState;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.name),
      Proto.encode(2, this.time),
      Proto.encode(3, this.height),
      Proto.encode(4, this.info),
      Proto.encode(5, this.upgradedClientState));
  }

  public static Plan decode(byte[] data) {
    Plan obj = new Plan();
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
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.time = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.height = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.info = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.upgradedClientState = google.protobuf.Any.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
