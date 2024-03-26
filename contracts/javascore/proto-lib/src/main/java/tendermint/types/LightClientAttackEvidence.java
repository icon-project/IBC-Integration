package tendermint.types;

import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class LightClientAttackEvidence extends ProtoMessage {
  private LightBlock conflictingBlock = new tendermint.types.LightBlock();

  private BigInteger commonHeight = BigInteger.ZERO;

  private List<Validator> byzantineValidators = new ArrayList<>();

  private BigInteger totalVotingPower = BigInteger.ZERO;

  private Timestamp timestamp = new google.protobuf.Timestamp();

  public LightBlock getConflictingBlock() {
    return this.conflictingBlock;
  }

  public void setConflictingBlock(LightBlock conflictingBlock) {
    this.conflictingBlock = conflictingBlock;
  }

  public BigInteger getCommonHeight() {
    return this.commonHeight;
  }

  public void setCommonHeight(BigInteger commonHeight) {
    this.commonHeight = commonHeight;
  }

  public List<Validator> getByzantineValidators() {
    return this.byzantineValidators;
  }

  public void setByzantineValidators(List<Validator> byzantineValidators) {
    this.byzantineValidators = byzantineValidators;
  }

  public BigInteger getTotalVotingPower() {
    return this.totalVotingPower;
  }

  public void setTotalVotingPower(BigInteger totalVotingPower) {
    this.totalVotingPower = totalVotingPower;
  }

  public Timestamp getTimestamp() {
    return this.timestamp;
  }

  public void setTimestamp(Timestamp timestamp) {
    this.timestamp = timestamp;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.conflictingBlock),
      Proto.encode(2, this.commonHeight),
      Proto.encodeMessageArray(3, this.byzantineValidators),
      Proto.encode(4, this.totalVotingPower),
      Proto.encode(5, this.timestamp));
  }

  public static LightClientAttackEvidence decode(byte[] data) {
    LightClientAttackEvidence obj = new LightClientAttackEvidence();
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
            obj.conflictingBlock = tendermint.types.LightBlock.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.commonHeight = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.byzantineValidators.add(tendermint.types.Validator.decode(resp.res));
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.totalVotingPower = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.timestamp = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
