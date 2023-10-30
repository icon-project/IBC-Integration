package icon.proto.core.client;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class Height extends ProtoMessage {
  private BigInteger revisionNumber = BigInteger.ZERO;

  private BigInteger revisionHeight = BigInteger.ZERO;

  public Height(BigInteger revisionNumber, BigInteger revisionHeight){
    this.revisionNumber=revisionNumber;
    this.revisionHeight=revisionHeight;
  }

  public Height(){
  }

  public BigInteger getRevisionNumber() {
    return this.revisionNumber;
  }

  public void setRevisionNumber(BigInteger revisionNumber) {
    this.revisionNumber = revisionNumber;
  }

  public BigInteger getRevisionHeight() {
    return this.revisionHeight;
  }

  public void setRevisionHeight(BigInteger revisionHeight) {
    this.revisionHeight = revisionHeight;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.revisionNumber),
      Proto.encode(2, this.revisionHeight));
  }

  public static Height decode(byte[] data) {
    Height obj = new Height();
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
            obj.revisionNumber = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.revisionHeight = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
