package ibc.core.client.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class ClientConsensusStates extends ProtoMessage {
  private String clientId = "";

  private List<ConsensusStateWithHeight> consensusStates = new ArrayList<>();

  public String getClientId() {
    return this.clientId;
  }

  public void setClientId(String clientId) {
    this.clientId = clientId;
  }

  public List<ConsensusStateWithHeight> getConsensusStates() {
    return this.consensusStates;
  }

  public void setConsensusStates(List<ConsensusStateWithHeight> consensusStates) {
    this.consensusStates = consensusStates;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.clientId),
      Proto.encodeMessageArray(2, this.consensusStates));
  }

  public static ClientConsensusStates decode(byte[] data) {
    ClientConsensusStates obj = new ClientConsensusStates();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.clientId = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.consensusStates.add(ConsensusStateWithHeight.decode(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
