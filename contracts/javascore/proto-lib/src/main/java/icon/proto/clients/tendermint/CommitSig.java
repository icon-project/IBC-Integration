package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;

public class CommitSig extends ProtoMessage {
  private int blockIdFlag = 0;

  private byte[] validatorAddress = new byte[0];

  private Timestamp timestamp = new Timestamp();

  private byte[] signature = new byte[0];

  public int getBlockIdFlag() {
    return this.blockIdFlag;
  }

  public void setBlockIdFlag(int blockIdFlag) {
    this.blockIdFlag = blockIdFlag;
  }

  public byte[] getValidatorAddress() {
    return this.validatorAddress;
  }

  public void setValidatorAddress(byte[] validatorAddress) {
    this.validatorAddress = validatorAddress;
  }

  public Timestamp getTimestamp() {
    return this.timestamp;
  }

  public void setTimestamp(Timestamp timestamp) {
    this.timestamp = timestamp;
  }

  public byte[] getSignature() {
    return this.signature;
  }

  public void setSignature(byte[] signature) {
    this.signature = signature;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.blockIdFlag),
      Proto.encode(2, this.validatorAddress),
      Proto.encode(3, this.timestamp),
      Proto.encode(4, this.signature));
  }

  public static CommitSig decode(byte[] data) {
    CommitSig obj = new CommitSig();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.blockIdFlag = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validatorAddress = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.timestamp = Timestamp.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.signature = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
