package ibc.lightclients.tendermint.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class Fraction extends ProtoMessage {
  private BigInteger numerator = BigInteger.ZERO;

  private BigInteger denominator = BigInteger.ZERO;

  public BigInteger getNumerator() {
    return this.numerator;
  }

  public void setNumerator(BigInteger numerator) {
    this.numerator = numerator;
  }

  public BigInteger getDenominator() {
    return this.denominator;
  }

  public void setDenominator(BigInteger denominator) {
    this.denominator = denominator;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.numerator),
      Proto.encode(2, this.denominator));
  }

  public static Fraction decode(byte[] data) {
    Fraction obj = new Fraction();
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
            obj.numerator = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.denominator = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
