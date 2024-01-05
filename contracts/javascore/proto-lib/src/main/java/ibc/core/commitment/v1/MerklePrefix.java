package ibc.core.commitment.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class MerklePrefix extends ProtoMessage {
  private byte[] keyPrefix = new byte[0];

  public byte[] getKeyPrefix() {
    return this.keyPrefix;
  }

  public void setKeyPrefix(byte[] keyPrefix) {
    this.keyPrefix = keyPrefix;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.keyPrefix));
  }

  public static MerklePrefix decode(byte[] data) {
    MerklePrefix obj = new MerklePrefix();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.keyPrefix = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
