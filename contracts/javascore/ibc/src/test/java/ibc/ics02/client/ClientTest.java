package ibc.ics02.client;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.interfaces.ILightClientScoreInterface;
import ibc.icon.interfaces.ILightClient;
import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.commitment.MerklePrefix;
import ibc.icon.structs.proto.core.connection.ConnectionEnd;
import ibc.icon.structs.proto.core.connection.Counterparty;
import ibc.icon.structs.proto.core.connection.Version;
import ibc.icon.test.MockContract;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.any;

public class ClientTest extends TestBase {

    private final ServiceManager sm = getServiceManager();
    private final Account owner = sm.createAccount();
    private Score client;
    private MockContract<ILightClient> lightClient;

    @BeforeEach
    public void setup() throws Exception {
        client = sm.deploy(owner, IBCClient.class);
        lightClient = new MockContract<>(ILightClientScoreInterface.class, ILightClient.class, sm, owner);
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
        msg.clientType = "type";
        msg.consensusState = new byte[0];
        msg.clientState = new byte[0];

        // Act & Assert
        String expectedErrorMessage = "Register client before creation.";
        Executable createWithoutRegisterExecutable = () -> client.invoke(owner,
                "createClient", msg);
        AssertionError e = assertThrows(AssertionError.class,
                createWithoutRegisterExecutable);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void createClient() {
        // Arrange
        MsgCreateClient msg = new MsgCreateClient();
        msg.clientType = "type";
        msg.consensusState = new byte[2];
        msg.clientState = new byte[3];
        // TODO mock lightclient update response

        // Act
        client.invoke(owner, "registerClient", msg.clientType, lightClient.getAddress());
        client.invoke(owner, "createClient", msg);
        client.invoke(owner, "createClient", msg);

        // Assert
        verify(lightClient.mock).createClient(msg.clientType + "-" + BigInteger.ZERO, msg.clientState,
                msg.consensusState);
        verify(lightClient.mock).createClient(msg.clientType + "-" + BigInteger.ONE, msg.clientState,
                msg.consensusState);
        // TODO verify commitements
    }

    @Test
    public void updateClient_NonExistingClient() {
        // Arrange
        MsgUpdateClient updateMsg = new MsgUpdateClient();
        updateMsg.clientId = "nonType" + "-0";
        updateMsg.clientMessage = new byte[4];

        // Act & Assert
        String expectedErrorMessage = "Client does not exist";
        Executable updateWithoutCreate = () -> client.invoke(owner, "updateClient", updateMsg);
        AssertionError e = assertThrows(AssertionError.class,
                updateWithoutCreate);
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    public void updateClient_NonExistingCommitment() {
        // TODO
    }

    @Test
    public void updateClient() {
        // Arrange
        MsgCreateClient msg = new MsgCreateClient();
        msg.clientType = "type";
        msg.consensusState = new byte[2];
        msg.clientState = new byte[3];

        MsgUpdateClient updateMsg = new MsgUpdateClient();
        updateMsg.clientId = msg.clientType + "-0";
        updateMsg.clientMessage = new byte[4];

        // TODO mock lightclient update responses

        // Act
        client.invoke(owner, "registerClient", msg.clientType, lightClient.getAddress());
        client.invoke(owner, "createClient", msg);
        client.invoke(owner, "updateClient", updateMsg);

        // Assert
        verify(lightClient.mock).createClient(msg.clientType + "-" + BigInteger.ZERO, msg.clientState,
                msg.consensusState);
        verify(lightClient.mock).updateClient(updateMsg.clientId, updateMsg.clientMessage);
        // TODO verify commitements
    }
}
