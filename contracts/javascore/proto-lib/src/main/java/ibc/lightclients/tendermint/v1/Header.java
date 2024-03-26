package ibc.lightclients.tendermint.v1;

import ibc.core.client.v1.Height;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import tendermint.types.SignedHeader;
import tendermint.types.ValidatorSet;

public class Header extends ProtoMessage {
  private SignedHeader signedHeader = new tendermint.types.SignedHeader();

  private ValidatorSet validatorSet = new tendermint.types.ValidatorSet();

  private Height trustedHeight = new ibc.core.client.v1.Height();

  private ValidatorSet trustedValidators = new tendermint.types.ValidatorSet();

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

  public Height getTrustedHeight() {
    return this.trustedHeight;
  }

  public void setTrustedHeight(Height trustedHeight) {
    this.trustedHeight = trustedHeight;
  }

  public ValidatorSet getTrustedValidators() {
    return this.trustedValidators;
  }

  public void setTrustedValidators(ValidatorSet trustedValidators) {
    this.trustedValidators = trustedValidators;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.signedHeader),
      Proto.encode(2, this.validatorSet),
      Proto.encode(3, this.trustedHeight),
      Proto.encode(4, this.trustedValidators));
  }

  public static Header decode(byte[] data) {
    Header obj = new Header();
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
            obj.signedHeader = tendermint.types.SignedHeader.decode(resp.res);
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.validatorSet = tendermint.types.ValidatorSet.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.trustedHeight = ibc.core.client.v1.Height.decode(resp.res);
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.trustedValidators = tendermint.types.ValidatorSet.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
