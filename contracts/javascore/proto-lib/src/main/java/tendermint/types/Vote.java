package tendermint.types;

import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.math.BigInteger;

public class Vote extends ProtoMessage {
  private int type = 0;

  private BigInteger height = BigInteger.ZERO;

  private BigInteger round = BigInteger.ZERO;

  private BlockID blockId = new BlockID();

  private Timestamp timestamp = new google.protobuf.Timestamp();

  private byte[] validatorAddress = new byte[0];

  private BigInteger validatorIndex = BigInteger.ZERO;

  private byte[] signature = new byte[0];

  private byte[] extension = new byte[0];

  private byte[] extensionSignature = new byte[0];

  public int getType() {
    return this.type;
  }

  public void setType(int type) {
    this.type = type;
  }

  public BigInteger getHeight() {
    return this.height;
  }

  public void setHeight(BigInteger height) {
    this.height = height;
  }

  public BigInteger getRound() {
    return this.round;
  }

  public void setRound(BigInteger round) {
    this.round = round;
  }

  public BlockID getBlockId() {
    return this.blockId;
  }

  public void setBlockId(BlockID blockId) {
    this.blockId = blockId;
  }

  public Timestamp getTimestamp() {
    return this.timestamp;
  }

  public void setTimestamp(Timestamp timestamp) {
    this.timestamp = timestamp;
  }

  public byte[] getValidatorAddress() {
    return this.validatorAddress;
  }

  public void setValidatorAddress(byte[] validatorAddress) {
    this.validatorAddress = validatorAddress;
  }

  public BigInteger getValidatorIndex() {
    return this.validatorIndex;
  }

  public void setValidatorIndex(BigInteger validatorIndex) {
    this.validatorIndex = validatorIndex;
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
      Proto.encode(1, this.type),
      Proto.encode(2, this.height),
      Proto.encode(3, this.round),
      Proto.encode(4, this.blockId),
      Proto.encode(5, this.timestamp),
      Proto.encode(6, this.validatorAddress),
      Proto.encode(7, this.validatorIndex),
      Proto.encode(8, this.signature),
      Proto.encode(9, this.extension),
      Proto.encode(10, this.extensionSignature));
  }

  public static Vote decode(byte[] data) {
    Vote obj = new Vote();
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
            obj.type = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.height = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.round = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.blockId = BlockID.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.timestamp = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validatorAddress = resp.res;
            break;
        }
        case 7: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.validatorIndex = resp.res;
            break;
        }
        case 8: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.signature = resp.res;
            break;
        }
        case 9: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.extension = resp.res;
            break;
        }
        case 10: {
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
