package ibc.icon.structs.proto.lightclient.tendermint;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

// ConsensusState defines the consensus state from Tendermint.
public class ConsensusState {
    // timestamp that corresponds to the block height in which the ConsensusState
    // was stored.
    public Timestamp timestamp;

    // commitment root (i.e app hash)
    public MerkleRoot root;
    public byte[] nextValidatorsHash;

    public ConsensusState() {
    }

    public ConsensusState(Timestamp timestamp, MerkleRoot root, byte[] nextValidatorsHash) {
        this.timestamp = timestamp;
        this.root = root;
        this.nextValidatorsHash = nextValidatorsHash;
    }

    public static void writeObject(ObjectWriter writer, ConsensusState obj) {
        obj.writeObject(writer);
    }

    public static ConsensusState readObject(ObjectReader reader) {
        ConsensusState obj = new ConsensusState();
        reader.beginList();
        obj.timestamp = reader.read(Timestamp.class);
        obj.root = reader.read(MerkleRoot.class);
        obj.nextValidatorsHash = reader.readByteArray();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(3);
        writer.write(timestamp);
        writer.write(root);
        writer.write(nextValidatorsHash);
        writer.end();
    }

    public static ConsensusState fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return ConsensusState.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        ConsensusState.writeObject(writer, this);
        return writer.toByteArray();
    }

    public Timestamp getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Timestamp timestamp) {
        this.timestamp = timestamp;
    }

    public MerkleRoot getRoot() {
        return root;
    }

    public void setRoot(MerkleRoot root) {
        this.root = root;
    }

    public byte[] getNextValidatorsHash() {
        return nextValidatorsHash;
    }

    public void setNextValidatorsHash(byte[] nextValidatorsHash) {
        this.nextValidatorsHash = nextValidatorsHash;
    }

    public boolean isEqual(ConsensusState obj) {
        return true;
        // TODO
        // keccak256(abi.encodePacked(ConsensusState.encode(cs1))) ==
        // keccak256(abi.encodePacked(ConsensusState.encode(cs2)));
    }
}