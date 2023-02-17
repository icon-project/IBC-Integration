package ibc.icon.structs.proto.core.client;

import java.math.BigInteger;

public class Height {
    public BigInteger revisionNumber;
    public BigInteger revisionHeight;

    public BigInteger getRevisionNumber() {
        return revisionNumber;
    }
    public void setRevisionNumber(BigInteger revisionNumber) {
        this.revisionNumber = revisionNumber;
    }
    public BigInteger getRevisionHeight() {
        return revisionHeight;
    }
    public void setRevisionHeight(BigInteger revisionHeight) {
        this.revisionHeight = revisionHeight;
    }
}
