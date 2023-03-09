package ibc.icon.structs.proto.core.client;

import java.math.BigInteger;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class Height {
    public BigInteger revisionNumber;
    public BigInteger revisionHeight;

    public Height() {

    }

    public Height(BigInteger revisionNumber, BigInteger revisionHeight) {
        this.revisionNumber = revisionNumber;
        this.revisionHeight = revisionHeight;
    }

    public static void writeObject(ObjectWriter writer, Height obj) {
        obj.writeObject(writer);
    }

    public static Height readObject(ObjectReader reader) {
        Height obj = new Height();
        reader.beginList();
        obj.revisionNumber = reader.readBigInteger();
        obj.revisionHeight = reader.readBigInteger();
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(this.revisionNumber);
        writer.write(this.revisionHeight);

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

    public BigInteger getRevisionNumber() {
        return revisionNumber;
    }

    public void setRevisionNumber(BigInteger revisionNumber) {
        this.revisionNumber = revisionNumber;
    }

    public BigInteger getRevisionHeight() {
        return revisionHeight;
    }

    public void setRevisionHeight(BigInteger revisionHeight) {
        this.revisionHeight = revisionHeight;
    }

    // public BigInteger toUint128() {
    // return (uint128(this.getRevisionNumber()) << 64) |
    // uint128(this.getRevisionHeight());
    // }

    public boolean isZero() {
        return this.getRevisionNumber().equals(BigInteger.ZERO) && this.getRevisionHeight().equals(BigInteger.ZERO);
    }

    public boolean lt(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) < 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().compareTo(other.getRevisionHeight()) < 0);
    }

    public boolean lte(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) < 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().compareTo(other.getRevisionHeight()) <= 0);
    }

    public boolean eq(Height other) {
        return this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().equals(other.getRevisionHeight());
    }

    public boolean gt(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) > 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().compareTo(other.getRevisionHeight()) > 0);
    }

    public boolean gte(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) > 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().compareTo(other.getRevisionHeight()) >= 0);
    }
}
