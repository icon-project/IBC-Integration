package ibc.core.client.v1;

import google.protobuf.Any;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class ConsensusStateWithHeight extends ProtoMessage {
  private Height height = new Height();

  private Any consensusState = new google.protobuf.Any();

  public Height getHeight() {
    return this.height;
  }

  public void setHeight(Height height) {
    this.height = height;
  }

  public Any getConsensusState() {
    return this.consensusState;
  }

  public void setConsensusState(Any consensusState) {
    this.consensusState = consensusState;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.height),
      Proto.encode(2, this.consensusState));
  }

  public static ConsensusStateWithHeight decode(byte[] data) {
    ConsensusStateWithHeight obj = new ConsensusStateWithHeight();
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
            obj.height = Height.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.consensusState = google.protobuf.Any.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
