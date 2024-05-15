package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class CanonicalVoteExtension extends ProtoMessage {
  private byte[] extension = new byte[0];

  private BigInteger height = BigInteger.ZERO;

  private BigInteger round = BigInteger.ZERO;

  private String chainId = "";

  public byte[] getExtension() {
    return this.extension;
  }

  public void setExtension(byte[] extension) {
    this.extension = extension;
  }

  public BigInteger getHeight() {
    return this.height;
  }

  public void setHeight(BigInteger height) {
    this.height = height;
  }

  public BigInteger getRound() {
    return this.round;
  }

  public void setRound(BigInteger round) {
    this.round = round;
  }

  public String getChainId() {
    return this.chainId;
  }

  public void setChainId(String chainId) {
    this.chainId = chainId;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.extension),
      Proto.encodeFixed64(2, this.height),
      Proto.encodeFixed64(3, this.round),
      Proto.encode(4, this.chainId));
  }

  public static CanonicalVoteExtension decode(byte[] data) {
    CanonicalVoteExtension obj = new CanonicalVoteExtension();
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
            obj.extension = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeFixed64(data, index);
            index = resp.index;
            obj.height = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeFixed64(data, index);
            index = resp.index;
            obj.round = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.chainId = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
