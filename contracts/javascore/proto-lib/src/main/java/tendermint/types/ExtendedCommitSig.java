package tendermint.types;

import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;

public class ExtendedCommitSig extends ProtoMessage {
  private int blockIdFlag = 0;

  private byte[] validatorAddress = new byte[0];

  private Timestamp timestamp = new google.protobuf.Timestamp();

  private byte[] signature = new byte[0];

  private byte[] extension = new byte[0];

  private byte[] extensionSignature = new byte[0];

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

  public byte[] getExtension() {
    return this.extension;
  }

  public void setExtension(byte[] extension) {
    this.extension = extension;
  }

  public byte[] getExtensionSignature() {
    return this.extensionSignature;
  }

  public void setExtensionSignature(byte[] extensionSignature) {
    this.extensionSignature = extensionSignature;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.blockIdFlag),
      Proto.encode(2, this.validatorAddress),
      Proto.encode(3, this.timestamp),
      Proto.encode(4, this.signature),
      Proto.encode(5, this.extension),
      Proto.encode(6, this.extensionSignature));
  }

  public static ExtendedCommitSig decode(byte[] data) {
    ExtendedCommitSig obj = new ExtendedCommitSig();
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
            obj.timestamp = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.signature = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.extension = resp.res;
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.extensionSignature = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
