package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class BlockMeta extends ProtoMessage {
  private BlockID blockId = new BlockID();

  private BigInteger blockSize = BigInteger.ZERO;

  private Header header = new Header();

  private BigInteger numTxs = BigInteger.ZERO;

  public BlockID getBlockId() {
    return this.blockId;
  }

  public void setBlockId(BlockID blockId) {
    this.blockId = blockId;
  }

  public BigInteger getBlockSize() {
    return this.blockSize;
  }

  public void setBlockSize(BigInteger blockSize) {
    this.blockSize = blockSize;
  }

  public Header getHeader() {
    return this.header;
  }

  public void setHeader(Header header) {
    this.header = header;
  }

  public BigInteger getNumTxs() {
    return this.numTxs;
  }

  public void setNumTxs(BigInteger numTxs) {
    this.numTxs = numTxs;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.blockId),
      Proto.encode(2, this.blockSize),
      Proto.encode(3, this.header),
      Proto.encode(4, this.numTxs));
  }

  public static BlockMeta decode(byte[] data) {
    BlockMeta obj = new BlockMeta();
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
            obj.blockId = BlockID.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.blockSize = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.header = Header.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.numTxs = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
