package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class Proof extends ProtoMessage {
  private BigInteger total = BigInteger.ZERO;

  private BigInteger index = BigInteger.ZERO;

  private byte[] leafHash = new byte[0];

  private List<byte[]> aunts = new ArrayList<>();

  public BigInteger getTotal() {
    return this.total;
  }

  public void setTotal(BigInteger total) {
    this.total = total;
  }

  public BigInteger getIndex() {
    return this.index;
  }

  public void setIndex(BigInteger index) {
    this.index = index;
  }

  public byte[] getLeafHash() {
    return this.leafHash;
  }

  public void setLeafHash(byte[] leafHash) {
    this.leafHash = leafHash;
  }

  public List<byte[]> getAunts() {
    return this.aunts;
  }

  public void setAunts(List<byte[]> aunts) {
    this.aunts = aunts;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.total),
      Proto.encode(2, this.index),
      Proto.encode(3, this.leafHash),
      Proto.encodeBytesArray(4, this.aunts));
  }

  public static Proof decode(byte[] data) {
    Proof obj = new Proof();
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
            obj.total = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.index = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.leafHash = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.aunts.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
