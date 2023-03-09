package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class MerkleRoot extends ProtoMessage {
  private byte[] hash = new byte[0];

  public byte[] getHash() {
    return this.hash;
  }

  public void setHash(byte[] hash) {
    this.hash = hash;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.hash));
  }

  public static MerkleRoot decode(byte[] data) {
    MerkleRoot obj = new MerkleRoot();
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
            obj.hash = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
