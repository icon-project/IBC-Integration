package tendermint.types;

import google.protobuf.Timestamp;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.math.BigInteger;

public class Proposal extends ProtoMessage {
  private int type = 0;

  private BigInteger height = BigInteger.ZERO;

  private BigInteger round = BigInteger.ZERO;

  private BigInteger polRound = BigInteger.ZERO;

  private BlockID blockId = new BlockID();

  private Timestamp timestamp = new google.protobuf.Timestamp();

  private byte[] signature = new byte[0];

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

  public BigInteger getPolRound() {
    return this.polRound;
  }

  public void setPolRound(BigInteger polRound) {
    this.polRound = polRound;
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

  public byte[] getSignature() {
    return this.signature;
  }

  public void setSignature(byte[] signature) {
    this.signature = signature;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.type),
      Proto.encode(2, this.height),
      Proto.encode(3, this.round),
      Proto.encode(4, this.polRound),
      Proto.encode(5, this.blockId),
      Proto.encode(6, this.timestamp),
      Proto.encode(7, this.signature));
  }

  public static Proposal decode(byte[] data) {
    Proposal obj = new Proposal();
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
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.polRound = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.blockId = BlockID.decode(resp.res);
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.timestamp = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
        case 7: {
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
