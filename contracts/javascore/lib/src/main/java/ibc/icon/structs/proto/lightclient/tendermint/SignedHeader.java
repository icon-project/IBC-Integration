package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class SignedHeader {
    public LightHeader header;
    public Commit commit;

    public boolean isExpired(Duration trustingPeriod, Duration currentTime) {
        Timestamp expirationTime = new Timestamp(this.header.time.seconds.add(trustingPeriod.seconds),
                this.header.time.nanos);

        return new Timestamp(currentTime.seconds, BigInteger.ZERO).gt(expirationTime);
    }

    public static void writeObject(ObjectWriter writer, SignedHeader obj) {
        obj.writeObject(writer);
    }

    public static SignedHeader readObject(ObjectReader reader) {
        SignedHeader obj = new SignedHeader();
        reader.beginList();
        obj.header = reader.read(LightHeader.class);
        obj.commit = reader.read(Commit.class);
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(header);
        writer.write(commit);
        writer.end();
    }
}