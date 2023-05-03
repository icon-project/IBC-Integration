package ibc.ics02.client;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.ics24.host.IBCHost;
import score.Address;
import score.Context;

import java.math.BigInteger;
import java.util.Map;

public class IBCClient extends IBCHost {

    static Logger logger = new Logger("ibc-core");

    public void registerClient(String clientType, Address lightClient) {
        Context.require(clientRegistry.get(clientType) == null, "Already registered.");
        clientRegistry.set(clientType, lightClient);
    }

    public String _createClient(MsgCreateClient msg) {
        String clientType = msg.getClientType();
        Address lightClientAddr = clientRegistry.get(clientType);
        NullChecker.requireNotNull(lightClientAddr, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        clientTypes.set(clientId, msg.getClientType());
        clientImplementations.set(clientId, lightClientAddr);
        btpNetworkId.set(clientId, msg.getBtpNetworkId());

        ILightClient client = getClient(clientId);
        client.createClient(clientId, msg.getClientState(), msg.getConsensusState(), msg.getStoragePrefix());
        // byte[] clientStateCommitment = response.get("clientStateCommitment");
        // byte[] consensusStateCommitment = response.get("consensusStateCommitment");
        // byte[] height = response.get("height");

        // byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        // Height updateHeight = Height.decode(height);
        // byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
        //         updateHeight.getRevisionNumber(),
        //         updateHeight.getRevisionHeight());

        // sendBTPMessage(clientId, ByteUtil.join(clientKey, clientStateCommitment));
        // sendBTPMessage(clientId, ByteUtil.join(consensusKey, consensusStateCommitment));

        return clientId;
    }

    public byte[] _updateClient(MsgUpdateClient msg) {
        String clientId = msg.getClientId();
        ILightClient client = getClient(clientId);

        Map<String, byte[]>  response = client.updateClient(clientId, msg.getClientMessage());
        // byte[] clientStateCommitment = response.get("clientStateCommitment");
        // byte[] consensusStateCommitment = response.get("consensusStateCommitment");
        // byte[] height = response.get("height");
        // byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);

        // Height updateHeight = Height.decode(height);
        // byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
        //         updateHeight.getRevisionNumber(),
        //         updateHeight.getRevisionHeight());

        // sendBTPMessage(clientId, ByteUtil.join(clientKey, clientStateCommitment));
        // sendBTPMessage(clientId, ByteUtil.join(consensusKey, consensusStateCommitment));

        return response.get("height");
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
