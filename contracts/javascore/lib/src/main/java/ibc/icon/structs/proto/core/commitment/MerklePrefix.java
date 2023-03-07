package ibc.icon.structs.proto.core.commitment;

import ibc.icon.score.util.Proto;

public class MerklePrefix {
    private byte[] keyPrefix;

    public byte[] getKeyPrefix() {
        return keyPrefix;
    }

    public void setKeyPrefix(byte[] keyPrefix) {
        this.keyPrefix = keyPrefix;
    }

    public byte[] encode() {
        return Proto.encode(1, keyPrefix);
    }

}
