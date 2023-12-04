package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import tendermint.crypto.PublicKey;

public class Validator extends ProtoMessage {
  private byte[] address = new byte[0];

  private PublicKey pubKey = new tendermint.crypto.PublicKey();

  private BigInteger votingPower = BigInteger.ZERO;

  private BigInteger proposerPriority = BigInteger.ZERO;

  public byte[] getAddress() {
    return this.address;
  }

  public void setAddress(byte[] address) {
    this.address = address;
  }

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

  public BigInteger getProposerPriority() {
    return this.proposerPriority;
  }

  public void setProposerPriority(BigInteger proposerPriority) {
    this.proposerPriority = proposerPriority;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.address),
      Proto.encode(2, this.pubKey),
      Proto.encode(3, this.votingPower),
      Proto.encode(4, this.proposerPriority));
  }

  public static Validator decode(byte[] data) {
    Validator obj = new Validator();
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
            obj.address = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.pubKey = tendermint.crypto.PublicKey.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.votingPower = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.proposerPriority = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
