package tendermint.types;

import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class DuplicateVoteEvidence extends ProtoMessage {
  private Vote voteA = new tendermint.types.Vote();

  private Vote voteB = new tendermint.types.Vote();

  private BigInteger totalVotingPower = BigInteger.ZERO;

  private BigInteger validatorPower = BigInteger.ZERO;

  private Timestamp timestamp = new google.protobuf.Timestamp();

  public Vote getVoteA() {
    return this.voteA;
  }

  public void setVoteA(Vote voteA) {
    this.voteA = voteA;
  }

  public Vote getVoteB() {
    return this.voteB;
  }

  public void setVoteB(Vote voteB) {
    this.voteB = voteB;
  }

  public BigInteger getTotalVotingPower() {
    return this.totalVotingPower;
  }

  public void setTotalVotingPower(BigInteger totalVotingPower) {
    this.totalVotingPower = totalVotingPower;
  }

  public BigInteger getValidatorPower() {
    return this.validatorPower;
  }

  public void setValidatorPower(BigInteger validatorPower) {
    this.validatorPower = validatorPower;
  }

  public Timestamp getTimestamp() {
    return this.timestamp;
  }

  public void setTimestamp(Timestamp timestamp) {
    this.timestamp = timestamp;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.voteA),
      Proto.encode(2, this.voteB),
      Proto.encode(3, this.totalVotingPower),
      Proto.encode(4, this.validatorPower),
      Proto.encode(5, this.timestamp));
  }

  public static DuplicateVoteEvidence decode(byte[] data) {
    DuplicateVoteEvidence obj = new DuplicateVoteEvidence();
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
            obj.voteA = tendermint.types.Vote.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.voteB = tendermint.types.Vote.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.totalVotingPower = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.validatorPower = resp.res;
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
