package cosmos.upgrade.v1beta1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class CancelSoftwareUpgradeProposal extends ProtoMessage {
  private String title = "";

  private String description = "";

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

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.title),
      Proto.encode(2, this.description));
  }

  public static CancelSoftwareUpgradeProposal decode(byte[] data) {
    CancelSoftwareUpgradeProposal obj = new CancelSoftwareUpgradeProposal();
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
      }
    }
    return obj;
  }
}
