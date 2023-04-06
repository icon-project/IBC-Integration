package icon.proto.core.connection;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.lang.String;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class ConnectionEnd extends ProtoMessage {
  private String clientId = "";

  private List<Version> versions = new ArrayList<>();

  private int state = 0;

  private Counterparty counterparty;

  private BigInteger delayPeriod = BigInteger.ZERO;

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public List<Version> getVersions() {
    return this.versions;
  }

  public void setVersions(List<Version> versions) {
    this.versions = versions;
  }

  public int getState() {
    return this.state;
  }

  public void setState(int state) {
    this.state = state;
  }

  public Counterparty getCounterparty() {
    return this.counterparty;
  }

  public void setCounterparty(Counterparty counterparty) {
    this.counterparty = counterparty;
  }

  public BigInteger getDelayPeriod() {
    return this.delayPeriod;
  }

  public void setDelayPeriod(BigInteger delayPeriod) {
    this.delayPeriod = delayPeriod;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.clientId),
      Proto.encodeMessageArray(2, this.versions),
      Proto.encode(3, this.state),
      Proto.encode(4, this.counterparty),
      Proto.encode(5, this.delayPeriod));
  }

  public static ConnectionEnd decode(byte[] data) {
    ConnectionEnd obj = new ConnectionEnd();
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
            obj.versions.add(Version.decode(resp.res));
            break;
        }
        case 3: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.state = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.counterparty = Counterparty.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.delayPeriod = resp.res;
            break;
        }
      }
    }
    return obj;
  }

  public static class State {
    public static final int STATE_UNINITIALIZED_UNSPECIFIED = 0;

    public static final int STATE_INIT = 1;

    public static final int STATE_TRYOPEN = 2;

    public static final int STATE_OPEN = 3;
  }
}
