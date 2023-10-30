package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import icon.proto.core.client.Height;
import java.lang.Boolean;
import java.lang.String;
import java.math.BigInteger;

public class ClientState extends ProtoMessage {
  private String chainId = "";

  private Fraction trustLevel = new Fraction();

  private Duration trustingPeriod = new Duration();

  private Duration unbondingPeriod = new Duration();

  private Duration maxClockDrift = new Duration();

  private Height frozenHeight = new Height();

  private Height latestHeight = new Height();

  private boolean allowUpdateAfterExpiry = false;

  private boolean allowUpdateAfterMisbehaviour = false;

  public String getChainId() {
    return this.chainId;
  }

  public void setChainId(String chainId) {
    this.chainId = chainId;
  }

  public Fraction getTrustLevel() {
    return this.trustLevel;
  }

  public void setTrustLevel(Fraction trustLevel) {
    this.trustLevel = trustLevel;
  }

  public Duration getTrustingPeriod() {
    return this.trustingPeriod;
  }

  public void setTrustingPeriod(Duration trustingPeriod) {
    this.trustingPeriod = trustingPeriod;
  }

  public Duration getUnbondingPeriod() {
    return this.unbondingPeriod;
  }

  public void setUnbondingPeriod(Duration unbondingPeriod) {
    this.unbondingPeriod = unbondingPeriod;
  }

  public Duration getMaxClockDrift() {
    return this.maxClockDrift;
  }

  public void setMaxClockDrift(Duration maxClockDrift) {
    this.maxClockDrift = maxClockDrift;
  }

  public BigInteger getFrozenHeight() {
    return this.frozenHeight.getRevisionHeight();
  }

  public void setFrozenHeight(BigInteger frozenHeight) {
    this.frozenHeight = new Height(new BigInteger("1"), frozenHeight);
  }

  public BigInteger getLatestHeight() {
    return this.latestHeight.getRevisionHeight();
  }

  public void setLatestHeight(BigInteger latestHeight) {
    this.latestHeight = new Height(new BigInteger("1"), latestHeight);
  }

  public boolean getAllowUpdateAfterExpiry() {
    return this.allowUpdateAfterExpiry;
  }

  public void setAllowUpdateAfterExpiry(boolean allowUpdateAfterExpiry) {
    this.allowUpdateAfterExpiry = allowUpdateAfterExpiry;
  }

  public boolean getAllowUpdateAfterMisbehaviour() {
    return this.allowUpdateAfterMisbehaviour;
  }

  public void setAllowUpdateAfterMisbehaviour(boolean allowUpdateAfterMisbehaviour) {
    this.allowUpdateAfterMisbehaviour = allowUpdateAfterMisbehaviour;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.chainId),
      Proto.encode(2, this.trustLevel),
      Proto.encode(3, this.trustingPeriod),
      Proto.encode(4, this.unbondingPeriod),
      Proto.encode(5, this.maxClockDrift),
      Proto.encode(6, this.frozenHeight),
      Proto.encode(7, this.latestHeight),
      Proto.encode(8, this.allowUpdateAfterExpiry),
      Proto.encode(9, this.allowUpdateAfterMisbehaviour));
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
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.chainId = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.trustLevel = Fraction.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.trustingPeriod = Duration.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.unbondingPeriod = Duration.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.maxClockDrift = Duration.decode(resp.res);
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.frozenHeight = icon.proto.core.client.Height.decode(resp.res);
            break;
        }
        case 7: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.latestHeight = icon.proto.core.client.Height.decode(resp.res);
            break;
        }
        case 8: {
            Proto.DecodeResponse<Boolean> resp = Proto.decodeBoolean(data, index);
            index = resp.index;
            obj.allowUpdateAfterExpiry = resp.res;
            break;
        }
        case 9: {
            Proto.DecodeResponse<Boolean> resp = Proto.decodeBoolean(data, index);
            index = resp.index;
            obj.allowUpdateAfterMisbehaviour = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
