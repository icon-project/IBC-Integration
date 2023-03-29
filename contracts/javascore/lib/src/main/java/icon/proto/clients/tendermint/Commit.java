package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class Commit extends ProtoMessage {
  private BigInteger height = BigInteger.ZERO;

  private BigInteger round = BigInteger.ZERO;

  private BlockID blockId;

  private List<CommitSig> signatures = new ArrayList<>();

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

  public List<CommitSig> getSignatures() {
    return this.signatures;
  }

  public void setSignatures(List<CommitSig> signatures) {
    this.signatures = signatures;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.height),
      Proto.encode(2, this.round),
      Proto.encode(3, this.blockId),
      Proto.encodeMessageArray(4, this.signatures));
  }

  public static Commit decode(byte[] data) {
    Commit obj = new Commit();
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
            obj.height = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.round = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.blockId = BlockID.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.signatures.add(CommitSig.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
