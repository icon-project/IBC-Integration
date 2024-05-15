package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class SignedHeader extends ProtoMessage {
  private LightHeader header = new LightHeader();

  private Commit commit = new Commit();

  public LightHeader getHeader() {
    return this.header;
  }

  public void setHeader(LightHeader header) {
    this.header = header;
  }

  public Commit getCommit() {
    return this.commit;
  }

  public void setCommit(Commit commit) {
    this.commit = commit;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.header),
      Proto.encode(2, this.commit));
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
            obj.header = LightHeader.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.commit = Commit.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
