package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.MerkleTree;
import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class LightHeader {
    public Consensus version;
    public String chainId;
    public BigInteger height;
    public Timestamp time;
    public BlockID lastBlockId;
    public byte[] lastCommitHash; // commit from validators from the last block
    public byte[] dataHash; // transactions
    public byte[] validatorsHash; // validators for the current block
    public byte[] nextValidatorsHash; // validators for the next block
    public byte[] consensusHash; // consensus params for current block
    public byte[] appHash; // state after txs from the previous block
    public byte[] lastResultsHash; // root hash of all results from the txs from the previous block
    public byte[] evidenceHash; // evidence included in the block
    public byte[] proposerAddress; // original proposer of the block

    public static void writeObject(ObjectWriter writer, LightHeader obj) {
        obj.writeObject(writer);
    }

    public static LightHeader readObject(ObjectReader reader) {
        LightHeader obj = new LightHeader();
        reader.beginList();
        obj.version = reader.read(Consensus.class);
        obj.chainId = reader.readString();
        obj.height = reader.readBigInteger();
        obj.time = reader.read(Timestamp.class);
        obj.lastBlockId = reader.read(BlockID.class);
        obj.lastCommitHash = reader.readByteArray();
        obj.dataHash = reader.readByteArray();
        obj.validatorsHash = reader.readByteArray();
        obj.nextValidatorsHash = reader.readByteArray();
        obj.consensusHash = reader.readByteArray();
        obj.appHash = reader.readByteArray();
        obj.lastResultsHash = reader.readByteArray();
        obj.evidenceHash = reader.readByteArray();
        obj.proposerAddress = reader.readByteArray();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(14);
        writer.write(version);
        writer.write(chainId);
        writer.write(height);
        writer.write(time);
        writer.write(lastBlockId);
        writer.write(lastCommitHash);
        writer.write(dataHash);
        writer.write(validatorsHash);
        writer.write(nextValidatorsHash);
        writer.write(consensusHash);
        writer.write(appHash);
        writer.write(lastResultsHash);
        writer.write(evidenceHash);
        writer.write(proposerAddress);
        writer.end();
    }

    public byte[] hash() {
        byte[] hbz = Proto.encode(1, this.version.block);
        byte[] pbt = this.time.encode();
        byte[] bzbi = this.lastBlockId.encode();

        byte[][] all = new byte[][]{
                hbz,
                Proto.encode(1, this.chainId),
                Proto.encode(1, this.height),
                pbt,
                bzbi,
                Proto.encode(1, this.lastCommitHash),
                Proto.encode(1, this.dataHash),
                Proto.encode(1, this.validatorsHash),
                Proto.encode(1, this.nextValidatorsHash),
                Proto.encode(1, this.consensusHash),
                Proto.encode(1, this.appHash),
                Proto.encode(1, this.lastResultsHash),
                Proto.encode(1, this.evidenceHash),
                Proto.encode(1, this.proposerAddress)
        };

        return MerkleTree.merkleRootHash(all, 0, all.length);
    }
}