package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class PartSetHeader {
    public BigInteger total;
    public byte[] hash;

    public static void writeObject(ObjectWriter writer, PartSetHeader obj) {
        obj.writeObject(writer);
    }

    public static PartSetHeader readObject(ObjectReader reader) {
        PartSetHeader obj = new PartSetHeader();
        reader.beginList();
        obj.total = reader.readBigInteger();
        obj.hash = reader.readByteArray();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(total);
        writer.write(hash);
        writer.end();
    }
}