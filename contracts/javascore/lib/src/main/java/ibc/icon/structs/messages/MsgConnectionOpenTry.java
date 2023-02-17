package ibc.icon.structs.messages;

import java.math.BigInteger;

import com.fasterxml.jackson.core.Version;

import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.connection.Counterparty;

public class MsgConnectionOpenTry {
    public String previousConnectionId;
    public Counterparty counterparty; // counterpartyConnectionIdentifier, counterpartyPrefix and
    // counterpartyClientIdentifier
    public BigInteger delayPeriod;
    public String clientId; // clientID of chainA
    public byte[] clientStateBytes; // clientState that chainA has for chainB
    public Version[] counterpartyVersions; // supported versions of chain A
    public byte[] proofInit; // proof that chainA stored connectionEnd in state (on ConnOpenInit)
    public byte[] proofClient; // proof that chainA stored a light client of chainB
    public byte[] proofConsensus; // proof that chainA stored chainB's consensus state at consensus height
    public Height proofHeight; // height at which relayer conpublic classs proof of A storing connectionEnd in
    // state
    public Height consensusHeight; // latest height of chain B which chain A has stored in its chain B client
}
