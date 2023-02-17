package ibc.ics02.client;

import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.MsgCreateClient;
import ibc.icon.structs.MsgUpdateClient;
import ibc.ics24.host.IBCStore;
import java.math.BigInteger;
import score.Address;
import score.Context;
import score.annotation.External;

public class IBCClient {

    IBCStore store = new IBCStore();
    Logger logger = new Logger("ibc-core");

    /**
     * Registers a client to registry
     * @param clientType Type of client
     * @param lightClient Light client contract address
     */
    @External
    public void registerClient(String clientType, Address lightClient) {
        NullChecker.requireNotNull(store.clientRegistry.get(clientType), "Already registered");
        store.clientRegistry.set(clientType, lightClient);
    }

    @External
    public void createClient(MsgCreateClient msg) {
        String clientType = msg.clientType;
        NullChecker.requireNotNull(clientType, "Client Type cannot be null");

        Address lightClientAddr = store.clientRegistry.get(clientType);
        NullChecker.requireNotNull(clientType, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        store.clientImpls.set(clientId, lightClientAddr);

        //
    }

    @External
    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.clientId;
        NullChecker.requireNotNull(clientId, "ClientId cannot be null");

        Address lightClientAddr = store.clientImpls.get(clientId);
        NullChecker.requireNotNull(lightClientAddr, "Invalid client id");

        //
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = store.nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        store.nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }


}
