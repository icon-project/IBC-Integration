package ibc.ics02.client;

import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.ics24.host.IBCHost;
import ibc.ics24.host.IBCStore;
import icon.ibc.interfaces.ILightClient;
import icon.ibc.structs.messages.MsgCreateClient;
import icon.ibc.structs.messages.MsgUpdateClient;
import score.Address;
import score.Context;

import java.math.BigInteger;
import java.util.Map;

public class IBCClient extends IBCHost {

    static Logger logger = new Logger("ibc-core");

    public void registerClient(String clientType, Address lightClient, int hashType) {
        Context.require(clientRegistry.get(clientType) == null, "Already registered.");
        clientRegistry.set(clientType, lightClient);
        IBCStore.hashType.set(clientType, hashType);
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
        // hashMethod.set(clientId, msg.getHashMethod());

        ILightClient client = getClient(clientId);
        client.createClient(clientId, msg.getClientState(), msg.getConsensusState());

        return clientId;
    }

    public byte[] _updateClient(MsgUpdateClient msg) {
        String clientId = msg.getClientId();
        ILightClient client = getClient(clientId);

        Map<String, byte[]>  response = client.updateClient(clientId, msg.getClientMessage());

        return response.get("height");
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
