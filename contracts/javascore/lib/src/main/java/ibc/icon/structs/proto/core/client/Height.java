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

    // public BigInteger toUint128() {
    // return (uint128(this.getRevisionNumber()) << 64) |
    // uint128(this.getRevisionHeight());
    // }

    public boolean isZero() {
        return this.getRevisionNumber().equals(0) && this.getRevisionHeight().equals(0);
    }

    public boolean lt(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) < 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                        && this.getRevisionHeight().compareTo(other.getRevisionHeight()) < 0);
    }

    public boolean lte(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) < 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                        && this.getRevisionHeight().compareTo(other.getRevisionHeight()) <= 0);
    }

    public boolean eq(Height other) {
        return this.getRevisionNumber().equals(other.getRevisionNumber())
                && this.getRevisionHeight().equals(other.getRevisionHeight());
    }

    public boolean gt(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) > 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                        && this.getRevisionHeight().compareTo(other.getRevisionHeight()) > 0);
    }

    public boolean gte(Height other) {
        return this.getRevisionNumber().compareTo(other.getRevisionNumber()) > 0
                || (this.getRevisionNumber().equals(other.getRevisionNumber())
                        && this.getRevisionHeight().compareTo(other.getRevisionHeight()) >= 0);
    }
}
