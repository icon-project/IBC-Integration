package ibc.core.client.v1;

import cosmos.upgrade.v1beta1.Plan;
import google.protobuf.Any;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class UpgradeProposal extends ProtoMessage {
  private String title = "";

  private String description = "";

  private Plan plan = new cosmos.upgrade.v1beta1.Plan();

  private Any upgradedClientState = new google.protobuf.Any();

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

  public Plan getPlan() {
    return this.plan;
  }

  public void setPlan(Plan plan) {
    this.plan = plan;
  }

  public Any getUpgradedClientState() {
    return this.upgradedClientState;
  }

  public void setUpgradedClientState(Any upgradedClientState) {
    this.upgradedClientState = upgradedClientState;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.title),
      Proto.encode(2, this.description),
      Proto.encode(3, this.plan),
      Proto.encode(4, this.upgradedClientState));
  }

  public static UpgradeProposal decode(byte[] data) {
    UpgradeProposal obj = new UpgradeProposal();
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
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.plan = cosmos.upgrade.v1beta1.Plan.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.upgradedClientState = google.protobuf.Any.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
