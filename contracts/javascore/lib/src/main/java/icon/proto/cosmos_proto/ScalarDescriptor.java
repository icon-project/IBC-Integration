package cosmos_proto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.lang.String;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class ScalarDescriptor extends ProtoMessage {
  private String name = "";

  private String description = "";

  private List<BigInteger> fieldType = new ArrayList<>();

  public String getName() {
    return this.name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public String getDescription() {
    return this.description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public List<BigInteger> getFieldType() {
    return this.fieldType;
  }

  public void setFieldType(List<BigInteger> fieldType) {
    this.fieldType = fieldType;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.name),
      Proto.encode(2, this.description),
      Proto.encodeVarIntArray(3, this.fieldType));
  }

  public static ScalarDescriptor decode(byte[] data) {
    ScalarDescriptor obj = new ScalarDescriptor();
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
            obj.name = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.description = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.fieldType.add(BigInteger.valueOf(resp.res));
            break;
        }
      }
    }
    return obj;
  }
}
