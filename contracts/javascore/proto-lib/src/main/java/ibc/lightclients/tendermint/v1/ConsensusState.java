package ibc.lightclients.tendermint.v1;

import google.protobuf.Timestamp;
import ibc.core.commitment.v1.MerkleRoot;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class ConsensusState extends ProtoMessage {
  private Timestamp timestamp = new google.protobuf.Timestamp();

  private MerkleRoot root = new ibc.core.commitment.v1.MerkleRoot();

  private byte[] nextValidatorsHash = new byte[0];

  public Timestamp getTimestamp() {
    return this.timestamp;
  }

  public void setTimestamp(Timestamp timestamp) {
    this.timestamp = timestamp;
  }

  public MerkleRoot getRoot() {
    return this.root;
  }

  public void setRoot(MerkleRoot root) {
    this.root = root;
  }

  public byte[] getNextValidatorsHash() {
    return this.nextValidatorsHash;
  }

  public void setNextValidatorsHash(byte[] nextValidatorsHash) {
    this.nextValidatorsHash = nextValidatorsHash;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.timestamp),
      Proto.encode(2, this.root),
      Proto.encode(3, this.nextValidatorsHash));
  }

  public static ConsensusState decode(byte[] data) {
    ConsensusState obj = new ConsensusState();
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
            obj.timestamp = google.protobuf.Timestamp.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.root = ibc.core.commitment.v1.MerkleRoot.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nextValidatorsHash = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
