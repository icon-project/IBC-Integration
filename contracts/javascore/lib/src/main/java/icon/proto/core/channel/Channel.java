package icon.proto.core.channel;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Integer;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class Channel extends ProtoMessage {
  private int state = 0;

  private int ordering = 0;

  private Counterparty counterparty;

  private List<String> connectionHops = new ArrayList<>();

  private String version = "";

  public int getState() {
    return this.state;
  }

  public void setState(int state) {
    this.state = state;
  }

  public int getOrdering() {
    return this.ordering;
  }

  public void setOrdering(int ordering) {
    this.ordering = ordering;
  }

  public Counterparty getCounterparty() {
    return this.counterparty;
  }

  public void setCounterparty(Counterparty counterparty) {
    this.counterparty = counterparty;
  }

  public List<String> getConnectionHops() {
    return this.connectionHops;
  }

  public void setConnectionHops(List<String> connectionHops) {
    this.connectionHops = connectionHops;
  }

  public String getVersion() {
    return this.version;
  }

  public void setVersion(String version) {
    this.version = version;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.state),
      Proto.encode(2, this.ordering),
      Proto.encode(3, this.counterparty),
      Proto.encodeStringArray(4, this.connectionHops),
      Proto.encode(5, this.version));
  }

  public static Channel decode(byte[] data) {
    Channel obj = new Channel();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.state = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
            index = resp.index;
            obj.ordering = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.counterparty = Counterparty.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.connectionHops.add(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.version = resp.res;
            break;
        }
      }
    }
    return obj;
  }

  public static class State {
    public static final int STATE_UNINITIALIZED_UNSPECIFIED = 0;

    public static final int STATE_INIT = 1;

    public static final int STATE_TRYOPEN = 2;

    public static final int STATE_OPEN = 3;

    public static final int STATE_CLOSED = 4;
  }

  public static class Order {
    public static final int ORDER_NONE_UNSPECIFIED = 0;

    public static final int ORDER_UNORDERED = 1;

    public static final int ORDER_ORDERED = 2;
  }

  public static class Counterparty extends ProtoMessage {
    private String portId = "";

    private String channelId = "";

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

    public byte[] encode() {
      return ByteUtil.join(
        Proto.encode(1, this.portId),
        Proto.encode(2, this.channelId));
    }

    public static Counterparty decode(byte[] data) {
      Counterparty obj = new Counterparty();
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
        }
      }
      return obj;
    }
  }

  public static class IdentifiedChannel extends ProtoMessage {
    private int state = 0;

    private int ordering = 0;

    private Counterparty counterparty;

    private List<String> connectionHops = new ArrayList<>();

    private String version = "";

    private String portId = "";

    private String channelId = "";

    public int getState() {
      return this.state;
    }

    public void setState(int state) {
      this.state = state;
    }

    public int getOrdering() {
      return this.ordering;
    }

    public void setOrdering(int ordering) {
      this.ordering = ordering;
    }

    public Counterparty getCounterparty() {
      return this.counterparty;
    }

    public void setCounterparty(Counterparty counterparty) {
      this.counterparty = counterparty;
    }

    public List<String> getConnectionHops() {
      return this.connectionHops;
    }

    public void setConnectionHops(List<String> connectionHops) {
      this.connectionHops = connectionHops;
    }

    public String getVersion() {
      return this.version;
    }

    public void setVersion(String version) {
      this.version = version;
    }

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

    public byte[] encode() {
      return ByteUtil.join(
        Proto.encode(1, this.state),
        Proto.encode(2, this.ordering),
        Proto.encode(3, this.counterparty),
        Proto.encodeStringArray(4, this.connectionHops),
        Proto.encode(5, this.version),
        Proto.encode(6, this.portId),
        Proto.encode(7, this.channelId));
    }

    public static IdentifiedChannel decode(byte[] data) {
      IdentifiedChannel obj = new IdentifiedChannel();
      int index = 0;
      int order;
      int length = data.length;
      while (index < length) {
        order = data[index] >> 3;
        index++;
        switch(order) {
          case 1: {
              Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
              index = resp.index;
              obj.state = resp.res;
              break;
          }
          case 2: {
              Proto.DecodeResponse<Integer> resp = Proto.decodeEnum(data, index);
              index = resp.index;
              obj.ordering = resp.res;
              break;
          }
          case 3: {
              Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
              index = resp.index;
              obj.counterparty = Counterparty.decode(resp.res);
              break;
          }
          case 4: {
              Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
              index = resp.index;
              obj.connectionHops.add(resp.res);
              break;
          }
          case 5: {
              Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
              index = resp.index;
              obj.version = resp.res;
              break;
          }
          case 6: {
              Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
              index = resp.index;
              obj.portId = resp.res;
              break;
          }
          case 7: {
              Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
              index = resp.index;
              obj.channelId = resp.res;
              break;
          }
        }
      }
      return obj;
    }
  }
}
