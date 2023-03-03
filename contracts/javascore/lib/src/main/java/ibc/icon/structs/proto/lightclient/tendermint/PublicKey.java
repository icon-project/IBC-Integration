package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

public class PublicKey {
    public byte[] ed25519;
    public byte[] secp256k1;

    public static void writeObject(ObjectWriter writer, PublicKey obj) {
        obj.writeObject(writer);
    }

    public static PublicKey readObject(ObjectReader reader) {
        PublicKey obj = new PublicKey();
        reader.beginList();
        obj.ed25519 = reader.readNullable(byte[].class);
        obj.secp256k1 = reader.readNullable(byte[].class);
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.writeNullable(ed25519);
        writer.writeNullable(secp256k1);
        writer.end();
    }

    public byte[] encode() {

        if (this.ed25519 != null && this.ed25519.length != 0) {
            return Proto.encode(1, this.ed25519);
        }

        return Proto.encode(2, this.secp256k1);

    }
}
