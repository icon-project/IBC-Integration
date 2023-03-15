package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class Consensus {
    public BigInteger block;
    public BigInteger app;

    public static void writeObject(ObjectWriter writer, Consensus obj) {
        obj.writeObject(writer);
    }

    public static Consensus readObject(ObjectReader reader) {
        Consensus obj = new Consensus();
        reader.beginList();
        obj.block = reader.readBigInteger();
        obj.app = reader.readNullable(BigInteger.class);
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(block);
        writer.writeNullable(app);
        writer.end();
    }
}