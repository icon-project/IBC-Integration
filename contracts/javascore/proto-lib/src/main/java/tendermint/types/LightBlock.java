package tendermint.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class LightBlock extends ProtoMessage {
  private SignedHeader signedHeader = new SignedHeader();

  private ValidatorSet validatorSet = new tendermint.types.ValidatorSet();

  public SignedHeader getSignedHeader() {
    return this.signedHeader;
  }

  public void setSignedHeader(SignedHeader signedHeader) {
    this.signedHeader = signedHeader;
  }

  public ValidatorSet getValidatorSet() {
    return this.validatorSet;
  }

  public void setValidatorSet(ValidatorSet validatorSet) {
    this.validatorSet = validatorSet;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.signedHeader),
      Proto.encode(2, this.validatorSet));
  }

  public static LightBlock decode(byte[] data) {
    LightBlock obj = new LightBlock();
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
            obj.signedHeader = SignedHeader.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validatorSet = tendermint.types.ValidatorSet.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
