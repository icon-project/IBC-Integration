package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;

public class DominoOp extends ProtoMessage {
  private String key = "";

  private String input = "";

  private String output = "";

  public String getKey() {
    return this.key;
  }

  public void setKey(String key) {
    this.key = key;
  }

  public String getInput() {
    return this.input;
  }

  public void setInput(String input) {
    this.input = input;
  }

  public String getOutput() {
    return this.output;
  }

  public void setOutput(String output) {
    this.output = output;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.key),
      Proto.encode(2, this.input),
      Proto.encode(3, this.output));
  }

  public static DominoOp decode(byte[] data) {
    DominoOp obj = new DominoOp();
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
            obj.key = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.input = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.output = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
