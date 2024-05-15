package tendermint.types;

import google.protobuf.Duration;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class EvidenceParams extends ProtoMessage {
  private BigInteger maxAgeNumBlocks = BigInteger.ZERO;

  private Duration maxAgeDuration = new google.protobuf.Duration();

  private BigInteger maxBytes = BigInteger.ZERO;

  public BigInteger getMaxAgeNumBlocks() {
    return this.maxAgeNumBlocks;
  }

  public void setMaxAgeNumBlocks(BigInteger maxAgeNumBlocks) {
    this.maxAgeNumBlocks = maxAgeNumBlocks;
  }

  public Duration getMaxAgeDuration() {
    return this.maxAgeDuration;
  }

  public void setMaxAgeDuration(Duration maxAgeDuration) {
    this.maxAgeDuration = maxAgeDuration;
  }

  public BigInteger getMaxBytes() {
    return this.maxBytes;
  }

  public void setMaxBytes(BigInteger maxBytes) {
    this.maxBytes = maxBytes;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.maxAgeNumBlocks),
      Proto.encode(2, this.maxAgeDuration),
      Proto.encode(3, this.maxBytes));
  }

  public static EvidenceParams decode(byte[] data) {
    EvidenceParams obj = new EvidenceParams();
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
            obj.maxAgeNumBlocks = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.maxAgeDuration = google.protobuf.Duration.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.maxBytes = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
