package cosmos.upgrade.v1beta1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class ModuleVersion extends ProtoMessage {
  private String name = "";

  private BigInteger version = BigInteger.ZERO;

  public String getName() {
    return this.name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public BigInteger getVersion() {
    return this.version;
  }

  public void setVersion(BigInteger version) {
    this.version = version;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.name),
      Proto.encode(2, this.version));
  }

  public static ModuleVersion decode(byte[] data) {
    ModuleVersion obj = new ModuleVersion();
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
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.version = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
