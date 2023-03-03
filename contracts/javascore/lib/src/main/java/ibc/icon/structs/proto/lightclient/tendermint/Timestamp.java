package ibc.icon.structs.proto.lightclient.tendermint;

import java.math.BigInteger;

import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

public class Timestamp {
    // Represents seconds of UTC time since Unix epoch
    // 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    // 9999-12-31T23:59:59Z inclusive.
    public BigInteger seconds;

    // Non-negative fractions of a second at nanosecond resolution. Negative
    // second values with fractions must still have non-negative nanos values
    // that count forward in time. Must be from 0 to 999,999,999
    // inclusive.
    public BigInteger nanos;

    public Timestamp(BigInteger seconds, BigInteger nanos) {
        this.seconds = seconds;
        this.nanos = nanos;
    }

    public Timestamp() {
    }

    public static void writeObject(ObjectWriter writer, Timestamp obj) {
        obj.writeObject(writer);
    }

    public static Timestamp readObject(ObjectReader reader) {
        Timestamp obj = new Timestamp();
        reader.beginList();
        obj.seconds = reader.readBigInteger();
        obj.nanos = reader.readBigInteger();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(seconds);
        writer.write(nanos);
        writer.end();
    }

    public boolean gt(Timestamp obj) {
        if (this.seconds.compareTo(obj.seconds) > 0) {
            return true;
        }

        if (this.seconds.equals(obj.seconds) && this.nanos.compareTo(obj.nanos) > 0) {
            return true;
        }

        return false;
    }

    public byte[] encode() {
        byte[] seconds = Proto.encode(1, this.seconds);
        byte[] nanos = Proto.encode(2, this.nanos);

        return Proto.join(seconds, nanos);
    }
}