package ibc.ics02.client;

import ibc.icon.interfaces.IIBCClient;
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
import score.Context;

public class IBCClient implements IIBCClient {

    Logger logger = new Logger("ibc-core");
    protected IBCStore store = new IBCStore();

    public void registerClient(String clientType, Address lightClient) {
        Context.require(store.clientRegistry.get(clientType) == null, "Already registered.");
        store.clientRegistry.set(clientType, lightClient);
    }

    public String createClient(MsgCreateClient msg) {
        String clientType = msg.clientType;
        Address lightClientAddr = store.clientRegistry.get(clientType);
        NullChecker.requireNotNull(lightClientAddr, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        store.clientImpls.set(clientId, lightClientAddr);
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
        Address lightClientAddr = store.clientImpls.get(clientId);
        NullChecker.requireNotNull(lightClientAddr, "Client does not exist");
        ILightClient client = new ILightClientScoreInterface(lightClientAddr);

        // require(commitments[IBCCommitment.clientStateCommitmentKey(msg_.clientId)] !=
        // bytes32(0));
        UpdateClientResponse response = client.updateClient(msg.clientId, msg.clientMessage);

        // TODO
        // commitments[keccak256(IBCCommitment.clientStatePath(msg_.clientId))] =
        // clientStateCommitment;
        // for (uint256 i = 0; i < updates.length; i++) {
        // commitments[IBCCommitment.consensusStateCommitmentKey(
        // msg_.clientId, updates[i].height.revision_number,
        // updates[i].height.revision_height)] = updates[i].consensusStateCommitment;
        // }
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = store.nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        store.nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
