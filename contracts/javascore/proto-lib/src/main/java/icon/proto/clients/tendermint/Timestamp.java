package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class Timestamp extends ProtoMessage {
  private BigInteger seconds = BigInteger.ZERO;

  private BigInteger nanos = BigInteger.ZERO;

  public BigInteger getSeconds() {
    return this.seconds;
  }

  public void setSeconds(BigInteger seconds) {
    this.seconds = seconds;
  }

  public BigInteger getNanos() {
    return this.nanos;
  }

  public void setNanos(BigInteger nanos) {
    this.nanos = nanos;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.seconds),
      Proto.encode(2, this.nanos));
  }

  public static Timestamp decode(byte[] data) {
    Timestamp obj = new Timestamp();
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
            obj.seconds = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.nanos = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
