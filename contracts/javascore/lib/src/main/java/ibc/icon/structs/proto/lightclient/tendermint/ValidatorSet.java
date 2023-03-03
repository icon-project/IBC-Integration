package ibc.icon.structs.proto.lightclient.tendermint;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

import ibc.icon.score.util.MerkleTree;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

public class ValidatorSet {
    public Validator[] validators;
    public Validator proposer;
    public BigInteger totalVotingPower;

    public static void writeObject(ObjectWriter writer, ValidatorSet obj) {
        obj.writeObject(writer);
    }

    public static ValidatorSet readObject(ObjectReader reader) {
        ValidatorSet obj = new ValidatorSet();
        reader.beginList();
        reader.beginList();
        List<Validator> validatorsList = new ArrayList<>();
        while (reader.hasNext()) {
            byte[] validatorBytes = reader.readNullable(byte[].class);
            if (validatorBytes != null) {
                ObjectReader validatorReader = Context.newByteArrayObjectReader("RLPn", validatorBytes);
                validatorsList.add(validatorReader.read(Validator.class));
            }
        }

        Validator[] validators = new Validator[validatorsList.size()];
        for (int i = 0; i < validatorsList.size(); i++) {
            validators[i] = validatorsList.get(i);
        }
        obj.validators = validators;
        reader.end();

        obj.proposer = reader.readNullable(Validator.class);
        obj.totalVotingPower = reader.readBigInteger();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(3);

        Validator[] validators = this.validators;
        if (validators != null) {
            writer.beginNullableList(validators.length);
            for (Validator v : validators) {
                ByteArrayObjectWriter vWriter = Context.newByteArrayObjectWriter("RLPn");
                vWriter.write(v);
                writer.write(vWriter.toByteArray());
            }
            writer.end();
        } else {
            writer.writeNull();
        }

        writer.writeNullable(proposer);
        writer.write(totalVotingPower);
        writer.end();
    }

    public BigInteger getTotalVotingPower() {
        if (!this.totalVotingPower.equals(BigInteger.ZERO)) {
            return this.totalVotingPower;
        }

        BigInteger sum = BigInteger.ZERO;
        for (Validator validator : validators) {
            sum = sum.add(validator.votingPower);
            // TODO do we need this?
            // Context.require(sum.compareTo(maxTotalVotingPower) <= 0, "total voting power should be guarded to not exceed");
        }

        totalVotingPower = sum;
        return totalVotingPower;
    }

    public int getByAddress(byte[] addr) {
        for (int idx = 0; idx < this.validators.length; idx++) {
            if (Arrays.equals(this.validators[idx].address, addr)) {
                return idx;
            }
        }

        return -1;
    }

    public byte[] hash() {
        int size = this.validators.length;
        byte[][] data = new byte[size][];
        for (int i = 0; i < size; i++) {
            data[i] = this.validators[i].toSimpleValidator().encode();
        }

        return MerkleTree.merkleRootHash(data, 0, size);
    }

}