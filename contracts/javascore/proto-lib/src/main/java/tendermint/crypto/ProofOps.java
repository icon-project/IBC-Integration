package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class ProofOps extends ProtoMessage {
  private List<ProofOp> ops = new ArrayList<>();

  public List<ProofOp> getOps() {
    return this.ops;
  }

  public void setOps(List<ProofOp> ops) {
    this.ops = ops;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.ops));
  }

  public static ProofOps decode(byte[] data) {
    ProofOps obj = new ProofOps();
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
            obj.ops.add(ProofOp.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
