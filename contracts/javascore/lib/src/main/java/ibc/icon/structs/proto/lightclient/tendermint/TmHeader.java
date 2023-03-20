package ibc.icon.structs.proto.lightclient.tendermint;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class TmHeader {
    public SignedHeader signedHeader;
    public ValidatorSet validatorSet;

    public BigInteger trustedHeight;
    public ValidatorSet trustedValidators;

    public TmHeader() {
    }

    public TmHeader(SignedHeader signedHeader, ValidatorSet validatorSet, BigInteger trustedHeight,
                    ValidatorSet trustedValidators) {
        this.signedHeader = signedHeader;
        this.validatorSet = validatorSet;
        this.trustedHeight = trustedHeight;
        this.trustedValidators = trustedValidators;
    }

    public static void writeObject(ObjectWriter writer, TmHeader obj) {
        obj.writeObject(writer);
    }

    public static TmHeader readObject(ObjectReader reader) {
        TmHeader obj = new TmHeader();
        reader.beginList();
        obj.signedHeader = reader.read(SignedHeader.class);
        obj.validatorSet = reader.read(ValidatorSet.class);
        obj.trustedHeight = reader.readBigInteger();
        obj.trustedValidators = reader.read(ValidatorSet.class);
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(4);
        writer.write(signedHeader);
        writer.write(validatorSet);
        writer.write(trustedHeight);
        writer.write(trustedValidators);
        writer.end();
    }

    public static TmHeader fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return TmHeader.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        TmHeader.writeObject(writer, this);
        return writer.toByteArray();
    }

    public SignedHeader getSignedHeader() {
        return signedHeader;
    }

    public void setSignedHeader(SignedHeader signedHeader) {
        this.signedHeader = signedHeader;
    }

    public ValidatorSet getValidatorSet() {
        return validatorSet;
    }

    public void setValidatorSet(ValidatorSet validatorSet) {
        this.validatorSet = validatorSet;
    }

    public BigInteger getTrustedHeight() {
        return trustedHeight;
    }

    public void setTrustedHeight(BigInteger trustedHeight) {
        this.trustedHeight = trustedHeight;
    }

    public ValidatorSet getTrustedValidators() {
        return trustedValidators;
    }

    public void setTrustedValidators(ValidatorSet trustedValidators) {
        this.trustedValidators = trustedValidators;
    }

    public ConsensusState toConsensusState() {
        return new ConsensusState(
                this.signedHeader.header.time,
                new MerkleRoot(this.signedHeader.header.appHash),
                this.signedHeader.header.nextValidatorsHash);
    }

}