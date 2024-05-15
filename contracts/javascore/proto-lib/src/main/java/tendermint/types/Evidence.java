package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class Evidence extends ProtoMessage {
  private DuplicateVoteEvidence duplicateVoteEvidence = new DuplicateVoteEvidence();

  private LightClientAttackEvidence lightClientAttackEvidence = new LightClientAttackEvidence();

  public DuplicateVoteEvidence getDuplicateVoteEvidence() {
    return this.duplicateVoteEvidence;
  }

  public void setDuplicateVoteEvidence(DuplicateVoteEvidence duplicateVoteEvidence) {
    this.duplicateVoteEvidence = duplicateVoteEvidence;
  }

  public LightClientAttackEvidence getLightClientAttackEvidence() {
    return this.lightClientAttackEvidence;
  }

  public void setLightClientAttackEvidence(LightClientAttackEvidence lightClientAttackEvidence) {
    this.lightClientAttackEvidence = lightClientAttackEvidence;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.duplicateVoteEvidence),
      Proto.encode(2, this.lightClientAttackEvidence));
  }

  public static Evidence decode(byte[] data) {
    Evidence obj = new Evidence();
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
            obj.duplicateVoteEvidence = DuplicateVoteEvidence.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lightClientAttackEvidence = LightClientAttackEvidence.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
