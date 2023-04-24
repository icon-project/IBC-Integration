package icon.proto.icon.lightclient.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class ClientState extends ProtoMessage {
  private BigInteger trustingPeriod = BigInteger.ZERO;

  private BigInteger frozenHeight = BigInteger.ZERO;

  private BigInteger maxClockDrift = BigInteger.ZERO;

  private BigInteger latestHeight = BigInteger.ZERO;

  private byte[] networkSectionHash = new byte[0];

  private List<byte[]> validators = new ArrayList<>();

  public BigInteger getTrustingPeriod() {
    return this.trustingPeriod;
  }

  public void setTrustingPeriod(BigInteger trustingPeriod) {
    this.trustingPeriod = trustingPeriod;
  }

  public BigInteger getFrozenHeight() {
    return this.frozenHeight;
  }

  public void setFrozenHeight(BigInteger frozenHeight) {
    this.frozenHeight = frozenHeight;
  }

  public BigInteger getMaxClockDrift() {
    return this.maxClockDrift;
  }

  public void setMaxClockDrift(BigInteger maxClockDrift) {
    this.maxClockDrift = maxClockDrift;
  }

  public BigInteger getLatestHeight() {
    return this.latestHeight;
  }

  public void setLatestHeight(BigInteger latestHeight) {
    this.latestHeight = latestHeight;
  }

  public byte[] getNetworkSectionHash() {
    return this.networkSectionHash;
  }

  public void setNetworkSectionHash(byte[] networkSectionHash) {
    this.networkSectionHash = networkSectionHash;
  }

  public List<byte[]> getValidators() {
    return this.validators;
  }

  public void setValidators(List<byte[]> validators) {
    this.validators = validators;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.trustingPeriod),
      Proto.encode(2, this.frozenHeight),
      Proto.encode(3, this.maxClockDrift),
      Proto.encode(4, this.latestHeight),
      Proto.encode(5, this.networkSectionHash),
      Proto.encodeBytesArray(6, this.validators));
  }

  public static ClientState decode(byte[] data) {
    ClientState obj = new ClientState();
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
            obj.trustingPeriod = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.frozenHeight = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.maxClockDrift = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.latestHeight = resp.res;
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.networkSectionHash = resp.res;
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validators.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
