package icon.proto.icon.lightclient.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class ConsensusState extends ProtoMessage {
  private byte[] messageRoot = new byte[0];

  public byte[] getMessageRoot() {
    return this.messageRoot;
  }

  public void setMessageRoot(byte[] messageRoot) {
    this.messageRoot = messageRoot;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.messageRoot));
  }

  public static ConsensusState decode(byte[] data) {
    ConsensusState obj = new ConsensusState();
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
            obj.messageRoot = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
