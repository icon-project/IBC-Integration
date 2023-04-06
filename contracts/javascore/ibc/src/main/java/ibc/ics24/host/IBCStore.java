package ibc.ics24.host;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.IIBCHost;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.NullChecker;
import ibc.ics05.port.ModuleManager;
import score.*;
import score.annotation.External;

import java.math.BigInteger;

public abstract class IBCStore extends ModuleManager implements IIBCHost {
    private static final String COMMITMENTS = "commitments";
    private static final String CLIENT_REGISTRY = "clientRegistry";
    private static final String CLIENT_TYPES = "clientTypes";
    private static final String CLIENT_IMPLEMENTATIONS = "clientImplementations";
    private static final String CONNECTIONS = "connections";
    private static final String CHANNELS = "channels";
    private static final String NEXT_SEQUENCE_SENDS = "nextSequenceSends";
    private static final String NEXT_SEQUENCE_RECEIVES = "nextSequenceReceives";
    private static final String NEXT_SEQUENCE_ACKNOWLEDGEMENTS = "nextSequenceAcknowledgements";
    private static final String PACKET_RECEIPTS = "packetReceipts";
    private static final String CAPABILITIES = "capabilities";
    private static final String EXPECTED_TIME_PER_BLOCK = "expectedTimePerBlock";
    private static final String NEXT_CLIENT_SEQUENCE = "nextClientSequence";
    private static final String NEXT_CONNECTION_SEQUENCE = "nextConnectionSequence";
    private static final String NEXT_CHANNEL_SEQUENCE = "nextChannelSequence";
    private static final String BTP_NETWORK_ID = "btpNetworkId";

    // DB Variables
    // Commitments
    public static final DictDB<byte[], byte[]> commitments = Context.newDictDB(COMMITMENTS, byte[].class);

    // Store
    // clientType => clientImpl
    public static final DictDB<String, Address> clientRegistry = Context.newDictDB(CLIENT_REGISTRY, Address.class);
    // clientID => clientType
    public static final DictDB<String, String> clientTypes = Context.newDictDB(CLIENT_TYPES, String.class);
    // clientID => clientImpl
    public static final DictDB<String, Address> clientImplementations = Context.newDictDB(CLIENT_IMPLEMENTATIONS,
            Address.class);

    public static final DictDB<String, byte[]> connections = Context.newDictDB(CONNECTIONS, byte[].class);
    public static final BranchDB<String, DictDB<String, byte[]>> channels = Context.newBranchDB(CHANNELS, byte[].class);

    public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceSends = Context
            .newBranchDB(NEXT_SEQUENCE_SENDS, BigInteger.class);
    public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceReceives = Context
            .newBranchDB(NEXT_SEQUENCE_RECEIVES, BigInteger.class);
    public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceAcknowledgements = Context
            .newBranchDB(NEXT_SEQUENCE_ACKNOWLEDGEMENTS, BigInteger.class);
    public static final BranchDB<String, BranchDB<String, DictDB<BigInteger, Boolean>>> packetReceipts = Context
            .newBranchDB(PACKET_RECEIPTS, Boolean.class);

    public static final BranchDB<byte[], ArrayDB<Address>> capabilities = Context.newBranchDB(CAPABILITIES, Address.class);

    // Host Parameters
    public static final VarDB<BigInteger> expectedTimePerBlock = Context.newVarDB(EXPECTED_TIME_PER_BLOCK, BigInteger.class);

    // Sequences for identifiers
    public static final VarDB<BigInteger> nextClientSequence = Context.newVarDB(NEXT_CLIENT_SEQUENCE, BigInteger.class);
    public static final VarDB<BigInteger> nextConnectionSequence = Context.newVarDB(NEXT_CONNECTION_SEQUENCE,
            BigInteger.class);
    public static final VarDB<BigInteger> nextChannelSequence = Context.newVarDB(NEXT_CHANNEL_SEQUENCE, BigInteger.class);

    public static final DictDB<String, Integer> btpNetworkId = Context.newDictDB(BTP_NETWORK_ID, Integer.class);

    @External(readonly = true)
    public byte[] getCommitment(byte[] key) {
        return commitments.get(key);
    }

    @External(readonly = true)
    public Address getClientRegistry(String type) {
        return clientRegistry.get(type);
    }

    @External(readonly = true)
    public String getClientType(String clientId) {
        return clientTypes.get(clientId);
    }

    @External(readonly = true)
    public Address getClientImplementation(String clientId) {
        return clientImplementations.get(clientId);
    }

    @External(readonly = true)
    public byte[] getConnection(String connectionId) {
        return connections.get(connectionId);
    }

    @External(readonly = true)
    public byte[] getChannel(String portId, String channelId) {
        return channels.at(portId).get(channelId);
    }

    @External(readonly = true)
    public BigInteger getNextSequenceSend(String portId, String channelId) {
        return nextSequenceSends.at(portId).get(channelId);
    }

    @External(readonly = true)
    public BigInteger getNextSequenceReceive(String portId, String channelId) {
        return nextSequenceReceives.at(portId).get(channelId);
    }

    @External(readonly = true)
    public BigInteger getNextSequenceAcknowledgement(String portId, String channelId) {
        return nextSequenceAcknowledgements.at(portId).get(channelId);
    }

    @External(readonly = true)
    public boolean getPacketReceipt(String portId, String channelId, BigInteger sequence) {
        return packetReceipts.at(portId).at(channelId).get(sequence);
    }

    @External(readonly = true)
    public String[] getCapability(byte[] name) {
        ArrayDB<Address> arrayDB = capabilities.at(name);
        final int size = arrayDB.size();
        String[] capability = new String[size];
        for (int i = 0; i < size; i++) {
            capability[i] = arrayDB.get(i).toString();
        }

        return capability;
    }

    @External(readonly = true)
    public BigInteger getExpectedTimePerBlock() {
        return expectedTimePerBlock.get();
    }

    @External(readonly = true)
    public BigInteger getNextClientSequence() {
        return nextClientSequence.get();
    }

    @External(readonly = true)
    public BigInteger getNextConnectionSequence() {
        return nextConnectionSequence.get();
    }

    @External(readonly = true)
    public BigInteger getNextChannelSequence() {
        return nextChannelSequence.get();
    }

    @External(readonly = true)
    public byte[] getClientState(String clientId) {
        return getClient(clientId).getClientState(clientId);
    }

    @External(readonly = true)
    public byte[] getConsensusState(String clientId, byte[] height) {
        return getClient(clientId).getConsensusState(clientId, height);
    }

    @External(readonly = true)
    public byte[] getPacketCommitment(String portId, String channelId, BigInteger sequence) {
        byte[] key = IBCCommitment.packetCommitmentKey(portId, channelId, sequence);
        return commitments.get(key);
    }

    @External(readonly = true)
    public byte[] getPacketAcknowledgementCommitment(String portId, String channelId, BigInteger sequence) {
        byte[] key = IBCCommitment.packetAcknowledgementCommitmentKey(portId, channelId, sequence);
        return commitments.get(key);
    }

    @External(readonly = true)
    public boolean hasPacketReceipt(String portId, String channelId, BigInteger sequence) {
        return packetReceipts.at(portId).at(channelId).getOrDefault(sequence, false);
    }

    @External(readonly = true)
    public int getBTPNetworkId(String clientId) {
        return btpNetworkId.get(clientId);
    }

    public ILightClient getClient(String clientId) {
        Address address = clientImplementations.get(clientId);
        NullChecker.requireNotNull(address, "Client does not exist");
        return new ILightClientScoreInterface(address);
    }

    public void setBTPNetworkId(String clientId, int btpNetworkId) {
        this.btpNetworkId.set(clientId, btpNetworkId);
    }

}
