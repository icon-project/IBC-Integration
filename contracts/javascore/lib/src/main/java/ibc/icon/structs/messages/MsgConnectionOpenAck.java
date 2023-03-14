package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;
import ibc.icon.structs.proto.core.connection.Version;

public class MsgConnectionOpenAck {
    public String connectionId;
    public byte[] clientStateBytes; // client state for chainA on chainB
    public Version version; // version that ChainB chose in ConnOpenTry
    public String counterpartyConnectionID;
    public byte[] proofTry; // proof that connectionEnd was added to ChainB state in ConnOpenTry
    public byte[] proofClient; // proof of client state on chainB for chainA
    public byte[] proofConsensus; // proof that chainB has stored ConsensusState of chainA on its client
    public Height proofHeight; // height that relayer conpublic classed proofTry
    public Height consensusHeight; // latest height of chainA that chainB has stored on its chainA client
}