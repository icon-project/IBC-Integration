package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class ValidatorParams extends ProtoMessage {
  private List<String> pubKeyTypes = new ArrayList<>();

  public List<String> getPubKeyTypes() {
    return this.pubKeyTypes;
  }

  public void setPubKeyTypes(List<String> pubKeyTypes) {
    this.pubKeyTypes = pubKeyTypes;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encodeStringArray(1, this.pubKeyTypes));
  }

  public static ValidatorParams decode(byte[] data) {
    ValidatorParams obj = new ValidatorParams();
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
            obj.pubKeyTypes.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
