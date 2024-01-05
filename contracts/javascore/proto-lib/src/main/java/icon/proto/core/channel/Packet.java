package icon.proto.core.channel;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import icon.proto.core.client.Height;
import java.lang.String;
import java.math.BigInteger;

public class Packet extends ProtoMessage {
  private BigInteger sequence = BigInteger.ZERO;

  private String sourcePort = "";

  private String sourceChannel = "";

  private String destinationPort = "";

  private String destinationChannel = "";

  private byte[] data = new byte[0];

  private Height timeoutHeight = new icon.proto.core.client.Height();

  private BigInteger timeoutTimestamp = BigInteger.ZERO;

  public BigInteger getSequence() {
    return this.sequence;
  }

  public void setSequence(BigInteger sequence) {
    this.sequence = sequence;
  }

  public String getSourcePort() {
    return this.sourcePort;
  }

  public void setSourcePort(String sourcePort) {
    this.sourcePort = sourcePort;
  }

  public String getSourceChannel() {
    return this.sourceChannel;
  }

  public void setSourceChannel(String sourceChannel) {
    this.sourceChannel = sourceChannel;
  }

  public String getDestinationPort() {
    return this.destinationPort;
  }

  public void setDestinationPort(String destinationPort) {
    this.destinationPort = destinationPort;
  }

  public String getDestinationChannel() {
    return this.destinationChannel;
  }

  public void setDestinationChannel(String destinationChannel) {
    this.destinationChannel = destinationChannel;
  }

  public byte[] getData() {
    return this.data;
  }

  public void setData(byte[] data) {
    this.data = data;
  }

  public Height getTimeoutHeight() {
    return this.timeoutHeight;
  }

  public void setTimeoutHeight(Height timeoutHeight) {
    this.timeoutHeight = timeoutHeight;
  }

  public BigInteger getTimeoutTimestamp() {
    return this.timeoutTimestamp;
  }

  public void setTimeoutTimestamp(BigInteger timeoutTimestamp) {
    this.timeoutTimestamp = timeoutTimestamp;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.sequence),
      Proto.encode(2, this.sourcePort),
      Proto.encode(3, this.sourceChannel),
      Proto.encode(4, this.destinationPort),
      Proto.encode(5, this.destinationChannel),
      Proto.encode(6, this.data),
      Proto.encode(7, this.timeoutHeight),
      Proto.encode(8, this.timeoutTimestamp));
  }

  public static Packet decode(byte[] data) {
    Packet obj = new Packet();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.sequence = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.sourcePort = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.sourceChannel = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.destinationPort = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.destinationChannel = resp.res;
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.data = resp.res;
            break;
        }
        case 7: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.timeoutHeight = icon.proto.core.client.Height.decode(resp.res);
            break;
        }
        case 8: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.timeoutTimestamp = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
