package ibc.ics24.host;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.IIBCHost;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.NullChecker;
import ibc.ics05.port.ModuleManager;
import score.*;
import score.annotation.External;
import scorex.util.ArrayList;
import scorex.util.HashMap;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

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
    private static final String PACKET_HEIGHTS= "packetHeights";
    private static final String CAPABILITIES = "capabilities";
    private static final String PORT_IDS = "portIds";
    private static final String EXPECTED_TIME_PER_BLOCK = "expectedTimePerBlock";
    private static final String NEXT_CLIENT_SEQUENCE = "nextClientSequence";
    private static final String NEXT_CONNECTION_SEQUENCE = "nextConnectionSequence";
    private static final String NEXT_CHANNEL_SEQUENCE = "nextChannelSequence";
    private static final String BTP_NETWORK_ID = "btpNetworkId";
    private static final String TIMEOUT_REQUESTS = "timeout_requests";

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
    public static final BranchDB<String, BranchDB<String, DictDB<BigInteger, Long>>> packetHeights = Context
            .newBranchDB(PACKET_HEIGHTS, Long.class);

    public static final DictDB<byte[],Address> capabilities = Context.newDictDB(CAPABILITIES, Address.class);
    public static final ArrayDB<String> portIds = Context.newArrayDB(PORT_IDS, String.class);
    // Host Parameters
    public static final VarDB<BigInteger> expectedTimePerBlock = Context.newVarDB(EXPECTED_TIME_PER_BLOCK, BigInteger.class);

    // Sequences for identifiers
    public static final VarDB<BigInteger> nextClientSequence = Context.newVarDB(NEXT_CLIENT_SEQUENCE, BigInteger.class);
    public static final VarDB<BigInteger> nextConnectionSequence = Context.newVarDB(NEXT_CONNECTION_SEQUENCE,
            BigInteger.class);
    public static final VarDB<BigInteger> nextChannelSequence = Context.newVarDB(NEXT_CHANNEL_SEQUENCE, BigInteger.class);

    public static final DictDB<String, Integer> btpNetworkId = Context.newDictDB(BTP_NETWORK_ID, Integer.class);
    public static final DictDB<byte[], Boolean> timeoutRequests = Context.newDictDB(TIMEOUT_REQUESTS, Boolean.class);

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
           return packetReceipts.at(portId).at(channelId).getOrDefault(sequence, false);
    }

    @External(readonly = true)
    public Address getCapability(byte[] name) {
        return capabilities.get(name);
    }

    @External(readonly = true)
    public List<String> getAllPorts() {
        List<String> ports = new ArrayList<>();
        final int size = portIds.size();
        for (int i = 0; i < size; i++) {
            ports.add(i, portIds.get(i));
        }
        return ports;
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
    public byte[] getLatestHeight(String clientId) {
        return getClient(clientId).getLatestHeight(clientId);
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
    public Map<String, Long> getPacketHeights(String portId, String channelId, int startSequence, int endSequence) {
        DictDB<BigInteger, Long> packets = packetHeights.at(portId).at(channelId);
        Map<String, Long> heights = new HashMap<>();
        for (int i = startSequence; i <= endSequence; i++) {
            BigInteger sequence = BigInteger.valueOf(i);
            Long height = packets.get(sequence);
            if (height != null){
                heights.put(sequence.toString(), height);
            }
        }

        return heights;
    }

    @External(readonly = true)
    public byte[] getPacketAcknowledgementCommitment(String portId, String channelId, BigInteger sequence) {
        byte[] key = IBCCommitment.packetAcknowledgementCommitmentKey(portId, channelId, sequence);
        return commitments.get(key);
    }

    @External(readonly = true)
    public List<Integer> getMissingPacketReceipts(String portId, String channelId, int startSequence, int endSequence) {
        DictDB<BigInteger, Boolean> receipts = packetReceipts.at(portId).at(channelId);
        List<Integer> missingReceipts = new ArrayList<>();
        for (int i = startSequence; i <= endSequence; i++) {
            BigInteger sequence = BigInteger.valueOf(i);
            if (!receipts.getOrDefault(sequence, false)) {
                missingReceipts.add(i);
            }
        }

        return missingReceipts;
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
        IBCStore.btpNetworkId.set(clientId, btpNetworkId);
    }

    @External(readonly = true)
    public boolean getRequestTimeout(byte[] packetHash) {
        return timeoutRequests.getOrDefault(packetHash, false);
    }

    public void setRequestTimeout(byte[] packetHash) {
        timeoutRequests.set(packetHash, true);
    }

}
