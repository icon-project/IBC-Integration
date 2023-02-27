package ibc.icon.structs;

import ibc.icon.structs.proto.core.client.Height;

// ClientState as defined by Tendermint Specifications

public class ClientState {
    public String chainId;
    public Fraction trustLevel;
    public Duration trustingPeriod;
    public Duration unbondingPeriod;
    public Duration maxClockDrift;
    public Height frozenHeight;
    public Height latestHeight;
    public ProofSpec proofSpecs;
    public boolean allowUpdateAfterExpiry;
    public boolean allowUpdateAfterMisbehaviour;
}
