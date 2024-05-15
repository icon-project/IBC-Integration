package icon.proto.core.channel;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class PacketState extends ProtoMessage {
  private String portId = "";

  private String channelId = "";

  private BigInteger sequence = BigInteger.ZERO;

  private byte[] data = new byte[0];

  public String getPortId() {
    return this.portId;
  }

  public void setPortId(String portId) {
    this.portId = portId;
  }

  public String getChannelId() {
    return this.channelId;
  }

  public void setChannelId(String channelId) {
    this.channelId = channelId;
  }

  public BigInteger getSequence() {
    return this.sequence;
  }

  public void setSequence(BigInteger sequence) {
    this.sequence = sequence;
  }

  public byte[] getData() {
    return this.data;
  }

  public void setData(byte[] data) {
    this.data = data;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.portId),
      Proto.encode(2, this.channelId),
      Proto.encode(3, this.sequence),
      Proto.encode(4, this.data));
  }

  public static PacketState decode(byte[] data) {
    PacketState obj = new PacketState();
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
            obj.portId = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.channelId = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.sequence = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.data = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
