package ibc.ics02.client;

import ibc.icon.interfaces.IIBCClient;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.ics24.host.IBCCommitment;
import ibc.ics24.host.IBCHost;
import score.Address;
import score.Context;

import java.math.BigInteger;

public class IBCClient extends IBCHost implements IIBCClient {

    Logger logger = new Logger("ibc-core");

    /**
     * Registers a client to registry
     *
     * @param clientType  Type of client
     * @param lightClient Light client contract address
     */
    public void registerClient(String clientType, Address lightClient) {
        Context.require(clientRegistry.get(clientType) == null, "Already registered.");
        clientRegistry.set(clientType, lightClient);
    }

    public String createClient(MsgCreateClient msg) {
        String clientType = msg.clientType;
        Address lightClientAddr = clientRegistry.get(clientType);
        NullChecker.requireNotNull(lightClientAddr, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        clientTypes.set(clientId, msg.clientType);
        clientImplementations.set(clientId, lightClientAddr);
        ILightClient client = getClient(clientId);
        UpdateClientResponse response = client.createClient(clientId, msg.clientState, msg.consensusState);
        Context.require(response.ok);

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        // update commitments
        // commitments.set(IBCCommitment.clientStateCommitmentKey(clientId), response.clientStateCommitment);
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                response.update.height.getRevisionNumber(),
                response.update.height.getRevisionHeight());
        // commitments.set(consensusKey, response.update.consensusStateCommitment);

        sendBTPMessage(ByteUtil.join(clientKey, response.clientStateCommitment));
        sendBTPMessage(ByteUtil.join(consensusKey, response.update.consensusStateCommitment));

        return clientId;
    }

    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.clientId;
        ILightClient client = getClient(clientId);

        // Should be required on client side
        // Context.require(commitments.get(IBCCommitment.clientStateCommitmentKey(clientId)) != null);
        UpdateClientResponse response = client.updateClient(clientId, msg.clientMessage);
        Context.require(response.ok);

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        // update commitments
        // commitments.set(IBCCommitment.clientStateCommitmentKey(clientId),response.clientStateCommitment);
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                response.update.height.getRevisionNumber(),
                response.update.height.getRevisionHeight());
        // commitments.set(consensusKey, response.update.consensusStateCommitment);

        sendBTPMessage(ByteUtil.join(clientKey, response.clientStateCommitment));
        sendBTPMessage(ByteUtil.join(consensusKey, response.update.consensusStateCommitment));
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
