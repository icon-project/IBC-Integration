package ibc.ics02.client;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.ics24.host.IBCStore;
import java.math.BigInteger;
import score.Address;

public class IBCClient {

    Logger logger = new Logger("ibc-core");

    /**
     * Registers a client to registry
     *
     * @param clientType  Type of client
     * @param lightClient Light client contract address
     */
    public void registerClient(String clientType, Address lightClient) {
        NullChecker.requireNotNull(IBCStore.clientRegistry.get(clientType), "Already registered");
        IBCStore.clientRegistry.set(clientType, lightClient);
    }

    public String createClient(MsgCreateClient msg) {
        String clientType = msg.clientType;
        NullChecker.requireNotNull(clientType, "Client Type cannot be null");

        Address lightClientAddr = IBCStore.clientRegistry.get(clientType);
        NullChecker.requireNotNull(clientType, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        IBCStore.clientImpls.set(clientId, lightClientAddr);
        ILightClient client = new ILightClientScoreInterface(lightClientAddr);
        UpdateClientResponse response = client.createClient(clientId, msg.clientState, msg.consensusState);

        // TODO
        // commitments[keccak256(IBCCommitment.clientStatePath(clientId))] =
        // clientStateCommitment;
        // commitments[IBCCommitment.consensusStateCommitmentKey(
        // clientId, update.height.revision_number,
        // response.update.height.revision_height)] = update.consensusStateCommitment;

        return clientId;
    }

    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.clientId;
        NullChecker.requireNotNull(clientId, "ClientId cannot be null");

        ILightClient client = new ILightClientScoreInterface(IBCStore.clientImpls.get(clientId));

        // require(commitments[IBCCommitment.clientStateCommitmentKey(msg_.clientId)] !=
        // bytes32(0));
        UpdateClientResponse response = client.updateClient(msg.clientId, msg.clientMessage);

        // update commitments
        // commitments[keccak256(IBCCommitment.clientStatePath(msg_.clientId))] =
        // clientStateCommitment;
        // for (uint256 i = 0; i < updates.length; i++) {
        // commitments[IBCCommitment.consensusStateCommitmentKey(
        // msg_.clientId, updates[i].height.revision_number,
        // updates[i].height.revision_height)] = updates[i].consensusStateCommitment;
        // }
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = IBCStore.nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        IBCStore.nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
