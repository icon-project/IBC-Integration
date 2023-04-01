package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.lang.String;
import java.math.BigInteger;

public class LightHeader extends ProtoMessage {
  private Consensus version;

  private String chainId = "";

  private BigInteger height = BigInteger.ZERO;

  private Timestamp time;

  private BlockID lastBlockId;

  private byte[] lastCommitHash = new byte[0];

  private byte[] dataHash = new byte[0];

  private byte[] validatorsHash = new byte[0];

  private byte[] nextValidatorsHash = new byte[0];

  private byte[] consensusHash = new byte[0];

  private byte[] appHash = new byte[0];

  private byte[] lastResultsHash = new byte[0];

  private byte[] evidenceHash = new byte[0];

  private byte[] proposerAddress = new byte[0];

  public Consensus getVersion() {
    return this.version;
  }

  public void setVersion(Consensus version) {
    this.version = version;
  }

  public String getChainId() {
    return this.chainId;
  }

  public void setChainId(String chainId) {
    this.chainId = chainId;
  }

  public BigInteger getHeight() {
    return this.height;
  }

  public void setHeight(BigInteger height) {
    this.height = height;
  }

  public Timestamp getTime() {
    return this.time;
  }

  public void setTime(Timestamp time) {
    this.time = time;
  }

  public BlockID getLastBlockId() {
    return this.lastBlockId;
  }

  public void setLastBlockId(BlockID lastBlockId) {
    this.lastBlockId = lastBlockId;
  }

  public byte[] getLastCommitHash() {
    return this.lastCommitHash;
  }

  public void setLastCommitHash(byte[] lastCommitHash) {
    this.lastCommitHash = lastCommitHash;
  }

  public byte[] getDataHash() {
    return this.dataHash;
  }

  public void setDataHash(byte[] dataHash) {
    this.dataHash = dataHash;
  }

  public byte[] getValidatorsHash() {
    return this.validatorsHash;
  }

  public void setValidatorsHash(byte[] validatorsHash) {
    this.validatorsHash = validatorsHash;
  }

  public byte[] getNextValidatorsHash() {
    return this.nextValidatorsHash;
  }

  public void setNextValidatorsHash(byte[] nextValidatorsHash) {
    this.nextValidatorsHash = nextValidatorsHash;
  }

  public byte[] getConsensusHash() {
    return this.consensusHash;
  }

  public void setConsensusHash(byte[] consensusHash) {
    this.consensusHash = consensusHash;
  }

  public byte[] getAppHash() {
    return this.appHash;
  }

  public void setAppHash(byte[] appHash) {
    this.appHash = appHash;
  }

  public byte[] getLastResultsHash() {
    return this.lastResultsHash;
  }

  public void setLastResultsHash(byte[] lastResultsHash) {
    this.lastResultsHash = lastResultsHash;
  }

  public byte[] getEvidenceHash() {
    return this.evidenceHash;
  }

  public void setEvidenceHash(byte[] evidenceHash) {
    this.evidenceHash = evidenceHash;
  }

  public byte[] getProposerAddress() {
    return this.proposerAddress;
  }

  public void setProposerAddress(byte[] proposerAddress) {
    this.proposerAddress = proposerAddress;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.version),
      Proto.encode(2, this.chainId),
      Proto.encode(3, this.height),
      Proto.encode(4, this.time),
      Proto.encode(5, this.lastBlockId),
      Proto.encode(6, this.lastCommitHash),
      Proto.encode(7, this.dataHash),
      Proto.encode(8, this.validatorsHash),
      Proto.encode(9, this.nextValidatorsHash),
      Proto.encode(10, this.consensusHash),
      Proto.encode(11, this.appHash),
      Proto.encode(12, this.lastResultsHash),
      Proto.encode(13, this.evidenceHash),
      Proto.encode(14, this.proposerAddress));
  }

  public static LightHeader decode(byte[] data) {
    LightHeader obj = new LightHeader();
    int index = 0;
    int order;
    int length = data.length;
    while (index < length) {
      order = data[index] >> 3;
      index++;
      switch(order) {
        case 1: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.version = Consensus.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<String> resp = Proto.decodeString(data, index);
            index = resp.index;
            obj.chainId = resp.res;
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.height = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.time = Timestamp.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lastBlockId = BlockID.decode(resp.res);
            break;
        }
        case 6: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lastCommitHash = resp.res;
            break;
        }
        case 7: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.dataHash = resp.res;
            break;
        }
        case 8: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validatorsHash = resp.res;
            break;
        }
        case 9: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.nextValidatorsHash = resp.res;
            break;
        }
        case 10: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.consensusHash = resp.res;
            break;
        }
        case 11: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.appHash = resp.res;
            break;
        }
        case 12: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.lastResultsHash = resp.res;
            break;
        }
        case 13: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.evidenceHash = resp.res;
            break;
        }
        case 14: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.proposerAddress = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
