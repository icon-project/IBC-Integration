package ibc.lightclients.tendermint.v1;

import cosmos.ics23.v1.ProofSpec;
import google.protobuf.Duration;
import ibc.core.client.v1.Height;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.Boolean;
import java.lang.String;
import java.util.List;
import scorex.util.ArrayList;

public class ClientState extends ProtoMessage {
  private String chainId = "";

  private Fraction trustLevel = new Fraction();

  private Duration trustingPeriod = new google.protobuf.Duration();

  private Duration unbondingPeriod = new google.protobuf.Duration();

  private Duration maxClockDrift = new google.protobuf.Duration();

  private Height frozenHeight = new ibc.core.client.v1.Height();

  private Height latestHeight = new ibc.core.client.v1.Height();

  private List<ProofSpec> proofSpecs = new ArrayList<>();

  private List<String> upgradePath = new ArrayList<>();

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

  public Height getFrozenHeight() {
    return this.frozenHeight;
  }

  public void setFrozenHeight(Height frozenHeight) {
    this.frozenHeight = frozenHeight;
  }

  public Height getLatestHeight() {
    return this.latestHeight;
  }

  public void setLatestHeight(Height latestHeight) {
    this.latestHeight = latestHeight;
  }

  public List<ProofSpec> getProofSpecs() {
    return this.proofSpecs;
  }

  public void setProofSpecs(List<ProofSpec> proofSpecs) {
    this.proofSpecs = proofSpecs;
  }

  public List<String> getUpgradePath() {
    return this.upgradePath;
  }

  public void setUpgradePath(List<String> upgradePath) {
    this.upgradePath = upgradePath;
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
      Proto.encodeMessageArray(8, this.proofSpecs),
      Proto.encodeStringArray(9, this.upgradePath),
      Proto.encode(10, this.allowUpdateAfterExpiry),
      Proto.encode(11, this.allowUpdateAfterMisbehaviour));
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
            obj.trustingPeriod = google.protobuf.Duration.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.unbondingPeriod = google.protobuf.Duration.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.maxClockDrift = google.protobuf.Duration.decode(resp.res);
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.frozenHeight = ibc.core.client.v1.Height.decode(resp.res);
            break;
        }
        case 7: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.latestHeight = ibc.core.client.v1.Height.decode(resp.res);
            break;
        }
        case 8: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.proofSpecs.add(cosmos.ics23.v1.ProofSpec.decode(resp.res));
            break;
        }
        case 9: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.upgradePath.add(resp.res);
            break;
        }
        case 10: {
            Proto.DecodeResponse<Boolean> resp = Proto.decodeBoolean(data, index);
            index = resp.index;
            obj.allowUpdateAfterExpiry = resp.res;
            break;
        }
        case 11: {
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
