package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class ValidatorSet extends ProtoMessage {
  private List<Validator> validators = new ArrayList<>();

  private Validator proposer = new Validator();

  private BigInteger totalVotingPower = BigInteger.ZERO;

  public List<Validator> getValidators() {
    return this.validators;
  }

  public void setValidators(List<Validator> validators) {
    this.validators = validators;
  }

  public Validator getProposer() {
    return this.proposer;
  }

  public void setProposer(Validator proposer) {
    this.proposer = proposer;
  }

  public BigInteger getTotalVotingPower() {
    return this.totalVotingPower;
  }

  public void setTotalVotingPower(BigInteger totalVotingPower) {
    this.totalVotingPower = totalVotingPower;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeMessageArray(1, this.validators),
      Proto.encode(2, this.proposer),
      Proto.encode(3, this.totalVotingPower));
  }

  public static ValidatorSet decode(byte[] data) {
    ValidatorSet obj = new ValidatorSet();
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
            obj.validators.add(Validator.decode(resp.res));
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.proposer = Validator.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.totalVotingPower = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
