package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import tendermint.crypto.Proof;

public class TxProof extends ProtoMessage {
  private byte[] rootHash = new byte[0];

  private byte[] data = new byte[0];

  private Proof proof = new tendermint.crypto.Proof();

  public byte[] getRootHash() {
    return this.rootHash;
  }

  public void setRootHash(byte[] rootHash) {
    this.rootHash = rootHash;
  }

  public byte[] getData() {
    return this.data;
  }

  public void setData(byte[] data) {
    this.data = data;
  }

  public Proof getProof() {
    return this.proof;
  }

  public void setProof(Proof proof) {
    this.proof = proof;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.rootHash),
      Proto.encode(2, this.data),
      Proto.encode(3, this.proof));
  }

  public static TxProof decode(byte[] data) {
    TxProof obj = new TxProof();
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
            obj.rootHash = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.data = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.proof = tendermint.crypto.Proof.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
