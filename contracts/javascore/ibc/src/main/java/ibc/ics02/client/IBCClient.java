package ibc.ics02.client;

import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.MsgCreateClient;
import ibc.icon.structs.MsgUpdateClient;
import ibc.ics24.host.IBCStore;
import score.Address;
import score.annotation.External;

import java.math.BigInteger;

public class IBCClient extends IBCStore {

    Logger logger = new Logger("ibc-core");

    /**
     * Registers a client to registry
     *
     * @param clientType  Type of client
     * @param lightClient Light client contract address
     */
    @External
    public void registerClient(String clientType, Address lightClient) {
        NullChecker.requireNotNull(clientRegistry.get(clientType), "Already registered");
        clientRegistry.set(clientType, lightClient);
    }

    @External
    public void createClient(MsgCreateClient msg) {
        String clientType = msg.clientType;
        NullChecker.requireNotNull(clientType, "Client Type cannot be null");

        Address lightClientAddr = clientRegistry.get(clientType);
        NullChecker.requireNotNull(clientType, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        clientImplementations.set(clientId, lightClientAddr);

        //
    }

    @External
    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.clientId;
        NullChecker.requireNotNull(clientId, "ClientId cannot be null");

        Address lightClientAddr = clientImplementations.get(clientId);
        NullChecker.requireNotNull(lightClientAddr, "Invalid client id");

        //
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }


}
