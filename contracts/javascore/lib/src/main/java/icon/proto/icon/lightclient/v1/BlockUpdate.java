package icon.proto.icon.lightclient.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import icon.proto.icon.types.v1.SignedHeader;

public class BlockUpdate extends ProtoMessage {
  private SignedHeader header = new SignedHeader();

  public SignedHeader getHeader() {
    return this.header;
  }

  public void setHeader(SignedHeader header) {
    this.header = header;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.header));
  }

  public static BlockUpdate decode(byte[] data) {
    BlockUpdate obj = new BlockUpdate();
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
            obj.header = SignedHeader.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
