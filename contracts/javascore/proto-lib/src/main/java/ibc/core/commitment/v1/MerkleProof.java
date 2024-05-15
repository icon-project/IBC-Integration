package ibc.core.commitment.v1;

import cosmos.ics23.v1.CommitmentProof;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class MerkleProof extends ProtoMessage {
  private List<CommitmentProof> proofs = new ArrayList<>();

  public List<CommitmentProof> getProofs() {
    return this.proofs;
  }

  public void setProofs(List<CommitmentProof> proofs) {
    this.proofs = proofs;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.proofs));
  }

  public static MerkleProof decode(byte[] data) {
    MerkleProof obj = new MerkleProof();
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
            obj.proofs.add(cosmos.ics23.v1.CommitmentProof.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
