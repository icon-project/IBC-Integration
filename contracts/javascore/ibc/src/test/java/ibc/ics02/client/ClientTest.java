package ibc.ics02.client;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.score.util.ByteUtil;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.structs.messages.ConsensusStateUpdate;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.icon.structs.messages.UpdateClientResponse;
import ibc.icon.test.MockContract;
import ibc.ics24.host.IBCCommitment;
import test.proto.core.client.Client.Height;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doNothing;
import static org.mockito.Mockito.spy;

public class ClientTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score client;
    private MockContract<ILightClient> lightClient;
    private IBCClient clientSpy;

    @BeforeEach
    public void setup() throws Exception {
        client = sm.deploy(owner, IBCClient.class);
        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);

        clientSpy = (IBCClient) spy(client.getInstance());
        client.setInstance(clientSpy);
        doNothing().when(clientSpy).sendBTPMessage(any(String.class), any(byte[].class));

    }

    @Test
    void registerClient_alreadyRegistered() {
        // Arrange
        String clientType = "clientType";
        client.invoke(owner, "registerClient", clientType, lightClient.getAddress());

        // Act & Assert
        String expectedErrorMessage = "Already registered";
        Executable registerWithSameType = () -> {
            client.invoke(owner, "registerClient", clientType,
                    lightClient.getAddress());
        };
        AssertionError e = assertThrows(AssertionError.class, registerWithSameType);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void createClient_withoutType() {
        // Arrange
        MsgCreateClient msg = new MsgCreateClient();
        msg.setClientType("type");
        msg.setConsensusState(new byte[0]);
        msg.setClientState(new byte[0]);

        // Act & Assert
        String expectedErrorMessage = "Register client before creation.";
        Executable createWithoutRegisterExecutable = () -> client.invoke(owner,
                "_createClient", msg);
        AssertionError e = assertThrows(AssertionError.class,
                createWithoutRegisterExecutable);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void createClient() {
        // Arrange
        MsgCreateClient msg = new MsgCreateClient();

        msg.setClientType("type");
        msg.setConsensusState(new byte[2]);
        msg.setClientState(new byte[3]);
        msg.setBtpNetworkId(4);
        String expectedClientId = msg.getClientType() + "-0";

        byte[] clientStateCommitment = new byte[4];
        byte[] consensusStateCommitment = new byte[5];
        Height consensusHeight = Height.newBuilder()
                .setRevisionHeight(1)
                .setRevisionNumber(2).build();
        ConsensusStateUpdate update = new ConsensusStateUpdate(consensusStateCommitment, consensusHeight.toByteArray());
        UpdateClientResponse response = new UpdateClientResponse(clientStateCommitment, update, true);
        when(lightClient.mock.createClient(msg.getClientType() + "-" + BigInteger.ZERO, msg.getClientState(),
                msg.getConsensusState())).thenReturn(response);

        // Act
        client.invoke(owner, "registerClient", msg.getClientType(), lightClient.getAddress());
        client.invoke(owner, "_createClient", msg);

        // Assert
        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(expectedClientId);
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(expectedClientId,
                BigInteger.valueOf(consensusHeight.getRevisionNumber()),
                BigInteger.valueOf(consensusHeight.getRevisionHeight()));
        verify(clientSpy).sendBTPMessage(expectedClientId, ByteUtil.join(clientKey, clientStateCommitment));
        verify(clientSpy).sendBTPMessage(expectedClientId, ByteUtil.join(consensusKey, consensusStateCommitment));

        assertEquals(BigInteger.ONE, client.call("getNextClientSequence"));
        assertEquals(4, client.call("getBTPNetworkId", expectedClientId));
    }

    @Test
    public void updateClient_NonExistingClient() {
        // Arrange
        MsgUpdateClient updateMsg = new MsgUpdateClient();
        updateMsg.setClientId("nonType" + "-0");
        updateMsg.setClientMessage(new byte[4]);

        // Act & Assert
        String expectedErrorMessage = "Client does not exist";
        Executable updateWithoutCreate = () -> client.invoke(owner, "_updateClient", updateMsg);
        AssertionError e = assertThrows(AssertionError.class,
                updateWithoutCreate);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    public void updateClient() {
        // Arrange
        createClient();
        MsgUpdateClient msg = new MsgUpdateClient();
        msg.setClientId("type-0");
        msg.setClientMessage(new byte[4]);

        byte[] clientStateCommitment = new byte[6];
        byte[] consensusStateCommitment = new byte[7];

        Height consensusHeight = Height.newBuilder()
                .setRevisionHeight(1)
                .setRevisionNumber(2).build();

        ConsensusStateUpdate update = new ConsensusStateUpdate(consensusStateCommitment, consensusHeight.toByteArray());

        UpdateClientResponse response = new UpdateClientResponse(clientStateCommitment, update, true);

        when(lightClient.mock.updateClient(msg.getClientId(), msg.getClientMessage())).thenReturn(response);

        // Act
        client.invoke(owner, "_updateClient", msg);

        // Assert
        verify(lightClient.mock).updateClient(msg.getClientId(), msg.getClientMessage());

        byte[] clientKey = IBCCommitment.clientStateCommitmentKey(msg.getClientId());
        byte[] consensusKey = IBCCommitment.consensusStateCommitmentKey(msg.getClientId(),
                BigInteger.valueOf(consensusHeight.getRevisionNumber()),
                BigInteger.valueOf(consensusHeight.getRevisionHeight()));
        verify(clientSpy).sendBTPMessage(msg.getClientId(), ByteUtil.join(clientKey, clientStateCommitment));
        verify(clientSpy).sendBTPMessage(msg.getClientId(), ByteUtil.join(consensusKey, consensusStateCommitment));
    }
}
