package icon.proto.icon.types.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class SignedHeader extends ProtoMessage {
  private BTPHeader header = new BTPHeader();

  private List<byte[]> signatures = new ArrayList<>();

  public BTPHeader getHeader() {
    return this.header;
  }

  public void setHeader(BTPHeader header) {
    this.header = header;
  }

  public List<byte[]> getSignatures() {
    return this.signatures;
  }

  public void setSignatures(List<byte[]> signatures) {
    this.signatures = signatures;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.header),
      Proto.encodeBytesArray(2, this.signatures));
  }

  public static SignedHeader decode(byte[] data) {
    SignedHeader obj = new SignedHeader();
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
            obj.header = BTPHeader.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.signatures.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
