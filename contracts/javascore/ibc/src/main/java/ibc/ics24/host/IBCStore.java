package ibc.ics24.host;

import score.*;

import java.math.BigInteger;

public abstract class IBCStore {
    private final String COMMITMENTS = "commitments";
    private final String CLIENT_REGISTRY = "clientRegistry";
    private final String CLIENT_TYPES = "clientTypes";
    private final String CLIENT_IMPLEMENTATIONS = "clientImplementations";
    private final String CONNECTIONS = "connections";
    private final String CHANNELS = "channels";
    private final String NEXT_SEQUENCE_SENDS = "nextSequenceSends";
    private final String NEXT_SEQUENCE_RECEIVES = "nextSequenceReceives";
    private final String NEXT_SEQUENCE_ACKNOWLEDGEMENTS = "nextSequenceAcknowledgements";
    private final String PACKET_RECEIPTS = "packetReceipts";
    private final String CAPABILITIES = "capabilities";
    private final String EXPECTED_TIME_PER_BLOCK = "expectedTimePerBlock";
    private final String NEXT_CLIENT_SEQUENCE = "nextClientSequence";
    private final String NEXT_CONNECTION_SEQUENCE = "nextConnectionSequence";
    private final String NEXT_CHANNEL_SEQUENCE = "nextChannelSequence";

    // DB Variables
    // Commitments
    public final DictDB<Byte[], Byte[]> commitments = Context.newDictDB(COMMITMENTS, Byte[].class);

    // Store
    // clientType => clientImpl
    public final DictDB<String, Address> clientRegistry = Context.newDictDB(CLIENT_REGISTRY, Address.class);
    // clientID => clientType
    public final DictDB<String, String> clientTypes = Context.newDictDB(CLIENT_TYPES, String.class);
    // clientID => clientImpl
    public final DictDB<String, Address> clientImplementations = Context.newDictDB(CLIENT_IMPLEMENTATIONS,
            Address.class);
    // TODO: connections, channels
    public final BranchDB<String, DictDB<String, BigInteger>> nextSequenceSends =
            Context.newBranchDB(NEXT_SEQUENCE_SENDS, BigInteger.class);
    public final BranchDB<String, DictDB<String, BigInteger>> nextSequenceReceives =
            Context.newBranchDB(NEXT_SEQUENCE_RECEIVES, BigInteger.class);
    public final BranchDB<String, DictDB<String, BigInteger>> nextSequenceAcknowledgements =
            Context.newBranchDB(NEXT_SEQUENCE_ACKNOWLEDGEMENTS, BigInteger.class);
    public final BranchDB<String, BranchDB<String, DictDB<BigInteger, BigInteger>>> packetReceipts =
            Context.newBranchDB(PACKET_RECEIPTS, BigInteger.class);
    public final BranchDB<Byte[], ArrayDB<Address>> capabilities = Context.newBranchDB(CAPABILITIES, Address.class);

    // Host Parameters
    public final VarDB<BigInteger> expectedTimePerBlock = Context.newVarDB(EXPECTED_TIME_PER_BLOCK, BigInteger.class);

    // Sequences for identifiers
    public final VarDB<BigInteger> nextClientSequence = Context.newVarDB(NEXT_CLIENT_SEQUENCE, BigInteger.class);
    public final VarDB<BigInteger> nextConnectionSequence = Context.newVarDB(NEXT_CONNECTION_SEQUENCE,
            BigInteger.class);
    public final VarDB<BigInteger> nextChannelSequence = Context.newVarDB(NEXT_CHANNEL_SEQUENCE, BigInteger.class);

}
