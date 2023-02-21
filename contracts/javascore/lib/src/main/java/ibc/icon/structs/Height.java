package ibc.icon.structs;

import java.math.BigInteger;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class Height {
    public BigInteger revisionNumber;
    public BigInteger revisionHeight;

    public BigInteger getRevisionHeight() {
        return revisionHeight;
    }

    public BigInteger getRevisionNumber() {
        return revisionNumber;
    }

    public void setRevisionHeight(BigInteger revisionHeight) {
        this.revisionHeight = revisionHeight;
    }

    public void setRevisionNumber(BigInteger revisionNumber) {
        this.revisionNumber = revisionNumber;
    }

    public static void writeObject(ObjectWriter writer, Height obj) {
        obj.writeObject(writer);
    }

    public static Height readObject(ObjectReader reader) {
        Height obj = new Height();
        reader.beginList();
        obj.setRevisionNumber(reader.readBigInteger());
        obj.setRevisionHeight(reader.readBigInteger());
        reader.end();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(this.getRevisionNumber());
        writer.write(this.getRevisionHeight());
        writer.end();
    }

    public static Height fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return Height.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        Height.writeObject(writer, this);
        return writer.toByteArray();
    }
}
