package ibc.ics02.client;

import ibc.icon.interfaces.ILightClient;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.NullChecker;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.ics24.host.IBCCommitment;
import ibc.ics24.host.IBCHost;
import icon.proto.core.client.Height;
import score.Address;
import score.Context;

import java.math.BigInteger;

public class IBCClient extends IBCHost {

    Logger logger = new Logger("ibc-core");

    public void registerClient(String clientType, Address lightClient) {
        Context.require(clientRegistry.get(clientType) == null, "Already registered.");
        clientRegistry.set(clientType, lightClient);
    }

    public String createClient(MsgCreateClient msg) {
        String clientType = msg.getClientType();
        Address lightClientAddr = clientRegistry.get(clientType);
        NullChecker.requireNotNull(lightClientAddr, "Register client before creation.");

        String clientId = generateClientIdentifier(clientType);
        logger.println("Create Client: ", " clientId: ", clientId);

        clientTypes.set(clientId, msg.getClientType());
        clientImplementations.set(clientId, lightClientAddr);
        ILightClient client = getClient(clientId);
        UpdateClientResponse response = client.createClient(clientId, msg.getClientState(), msg.getConsensusState());
        Context.require(response.isOk());

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        // commitments.set(IBCCommitment.clientStateCommitmentKey(clientId),
        // response.clientStateCommitment);
        Height updateHeight = response.getUpdate().getHeight();
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                updateHeight.getRevisionNumber(),
                updateHeight.getRevisionHeight());
        // commitments.set(consensusKey, response.update.consensusStateCommitment);

        sendBTPMessage(ByteUtil.join(clientKey, response.getClientStateCommitment()));
        sendBTPMessage(ByteUtil.join(consensusKey, response.getUpdate().getConsensusStateCommitment()));

        return clientId;
    }

    public void updateClient(MsgUpdateClient msg) {
        String clientId = msg.getClientId();
        ILightClient client = getClient(clientId);

        // Should be required on client side
        // Context.require(commitments.get(IBCCommitment.clientStateCommitmentKey(clientId))
        // != null);
        UpdateClientResponse response = client.updateClient(clientId, msg.getClientMessage());
        Context.require(response.isOk());

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        // commitments.set(IBCCommitment.clientStateCommitmentKey(clientId),
        // response.clientStateCommitment);
        Height updateHeight = response.getUpdate().getHeight();
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                updateHeight.getRevisionNumber(),
                updateHeight.getRevisionHeight());
        // commitments.set(consensusKey, response.update.consensusStateCommitment);

        sendBTPMessage(ByteUtil.join(clientKey, response.getClientStateCommitment()));
        sendBTPMessage(ByteUtil.join(consensusKey, response.getUpdate().getConsensusStateCommitment()));
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
