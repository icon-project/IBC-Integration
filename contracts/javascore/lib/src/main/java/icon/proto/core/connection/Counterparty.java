package icon.proto.core.connection;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class Counterparty extends ProtoMessage {
  private String clientId = "";

  private String connectionId = "";

  private MerklePrefix prefix;

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public String getConnectionId() {
    return this.connectionId;
  }

  public void setConnectionId(String connectionId) {
    this.connectionId = connectionId;
  }

  public MerklePrefix getPrefix() {
    return this.prefix;
  }

  public void setPrefix(MerklePrefix prefix) {
    this.prefix = prefix;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.clientId),
      Proto.encode(2, this.connectionId),
      Proto.encode(3, this.prefix));
  }

  public static Counterparty decode(byte[] data) {
    Counterparty obj = new Counterparty();
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
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.connectionId = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.prefix = MerklePrefix.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
