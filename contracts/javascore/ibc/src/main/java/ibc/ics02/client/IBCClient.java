package ibc.ics02.client;

import ibc.icon.interfaces.IIBCClient;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.messages.*;
import ibc.ics24.host.IBCCommitment;
import ibc.ics24.host.IBCHost;
import score.Address;
import score.Context;

import java.math.BigInteger;

public class IBCClient extends IBCHost implements IIBCClient {

    Logger logger = new Logger("ibc-core");

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
        CreateClientResponse response = client.createClient(clientId, msg.clientState, msg.consensusState);
        Context.require(response.ok);

        // update commitments
        commitments.set(IBCCommitment.clientStateCommitmentKey(clientId), response.clientStateCommitment);
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                response.update.height.getRevisionNumber(), response.update.height.getRevisionHeight());
        commitments.set(consensusKey, response.update.consensusStateCommitment);

        return clientId;
    }

    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.clientId;
        ILightClient client = getClient(clientId);

        Context.require(commitments.get(IBCCommitment.clientStateCommitmentKey(clientId)) != null);
        UpdateClientResponse response = client.updateClient(clientId, msg.clientMessage);
        Context.require(response.ok);

        // update commitments
        commitments.set(IBCCommitment.clientStateCommitmentKey(clientId), response.clientStateCommitment);
        for (ConsensusStateUpdate update : response.updates) {
            byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId, update.height.getRevisionNumber(),
                    update.height.getRevisionHeight());
            commitments.set(consensusKey, update.consensusStateCommitment);
        }
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
