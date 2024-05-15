package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.util.List;
import scorex.util.ArrayList;

public class Data extends ProtoMessage {
  private List<byte[]> txs = new ArrayList<>();

  public List<byte[]> getTxs() {
    return this.txs;
  }

  public void setTxs(List<byte[]> txs) {
    this.txs = txs;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeBytesArray(1, this.txs));
  }

  public static Data decode(byte[] data) {
    Data obj = new Data();
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
            obj.txs.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
