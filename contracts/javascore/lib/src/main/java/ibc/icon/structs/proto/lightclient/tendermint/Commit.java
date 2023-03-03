package ibc.icon.structs.proto.lightclient.tendermint;

import java.math.BigInteger;
import java.util.List;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

public class Commit {
    public BigInteger height;
    public BigInteger round;
    public BlockID blockId;
    public CommitSig[] signatures;

    public CanonicalVote toCanonicalVote(int valIdx, String chainId) {
        CommitSig commitSig = this.signatures[valIdx];
        CanonicalVote vote = new CanonicalVote();

        vote.type = SignedMsgType.SIGNED_MSG_TYPE_PRECOMMIT;
        vote.height = this.height;
        vote.round = this.round;
        vote.blockId = this.blockId;
        vote.timestamp = commitSig.timestamp;
        vote.chainId = chainId;
        return vote;
    }

    public static void writeObject(ObjectWriter writer, Commit obj) {
        obj.writeObject(writer);
    }

    public static Commit readObject(ObjectReader reader) {
        Commit obj = new Commit();
        reader.beginList();
        obj.height = reader.readBigInteger();
        obj.round = reader.readNullable(BigInteger.class);
        obj.blockId = reader.read(BlockID.class);

        reader.beginList();
        List<CommitSig> signaturesList = new ArrayList<>();
        while (reader.hasNext()) {
            byte[] commitSigBytes = reader.readNullable(byte[].class);
            if (commitSigBytes != null) {
                ObjectReader commitSigReader = Context.newByteArrayObjectReader("RLPn", commitSigBytes);
                signaturesList.add(commitSigReader.read(CommitSig.class));
            }
        }

        CommitSig[] signatures = new CommitSig[signaturesList.size()];
        for (int i = 0; i < signaturesList.size(); i++) {
            signatures[i] = signaturesList.get(i);
        }
        obj.signatures = signatures;
        reader.end();
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(4);
        writer.write(height);
        writer.writeNullable(round);
        writer.write(blockId);

        CommitSig[] signatures = this.signatures;
        if (signatures != null) {
            writer.beginNullableList(signatures.length);
            for (CommitSig v : signatures) {
                ByteArrayObjectWriter vWriter = Context.newByteArrayObjectWriter("RLPn");
                vWriter.write(v);
                writer.write(vWriter.toByteArray());
            }
            writer.end();
        } else {
            writer.writeNull();
        }

        writer.end();
    }
}