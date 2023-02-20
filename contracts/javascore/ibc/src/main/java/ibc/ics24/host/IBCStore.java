package ibc.ics24.host;

import java.math.BigInteger;

import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import score.Address;
import score.ArrayDB;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.VarDB;

public class IBCStore {
        // Commitments
        public static final DictDB<byte[], byte[]> commitments = Context.newDictDB("commitments", byte[].class);

        // Store
        public static final DictDB<String, Address> clientRegistry = Context.newDictDB("clientRegistry", Address.class);
        public static final DictDB<String, String> clientTypes = Context.newDictDB("clientTypes", String.class);
        public static final DictDB<String, Address> clientImpls = Context.newDictDB("clientImpls", Address.class);
        public static final DictDB<String, ConnectionEnd> connections = Context.newDictDB("connections",
                        ConnectionEnd.class);
        public static final BranchDB<String, DictDB<String, Channel>> channels = Context.newBranchDB("channels",
                        Channel.class);
        public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceSends = Context.newBranchDB(
                        "nextSequenceSends", BigInteger.class);
        public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceRecvs = Context.newBranchDB(
                        "nextSequenceRecvs", Address.class);
        public static final BranchDB<String, DictDB<String, BigInteger>> nextSequenceAcks = Context.newBranchDB(
                        "nextSequenceAcks", Address.class);
        public static final BranchDB<String, BranchDB<String, DictDB<BigInteger, BigInteger>>> packetReceipts = Context
                        .newBranchDB("packetReceipts", Address.class);
        public static final BranchDB<byte[], ArrayDB<Address>> capabilities = Context.newBranchDB("capabilities",
                        Address[].class);

        // Host parameters
        public static final VarDB<BigInteger> expectedTimePerBlock = Context.newVarDB("expectedTimePerBlock",
                        BigInteger.class);

        // Sequences for identifier
        public static final VarDB<BigInteger> nextClientSequence = Context.newVarDB("nextClientSequence",
                        BigInteger.class);
        public static final VarDB<BigInteger> nextConnectionSequence = Context.newVarDB("nextConnectionSequence",
                        BigInteger.class);
        public static final VarDB<BigInteger> nextChannelSequence = Context.newVarDB("nextChannelSequence",
                        BigInteger.class);
}
