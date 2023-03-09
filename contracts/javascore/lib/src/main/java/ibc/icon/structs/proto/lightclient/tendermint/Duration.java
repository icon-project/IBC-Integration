package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Timestamp
public class Duration {
    public BigInteger seconds;
    public BigInteger nanos;

    public Duration(BigInteger seconds, BigInteger nanos) {
        this.seconds = seconds;
        this.nanos = nanos;
    }

    public Duration() {

    }

    public static void writeObject(ObjectWriter writer, Duration obj) {
        obj.writeObject(writer);
    }

    public static Duration readObject(ObjectReader reader) {
        Duration obj = new Duration();
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

    public byte[] encode() {
        byte[] seconds = Proto.encode(1, this.seconds);
        byte[] nanos = Proto.encode(2, this.nanos);

        return ByteUtil.join(seconds, nanos);
    }

}