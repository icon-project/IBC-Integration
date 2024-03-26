package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class ConsensusParams extends ProtoMessage {
  private BlockParams block = new BlockParams();

  private EvidenceParams evidence = new EvidenceParams();

  private ValidatorParams validator = new ValidatorParams();

  private VersionParams version = new VersionParams();

  private ABCIParams abci = new ABCIParams();

  public BlockParams getBlock() {
    return this.block;
  }

  public void setBlock(BlockParams block) {
    this.block = block;
  }

  public EvidenceParams getEvidence() {
    return this.evidence;
  }

  public void setEvidence(EvidenceParams evidence) {
    this.evidence = evidence;
  }

  public ValidatorParams getValidator() {
    return this.validator;
  }

  public void setValidator(ValidatorParams validator) {
    this.validator = validator;
  }

  public VersionParams getVersion() {
    return this.version;
  }

  public void setVersion(VersionParams version) {
    this.version = version;
  }

  public ABCIParams getAbci() {
    return this.abci;
  }

  public void setAbci(ABCIParams abci) {
    this.abci = abci;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.block),
      Proto.encode(2, this.evidence),
      Proto.encode(3, this.validator),
      Proto.encode(4, this.version),
      Proto.encode(5, this.abci));
  }

  public static ConsensusParams decode(byte[] data) {
    ConsensusParams obj = new ConsensusParams();
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
            obj.block = BlockParams.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.evidence = EvidenceParams.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validator = ValidatorParams.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.version = VersionParams.decode(resp.res);
            break;
        }
        case 5: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.abci = ABCIParams.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
