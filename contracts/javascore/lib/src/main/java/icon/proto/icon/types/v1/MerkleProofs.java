package icon.proto.icon.types.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class MerkleProofs extends ProtoMessage {
  private List<MerkleNode> proofs = new ArrayList<>();

  public List<MerkleNode> getProofs() {
    return this.proofs;
  }

  public void setProofs(List<MerkleNode> proofs) {
    this.proofs = proofs;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.proofs));
  }

  public static MerkleProofs decode(byte[] data) {
    MerkleProofs obj = new MerkleProofs();
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
            obj.proofs.add(MerkleNode.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
