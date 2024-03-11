package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class ValueOp extends ProtoMessage {
  private byte[] key = new byte[0];

  private Proof proof = new Proof();

  public byte[] getKey() {
    return this.key;
  }

  public void setKey(byte[] key) {
    this.key = key;
  }

  public Proof getProof() {
    return this.proof;
  }

  public void setProof(Proof proof) {
    this.proof = proof;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.key),
      Proto.encode(2, this.proof));
  }

  public static ValueOp decode(byte[] data) {
    ValueOp obj = new ValueOp();
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
            obj.key = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.proof = Proof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
