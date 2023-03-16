package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ObjectReader;
import score.ObjectWriter;

import java.util.Arrays;

public class BlockID {
    public byte[] hash;
    public PartSetHeader partSetHeader;

    public static void writeObject(ObjectWriter writer, BlockID obj) {
        obj.writeObject(writer);
    }

    public static BlockID readObject(ObjectReader reader) {
        BlockID obj = new BlockID();
        reader.beginList();
        obj.hash = reader.readByteArray();
        obj.partSetHeader = reader.read(PartSetHeader.class);
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(hash);
        writer.write(partSetHeader);
        writer.end();
    }

    public boolean equals(BlockID obj) {
        if (!Arrays.equals(this.hash, obj.hash)) {
            return false;
        }

        if (!this.partSetHeader.total.equals(obj.partSetHeader.total)) {
            return false;
        }

        if (!Arrays.equals(this.partSetHeader.hash, obj.partSetHeader.hash)) {
            return false;
        }

        return true;
    }

    public byte[] encode() {
        byte[] blockId1 = Proto.encode(1, this.hash);
        byte[] blockId2 = Proto.encode(1, this.partSetHeader.total);
        byte[] blockId3 = Proto.encode(2, this.partSetHeader.hash);

        byte[] groupedPartSetHeader = Proto.encode(2, ByteUtil.join(blockId2, blockId3));

        return ByteUtil.join(blockId1, groupedPartSetHeader);
    }
}