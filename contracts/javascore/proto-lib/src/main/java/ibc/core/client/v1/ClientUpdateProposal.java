package ibc.core.client.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class ClientUpdateProposal extends ProtoMessage {
  private String title = "";

  private String description = "";

  private String subjectClientId = "";

  private String substituteClientId = "";

  public String getTitle() {
    return this.title;
  }

  public void setTitle(String title) {
    this.title = title;
  }

  public String getDescription() {
    return this.description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public String getSubjectClientId() {
    return this.subjectClientId;
  }

  public void setSubjectClientId(String subjectClientId) {
    this.subjectClientId = subjectClientId;
  }

  public String getSubstituteClientId() {
    return this.substituteClientId;
  }

  public void setSubstituteClientId(String substituteClientId) {
    this.substituteClientId = substituteClientId;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.title),
      Proto.encode(2, this.description),
      Proto.encode(3, this.subjectClientId),
      Proto.encode(4, this.substituteClientId));
  }

  public static ClientUpdateProposal decode(byte[] data) {
    ClientUpdateProposal obj = new ClientUpdateProposal();
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
            obj.title = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.description = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.subjectClientId = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.substituteClientId = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
