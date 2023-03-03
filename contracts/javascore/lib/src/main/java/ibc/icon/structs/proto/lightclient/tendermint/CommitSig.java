package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

// CommitSig is a part of the Vote included in a Commit.
public class CommitSig {
    public int blockIdFlag;
    public byte[] validatorAddress;
    public Timestamp timestamp;
    public byte[] signature;

    public static void writeObject(ObjectWriter writer, CommitSig obj) {
        obj.writeObject(writer);
    }

    public static CommitSig readObject(ObjectReader reader) {
        CommitSig obj = new CommitSig();
        reader.beginList();
        obj.blockIdFlag = reader.readInt();
        obj.validatorAddress = reader.readByteArray();
        obj.timestamp = reader.read(Timestamp.class);
        obj.signature = reader.readByteArray();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(4);
        writer.write(blockIdFlag);
        writer.write(validatorAddress);
        writer.write(timestamp);
        writer.write(signature);
        writer.end();
    }
}