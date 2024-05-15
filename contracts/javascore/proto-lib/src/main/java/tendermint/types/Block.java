package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class Block extends ProtoMessage {
  private Header header = new Header();

  private Data data = new Data();

  private EvidenceList evidence = new tendermint.types.EvidenceList();

  private Commit lastCommit = new Commit();

  public Header getHeader() {
    return this.header;
  }

  public void setHeader(Header header) {
    this.header = header;
  }

  public Data getData() {
    return this.data;
  }

  public void setData(Data data) {
    this.data = data;
  }

  public EvidenceList getEvidence() {
    return this.evidence;
  }

  public void setEvidence(EvidenceList evidence) {
    this.evidence = evidence;
  }

  public Commit getLastCommit() {
    return this.lastCommit;
  }

  public void setLastCommit(Commit lastCommit) {
    this.lastCommit = lastCommit;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.header),
      Proto.encode(2, this.data),
      Proto.encode(3, this.evidence),
      Proto.encode(4, this.lastCommit));
  }

  public static Block decode(byte[] data) {
    Block obj = new Block();
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
            obj.header = Header.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.data = Data.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.evidence = tendermint.types.EvidenceList.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lastCommit = Commit.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
