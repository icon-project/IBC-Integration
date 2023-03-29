package ibc.icon.structs.messages;

import java.math.BigInteger;
import java.util.List;

import icon.proto.core.client.Height;
import icon.proto.core.connection.Counterparty;
import icon.proto.core.connection.Version;
import scorex.util.ArrayList;

public class MsgConnectionOpenTry {
    private String previousConnectionId;
    private byte[] counterparty; // counterpartyConnectionIdentifier, counterpartyPrefix and
    // counterpartyClientIdentifier
    private BigInteger delayPeriod;
    private String clientId; // clientID of chainA
    private byte[] clientStateBytes; // clientState that chainA has for chainB
    private byte[][] counterpartyVersions; // supported versions of chain A
    private byte[] proofInit; // proof that chainA stored connectionEnd in state (on ConnOpenInit)
    private byte[] proofClient; // proof that chainA stored a light client of chainB
    private byte[] proofConsensus; // proof that chainA stored chainB's consensus state at consensus height
    private byte[] proofHeight; // height at which relayer conpublic classs proof of A storing connectionEnd in
    // state
    private byte[] consensusHeight; // latest height of chain B which chain A has stored in its chain B client

    public String getPreviousConnectionId() {
        return previousConnectionId;
    }

    public void setPreviousConnectionId(String previousConnectionId) {
        this.previousConnectionId = previousConnectionId;
    }

    public byte[] getCounterpartyRaw() {
        return counterparty;
    }

    public Counterparty getCounterparty() {
        return Counterparty.decode(counterparty);
    }

    public void setCounterparty(byte[] counterparty) {
        this.counterparty = counterparty;
    }

    public BigInteger getDelayPeriod() {
        return delayPeriod;
    }

    public void setDelayPeriod(BigInteger delayPeriod) {
        this.delayPeriod = delayPeriod;
    }

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public byte[] getClientStateBytes() {
        return clientStateBytes;
    }

    public void setClientStateBytes(byte[] clientStateBytes) {
        this.clientStateBytes = clientStateBytes;
    }

    public byte[][] getCounterpartyVersionsRaw() {
        return counterpartyVersions;
    }

    public List<Version> getCounterpartyVersions() {
        List<Version> versions = new ArrayList<>();
        for (int i = 0; i < counterpartyVersions.length; i++) {
            versions.add(Version.decode(counterpartyVersions[i]));
        }
        return versions;
    }

    public void setCounterpartyVersions(byte[][] counterpartyVersions) {
        this.counterpartyVersions = counterpartyVersions;
    }

    public byte[] getProofInit() {
        return proofInit;
    }

    public void setProofInit(byte[] proofInit) {
        this.proofInit = proofInit;
    }

    public byte[] getProofClient() {
        return proofClient;
    }

    public void setProofClient(byte[] proofClient) {
        this.proofClient = proofClient;
    }

    public byte[] getProofConsensus() {
        return proofConsensus;
    }

    public void setProofConsensus(byte[] proofConsensus) {
        this.proofConsensus = proofConsensus;
    }

    public byte[] getProofHeightRaw() {
        return proofHeight;
    }

    public Height getProofHeight() {
        return Height.decode(proofHeight);
    }

    public void setProofHeight(byte[] proofHeight) {
        this.proofHeight = proofHeight;
    }

    public byte[] getConsensusHeightRaw() {
        return consensusHeight;
    }

    public Height getConsensusHeight() {
        return Height.decode(consensusHeight);
    }

    public void setConsensusHeight(byte[] consensusHeight) {
        this.consensusHeight = consensusHeight;
    }

}
