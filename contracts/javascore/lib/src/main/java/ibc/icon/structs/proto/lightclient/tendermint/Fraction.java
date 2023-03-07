package ibc.icon.structs.proto.lightclient.tendermint;

import java.math.BigInteger;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

public class Fraction {
    public BigInteger numerator;
    public BigInteger denominator;

    public Fraction(BigInteger numerator, BigInteger denominator) {
        this.numerator = numerator;
        this.denominator = denominator;
    }

    public Fraction() {

    }

    public static void writeObject(ObjectWriter writer, Fraction obj) {
        obj.writeObject(writer);
    }

    public static Fraction readObject(ObjectReader reader) {
        Fraction obj = new Fraction();
        reader.beginList();
        obj.numerator = reader.readBigInteger();
        obj.denominator = reader.readBigInteger();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(numerator);
        writer.write(denominator);
        writer.end();
    }

    public byte[] encode() {
        byte[] seconds = Proto.encode(1, this.numerator);
        byte[] nanos = Proto.encode(2, this.denominator);

        return ByteUtil.join(seconds, nanos);
    }
}