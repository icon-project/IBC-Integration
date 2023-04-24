package icon.proto.icon.types.v1;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;
import java.util.List;
import scorex.util.ArrayList;

public class BTPHeader extends ProtoMessage {
  private BigInteger mainHeight = BigInteger.ZERO;

  private BigInteger round = BigInteger.ZERO;

  private byte[] nextProofContextHash = new byte[0];

  private List<MerkleNode> networkSectionToRoot = new ArrayList<>();

  private BigInteger networkId = BigInteger.ZERO;

  private BigInteger updateNumber = BigInteger.ZERO;

  private byte[] prevNetworkSectionHash = new byte[0];

  private BigInteger messageCount = BigInteger.ZERO;

  private byte[] messageRoot = new byte[0];

  private List<byte[]> nextvalidators = new ArrayList<>();

  public BigInteger getMainHeight() {
    return this.mainHeight;
  }

  public void setMainHeight(BigInteger mainHeight) {
    this.mainHeight = mainHeight;
  }

  public BigInteger getRound() {
    return this.round;
  }

  public void setRound(BigInteger round) {
    this.round = round;
  }

  public byte[] getNextProofContextHash() {
    return this.nextProofContextHash;
  }

  public void setNextProofContextHash(byte[] nextProofContextHash) {
    this.nextProofContextHash = nextProofContextHash;
  }

  public List<MerkleNode> getNetworkSectionToRoot() {
    return this.networkSectionToRoot;
  }

  public void setNetworkSectionToRoot(List<MerkleNode> networkSectionToRoot) {
    this.networkSectionToRoot = networkSectionToRoot;
  }

  public BigInteger getNetworkId() {
    return this.networkId;
  }

  public void setNetworkId(BigInteger networkId) {
    this.networkId = networkId;
  }

  public BigInteger getUpdateNumber() {
    return this.updateNumber;
  }

  public void setUpdateNumber(BigInteger updateNumber) {
    this.updateNumber = updateNumber;
  }

  public byte[] getPrevNetworkSectionHash() {
    return this.prevNetworkSectionHash;
  }

  public void setPrevNetworkSectionHash(byte[] prevNetworkSectionHash) {
    this.prevNetworkSectionHash = prevNetworkSectionHash;
  }

  public BigInteger getMessageCount() {
    return this.messageCount;
  }

  public void setMessageCount(BigInteger messageCount) {
    this.messageCount = messageCount;
  }

  public byte[] getMessageRoot() {
    return this.messageRoot;
  }

  public void setMessageRoot(byte[] messageRoot) {
    this.messageRoot = messageRoot;
  }

  public List<byte[]> getNextvalidators() {
    return this.nextvalidators;
  }

  public void setNextvalidators(List<byte[]> nextvalidators) {
    this.nextvalidators = nextvalidators;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.mainHeight),
      Proto.encode(2, this.round),
      Proto.encode(3, this.nextProofContextHash),
      Proto.encodeMessageArray(4, this.networkSectionToRoot),
      Proto.encode(5, this.networkId),
      Proto.encode(6, this.updateNumber),
      Proto.encode(7, this.prevNetworkSectionHash),
      Proto.encode(8, this.messageCount),
      Proto.encode(9, this.messageRoot),
      Proto.encodeBytesArray(10, this.nextvalidators));
  }

  public static BTPHeader decode(byte[] data) {
    BTPHeader obj = new BTPHeader();
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
            obj.mainHeight = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.round = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nextProofContextHash = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.networkSectionToRoot.add(MerkleNode.decode(resp.res));
            break;
        }
        case 5: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.networkId = resp.res;
            break;
        }
        case 6: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.updateNumber = resp.res;
            break;
        }
        case 7: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.prevNetworkSectionHash = resp.res;
            break;
        }
        case 8: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.messageCount = resp.res;
            break;
        }
        case 9: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.messageRoot = resp.res;
            break;
        }
        case 10: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nextvalidators.add(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
