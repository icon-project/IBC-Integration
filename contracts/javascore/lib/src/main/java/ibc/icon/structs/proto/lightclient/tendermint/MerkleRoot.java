package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

// MerkleRoot defines a merkle root hash.
// In the Cosmos SDK, the AppHash of a block header becomes the root.
public class MerkleRoot {
    public byte[] hash;

    public MerkleRoot() {
    }

    public MerkleRoot(byte[] hash) {
        this.hash = hash;
    }

    public static void writeObject(ObjectWriter writer, MerkleRoot obj) {
        obj.writeObject(writer);
    }

    public static MerkleRoot readObject(ObjectReader reader) {
        MerkleRoot obj = new MerkleRoot();
        reader.beginList();
        obj.hash = reader.readByteArray();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(1);
        writer.write(hash);
        writer.end();
    }

    public byte[] getHash() {
        return hash;
    }

    public void setHash(byte[] hash) {
        this.hash = hash;
    }

}