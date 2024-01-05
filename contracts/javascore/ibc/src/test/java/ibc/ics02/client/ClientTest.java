package ibc.ics02.client;

import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.icon.test.MockContract;
import icon.ibc.interfaces.ILightClient;
import icon.ibc.interfaces.ILightClientScoreInterface;
import icon.ibc.structs.messages.MsgCreateClient;
import icon.ibc.structs.messages.MsgUpdateClient;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.*;

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

        // Act
        client.invoke(owner, "registerClient", msg.getClientType(), lightClient.getAddress());
        client.invoke(owner, "_createClient", msg);

        // Assert
        assertEquals(BigInteger.ONE, client.call("getNextClientSequence"));
        verify(lightClient.mock).createClient(expectedClientId, msg.getClientState(), msg.getConsensusState());
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

        // Act
        client.invoke(owner, "_updateClient", msg);

        // Assert
        verify(lightClient.mock).updateClient(msg.getClientId(), msg.getClientMessage());
    }
}
