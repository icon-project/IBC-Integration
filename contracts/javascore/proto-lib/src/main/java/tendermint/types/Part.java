package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import tendermint.crypto.Proof;

public class Part extends ProtoMessage {
  private BigInteger index = BigInteger.ZERO;

  private byte[] bytes = new byte[0];

  private Proof proof = new tendermint.crypto.Proof();

  public BigInteger getIndex() {
    return this.index;
  }

  public void setIndex(BigInteger index) {
    this.index = index;
  }

  public byte[] getBytes() {
    return this.bytes;
  }

  public void setBytes(byte[] bytes) {
    this.bytes = bytes;
  }

  public Proof getProof() {
    return this.proof;
  }

  public void setProof(Proof proof) {
    this.proof = proof;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.index),
      Proto.encode(2, this.bytes),
      Proto.encode(3, this.proof));
  }

  public static Part decode(byte[] data) {
    Part obj = new Part();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.index = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.bytes = resp.res;
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
