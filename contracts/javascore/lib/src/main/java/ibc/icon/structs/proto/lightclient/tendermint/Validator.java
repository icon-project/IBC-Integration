package ibc.icon.structs.proto.lightclient.tendermint;

import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class Validator {
    public byte[] address;
    public PublicKey pubKey;
    public BigInteger votingPower;
    public BigInteger proposerPriority;

    public SimpleValidator toSimpleValidator() {
        return new SimpleValidator(this.pubKey, this.votingPower);
    }

    public static void writeObject(ObjectWriter writer, Validator obj) {
        obj.writeObject(writer);
    }

    public static Validator readObject(ObjectReader reader) {
        Validator obj = new Validator();
        reader.beginList();
        obj.address = reader.readByteArray();
        obj.pubKey = reader.read(PublicKey.class);
        obj.votingPower = reader.readBigInteger();
        obj.proposerPriority = reader.readBigInteger();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(4);
        writer.write(address);
        writer.write(pubKey);
        writer.write(votingPower);
        writer.write(proposerPriority);
        writer.end();
    }

}