package tendermint.crypto;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import ibc.icon.score.util.ProtoMessage;

public class PublicKey extends ProtoMessage {
  private byte[] ed25519 = new byte[0];

  private byte[] secp256k1 = new byte[0];

  public byte[] getEd25519() {
    return this.ed25519;
  }

  public void setEd25519(byte[] ed25519) {
    this.ed25519 = ed25519;
  }

  public byte[] getSecp256k1() {
    return this.secp256k1;
  }

  public void setSecp256k1(byte[] secp256k1) {
    this.secp256k1 = secp256k1;
  }

  public byte[] encode() {
    return ByteUtil.join(
      Proto.encode(1, this.ed25519),
      Proto.encode(2, this.secp256k1));
  }

  public static PublicKey decode(byte[] data) {
    PublicKey obj = new PublicKey();
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
            obj.ed25519 = resp.res;
            break;
        }
        case 2: {
            Proto.DecodeResponse<byte[]> resp = Proto.decodeBytes(data, index);
            index = resp.index;
            obj.secp256k1 = resp.res;
            break;
        }
      }
    }
    return obj;
  }
}
