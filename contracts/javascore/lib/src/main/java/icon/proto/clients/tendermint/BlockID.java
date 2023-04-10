package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class BlockID extends ProtoMessage {
  private byte[] hash = new byte[0];

  private PartSetHeader partSetHeader = new PartSetHeader();

  public byte[] getHash() {
    return this.hash;
  }

  public void setHash(byte[] hash) {
    this.hash = hash;
  }

  public PartSetHeader getPartSetHeader() {
    return this.partSetHeader;
  }

  public void setPartSetHeader(PartSetHeader partSetHeader) {
    this.partSetHeader = partSetHeader;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.hash),
      Proto.encode(2, this.partSetHeader));
  }

  public static BlockID decode(byte[] data) {
    BlockID obj = new BlockID();
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
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.partSetHeader = PartSetHeader.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
