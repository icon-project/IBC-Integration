package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class ProofOp extends ProtoMessage {
  private String type = "";

  private byte[] key = new byte[0];

  private byte[] data = new byte[0];

  public String getType() {
    return this.type;
  }

  public void setType(String type) {
    this.type = type;
  }

  public byte[] getKey() {
    return this.key;
  }

  public void setKey(byte[] key) {
    this.key = key;
  }

  public byte[] getData() {
    return this.data;
  }

  public void setData(byte[] data) {
    this.data = data;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.type),
      Proto.encode(2, this.key),
      Proto.encode(3, this.data));
  }

  public static ProofOp decode(byte[] data) {
    ProofOp obj = new ProofOp();
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
            obj.type = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.key = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.data = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
