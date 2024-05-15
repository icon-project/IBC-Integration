package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class SimpleValidator extends ProtoMessage {
  private PublicKey pubKey = new PublicKey();

  private BigInteger votingPower = BigInteger.ZERO;

  public PublicKey getPubKey() {
    return this.pubKey;
  }

  public void setPubKey(PublicKey pubKey) {
    this.pubKey = pubKey;
  }

  public BigInteger getVotingPower() {
    return this.votingPower;
  }

  public void setVotingPower(BigInteger votingPower) {
    this.votingPower = votingPower;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.pubKey),
      Proto.encode(2, this.votingPower));
  }

  public static SimpleValidator decode(byte[] data) {
    SimpleValidator obj = new SimpleValidator();
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
            obj.pubKey = PublicKey.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.votingPower = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
