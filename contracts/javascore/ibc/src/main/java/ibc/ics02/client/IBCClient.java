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
        UpdateClientResponse response = client.createClient(clientId, msg.getClientState(), msg.getConsensusState());

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);
        Height updateHeight = Height.decode(response.getHeight());
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                updateHeight.getRevisionNumber(),
                updateHeight.getRevisionHeight());

        sendBTPMessage(clientId, ByteUtil.join(clientKey, response.getClientStateCommitment()));
        sendBTPMessage(clientId, ByteUtil.join(consensusKey, response.getConsensusStateCommitment()));

        return clientId;
    }

    public byte[] _updateClient(MsgUpdateClient msg) {
        String clientId = msg.getClientId();
        ILightClient client = getClient(clientId);

        UpdateClientResponse response = client.updateClient(clientId, msg.getClientMessage());

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(clientId);

        Height updateHeight = Height.decode(response.getHeight());
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(clientId,
                updateHeight.getRevisionNumber(),
                updateHeight.getRevisionHeight());

        sendBTPMessage(clientId, ByteUtil.join(clientKey, response.getClientStateCommitment()));
        sendBTPMessage(clientId, ByteUtil.join(consensusKey, response.getConsensusStateCommitment()));

        return response.getHeight();
    }

    private String generateClientIdentifier(String clientType) {
        BigInteger currClientSequence = nextClientSequence.getOrDefault(BigInteger.ZERO);
        String identifier = clientType + "-" + currClientSequence.toString();
        nextClientSequence.set(currClientSequence.add(BigInteger.ONE));
        return identifier;
    }

}
