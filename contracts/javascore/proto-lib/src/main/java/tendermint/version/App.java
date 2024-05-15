package tendermint.version;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class App extends ProtoMessage {
  private BigInteger protocol = BigInteger.ZERO;

  private String software = "";

  public BigInteger getProtocol() {
    return this.protocol;
  }

  public void setProtocol(BigInteger protocol) {
    this.protocol = protocol;
  }

  public String getSoftware() {
    return this.software;
  }

  public void setSoftware(String software) {
    this.software = software;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.protocol),
      Proto.encode(2, this.software));
  }

  public static App decode(byte[] data) {
    App obj = new App();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.protocol = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.software = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
