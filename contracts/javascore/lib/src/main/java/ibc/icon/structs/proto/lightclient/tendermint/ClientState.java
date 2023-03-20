package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

import java.math.BigInteger;

public class ClientState {
    public String chainId;
    public Fraction trustLevel;

    // duration of the period since the LastestTimestamp during which the
    // submitted headers are valid for upgrade
    public Duration trustingPeriod;
    // duration of the staking unbonding period
    public Duration unbondingPeriod;
    // defines how much new (untrusted) header's Time can drift into the future.
    public Duration maxClockDrift;
    // Block height when the client was frozen due to a misbehaviour
    // ibc.core.client.v1.Height frozenHeight;
    public BigInteger frozenHeight;
    // Latest height the client was updated to
    public BigInteger latestHeight;
    // This flag, when set to true, will allow governance to recover a client
    // which has expired
    public boolean allowUpdateAfterExpiry;
    // This flag, when set to true, will allow governance to unfreeze a client
    // whose chain has experienced a misbehaviour event
    public boolean allowUpdateAfterMisbehaviour;

    public static void writeObject(ObjectWriter writer, ClientState obj) {
        obj.writeObject(writer);
    }

    public static ClientState readObject(ObjectReader reader) {
        ClientState obj = new ClientState();
        reader.beginList();
        obj.chainId = reader.readString();
        obj.trustLevel = reader.read(Fraction.class);
        obj.trustingPeriod = reader.read(Duration.class);
        obj.unbondingPeriod = reader.readNullable(Duration.class);
        obj.maxClockDrift = reader.read(Duration.class);
        obj.frozenHeight = reader.readNullable(BigInteger.class);
        obj.latestHeight = reader.readBigInteger();
        obj.allowUpdateAfterExpiry = reader.readBoolean();
        obj.allowUpdateAfterMisbehaviour = reader.readBoolean();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(9);
        writer.write(chainId);
        writer.write(trustLevel);
        writer.write(trustingPeriod);
        writer.writeNullable(unbondingPeriod);
        writer.write(maxClockDrift);
        writer.writeNullable(frozenHeight);
        writer.write(latestHeight);
        writer.write(allowUpdateAfterExpiry);
        writer.write(allowUpdateAfterMisbehaviour);
        writer.end();
    }

    public static ClientState fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return ClientState.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        ClientState.writeObject(writer, this);
        return writer.toByteArray();
    }

    public byte[] encode() {
        return ByteUtil.join(
                Proto.encode(1, chainId),
                Proto.encode(2, trustLevel.encode()),
                Proto.encode(3, trustingPeriod.encode()),
                Proto.encode(4, unbondingPeriod.encode()),
                Proto.encode(5, maxClockDrift.encode()),
                Proto.encode(6, frozenHeight),
                Proto.encode(7, latestHeight),
                Proto.encode(8, allowUpdateAfterExpiry),
                Proto.encode(9, allowUpdateAfterMisbehaviour));
    }

    public String getChainId() {
        return chainId;
    }

    public void setChainId(String chainId) {
        this.chainId = chainId;
    }

    public Fraction getTrustLevel() {
        return trustLevel;
    }

    public void setTrustLevel(Fraction trustLevel) {
        this.trustLevel = trustLevel;
    }

    public Duration getTrustingPeriod() {
        return trustingPeriod;
    }

    public void setTrustingPeriod(Duration trustingPeriod) {
        this.trustingPeriod = trustingPeriod;
    }

    public Duration getUnbondingPeriod() {
        return unbondingPeriod;
    }

    public void setUnbondingPeriod(Duration unbondingPeriod) {
        this.unbondingPeriod = unbondingPeriod;
    }

    public Duration getMaxClockDrift() {
        return maxClockDrift;
    }

    public void setMaxClockDrift(Duration maxClockDrift) {
        this.maxClockDrift = maxClockDrift;
    }

    public BigInteger getFrozenHeight() {
        return frozenHeight;
    }

    public void setFrozenHeight(BigInteger frozenHeight) {
        this.frozenHeight = frozenHeight;
    }

    public BigInteger getLatestHeight() {
        return latestHeight;
    }

    public void setLatestHeight(BigInteger latestHeight) {
        this.latestHeight = latestHeight;
    }

    public boolean isAllowUpdateAfterExpiry() {
        return allowUpdateAfterExpiry;
    }

    public void setAllowUpdateAfterExpiry(boolean allowUpdateAfterExpiry) {
        this.allowUpdateAfterExpiry = allowUpdateAfterExpiry;
    }

    public boolean isAllowUpdateAfterMisbehaviour() {
        return allowUpdateAfterMisbehaviour;
    }

    public void setAllowUpdateAfterMisbehaviour(boolean allowUpdateAfterMisbehaviour) {
        this.allowUpdateAfterMisbehaviour = allowUpdateAfterMisbehaviour;
    }

}