package icon.proto.clients.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;
import java.math.BigInteger;

public class TmHeader extends ProtoMessage {
  private SignedHeader signedHeader = new SignedHeader();

  private ValidatorSet validatorSet = new ValidatorSet();

  private BigInteger trustedHeight = BigInteger.ZERO;

  private ValidatorSet trustedValidators = new ValidatorSet();

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

  public BigInteger getTrustedHeight() {
    return this.trustedHeight;
  }

  public void setTrustedHeight(BigInteger trustedHeight) {
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

  public static TmHeader decode(byte[] data) {
    TmHeader obj = new TmHeader();
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
            obj.validatorSet = ValidatorSet.decode(resp.res);
            break;
        }
        case 3: {
            Proto.DecodeResponse<BigInteger> resp = Proto.decodeVarInt(data, index);
            index = resp.index;
            obj.trustedHeight = resp.res;
            break;
        }
        case 4: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.trustedValidators = ValidatorSet.decode(resp.res);
            break;
        }
      }
    }
    return obj;
  }
}
