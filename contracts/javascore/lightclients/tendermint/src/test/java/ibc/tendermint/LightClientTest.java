package ibc.tendermint;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doNothing;

import java.math.BigInteger;


import com.google.protobuf.Timestamp;
import com.google.protobuf.Duration;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import com.ibc.core.client.v1.Height;

import score.Address;

import com.ibc.lightclients.tendermint.v1.*;
import com.tendermint.types.*;

public class LightClientTest extends LightClientTestBase {

    @BeforeEach
    protected void setup() throws Exception {
        super.setup();
    }

    @Test
    void createClient() throws Exception {
        // Arrange
        SignedHeader initialHeader = parseSignedHeader(1);

        // Act
        initializeClient(1);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getLatestHeight().getRevisionHeight(), initialHeader.getHeader().getHeight());
        assertEquals(clientState.getAllowUpdateAfterExpiry(), allowUpdateAfterExpiry);
        assertEquals(clientState.getAllowUpdateAfterMisbehaviour(), allowUpdateAfterMisbehaviour);
        assertEquals(clientState.getChainId(), initialHeader.getHeader().getChainId());
        assertEquals(clientState.getFrozenHeight(), Height.newBuilder().build());
        assertEquals(clientState.getMaxClockDrift(), maxClockDrift);
        assertEquals(clientState.getTrustLevel(), trustLevel);
        assertEquals(clientState.getTrustingPeriod(), trustingPeriod);
        assertEquals(clientState.getUnbondingPeriod(), Duration.getDefaultInstance());
    }

    @Test
    void createClient_withZeroDenomTrustLevel() throws Exception {
        // Arrange
        // Default is zero denominator
        trustLevel = Fraction.getDefaultInstance();
        String expectedErrorMessage = "trustLevel has zero Denominator";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> initializeClient(1));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void update_NonAdjacentInOrder() throws Exception {
        // Arrange
        SignedHeader lastHeader = parseSignedHeader(3);

        // Act
        initializeClient(1);
        updateClient(2, 1);
        updateClient(3, 2);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getLatestHeight().getRevisionHeight(), lastHeader.getHeader().getHeight());
        assertConsensusState(parseSignedHeader(1));
        assertConsensusState(parseSignedHeader(2));
        assertConsensusState(lastHeader);
    }

    @Test
    void update_NonAdjacentOutOfOrder() throws Exception {
        // Arrange
        SignedHeader lastHeader = parseSignedHeader(3);

        // Act
        initializeClient(1);
        updateClient(3, 1);
        updateClient(2, 1);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getLatestHeight().getRevisionHeight(), lastHeader.getHeader().getHeight());
        assertConsensusState(parseSignedHeader(1));
        assertConsensusState(parseSignedHeader(2));
        assertConsensusState(lastHeader);
    }

    @Test
    void updateMultiValidator() throws Exception {
        // Arrange
        blockSetPath = BLOCK_SET_MUTILPLE_VALIDATORS;
        SignedHeader lastHeader = parseSignedHeader(3);

        // Act
        initializeClient(1);
        updateClient(2, 1);
        updateClient(3, 2);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getLatestHeight().getRevisionHeight(), lastHeader.getHeader().getHeight());
        assertConsensusState(parseSignedHeader(1));
        assertConsensusState(parseSignedHeader(2));
        assertConsensusState(lastHeader);
    }

    @Test
    void updateAdjacentBlocks() throws Exception {
        // Arrange
        blockSetPath = BLOCK_SET_ADJACENT;
        SignedHeader lastHeader = parseSignedHeader(3);

        // Act
        initializeClient(1);
        updateClient(2, 1);
        updateClient(3, 2);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getLatestHeight().getRevisionHeight(), lastHeader.getHeader().getHeight());
        assertConsensusState(parseSignedHeader(1));
        assertConsensusState(parseSignedHeader(2));
        assertConsensusState(lastHeader);
    }

    @Test
    void updateConflictingHeader() throws Exception {
        // Arrange
        blockSetPath = BLOCK_SET_MALICIOUS;
        SignedHeader duplicatedHeader = parseSignedHeader(3);
        initializeClient(1);
        updateClient(2, 1);
        doNothing().when(clientSpy).checkValidity(
                any(ibc.lightclients.tendermint.v1.ClientState.class),
                any(ibc.lightclients.tendermint.v1.ConsensusState.class),
                any(ibc.lightclients.tendermint.v1.Header.class),
                any());

        // Act
        updateClient(3, 1);

        // Assert
        ClientState clientState = getClientState();
        assertEquals(clientState.getFrozenHeight().getRevisionHeight(), duplicatedHeader.getHeader().getHeight());
    }

    @Test
    void update_withNonTrustedHeight() throws Exception {
        // Arrange
        initializeClient(1);
        String expectedErrorMessage = "LC: consensusState not found at trusted height";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> updateClient(3, 2));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void update_withTrustOnFutureBlock() throws Exception {
        // Arrange
        initializeClient(1);
        updateClient(3, 1);
        String expectedErrorMessage = "LC: Trusted height is higher than untrusted header height";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> updateClient(2, 3));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void update_withoutInit() throws Exception {
        // Arrange
        String expectedErrorMessage = "LC: client state is invalid";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> updateClient(3, 2));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void doubleUpdate() throws Exception {
        // Arrange
        initializeClient(1);
        updateClient(3, 1);
        String expectedErrorMessage = "LC: This header has already been submitted";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> updateClient(3, 1));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void initOnlyByIBCHandler() {
        // Arrange
        Address handlerAddress = ibcHandler.getAddress();
        ibcHandler = sm.createAccount();
        String expectedErrorMessage = "Only the IBC handler: " + handlerAddress + " is allowed";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> initializeClient(1));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void updateOnlyByIBCHandler() throws Exception {
        // Arrange
        initializeClient(1);
        Address handlerAddress = ibcHandler.getAddress();
        ibcHandler = sm.createAccount();
        String expectedErrorMessage = "Only the IBC handler: " + handlerAddress + " is allowed";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> updateClient(2, 1));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void alreadyInitialized() throws Exception {
        // Arrange
        initializeClient(1);
        String expectedErrorMessage = "Client already exists";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class, () -> initializeClient(2));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void getTimestampAtHeight() throws Exception {
        // Arrange
        initializeClient(1);
        updateClient(2, 1);
        SignedHeader header1 = parseSignedHeader(1);
        SignedHeader header2 = parseSignedHeader(2);

        Height height1 =  Height.newBuilder().setRevisionHeight(header1.getHeader().getHeight()).build();
        Height height2 =  Height.newBuilder().setRevisionHeight(header2.getHeader().getHeight()).build();
        long expectedTime1 = header1.getHeader().getTime().getSeconds();
        long expectedTime2 = header2.getHeader().getTime().getSeconds();

        // Act
        BigInteger t1 = (BigInteger) client.call("getTimestampAtHeight", clientId, height1.toByteArray());
        BigInteger t2 = (BigInteger) client.call("getTimestampAtHeight", clientId, height2.toByteArray());

        // Assert
        assertEquals(expectedTime1, t1.longValue());
        assertEquals(expectedTime2, t2.longValue());
    }

    @Test
    void getTimestampAtHeight_noConsensusState() throws Exception {
        // Arrange
        Height height = Height.newBuilder().setRevisionHeight(1).build();
        String expectedErrorMessage = "height: " + height.getRevisionHeight()
                + " does not have a consensus state";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class,
                () -> client.call("getTimestampAtHeight", clientId, height.toByteArray()));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void getLatestHeight() throws Exception {
        // Arrange
        SignedHeader header1 = parseSignedHeader(1);
        SignedHeader header2 = parseSignedHeader(2);

        BigInteger revision = TendermintHelper.getRevisionNumber(header1.getHeader().getChainId());
        Height height1 = Height.newBuilder().setRevisionHeight(header1.getHeader().getHeight()).setRevisionNumber(revision.intValue()).build();
        Height height2 = Height.newBuilder().setRevisionHeight(header2.getHeader().getHeight()).setRevisionNumber(revision.intValue()).build();

        // Act
        initializeClient(1);

        // Assert
        assertArrayEquals(height1.toByteArray(), (byte[]) client.call("getLatestHeight", clientId));

        // Act
        updateClient(2, 1);

        // Assert
        assertArrayEquals(height2.toByteArray(), (byte[]) client.call("getLatestHeight", clientId));
    }

    @Test
    void getLatestHeight_noClientState() throws Exception {
        // Arrange
        String expectedErrorMessage = "Client does not exist";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class,
                () -> client.call("getLatestHeight", clientId));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }

    @Test
    void updateExpired() throws Exception {
        // Arrange
        SignedHeader header = parseSignedHeader(1);

        long time = header.getHeader().getTime().getSeconds() * 1000 * 1000;
        long currentTime = System.currentTimeMillis() * 1000 + (sm.getBlock().getHeight() * 2_000_000);
        long period = currentTime - time;
        trustingPeriod = Duration.newBuilder()
                .setSeconds((period / (1000 * 1000)) - 3).build();
        initializeClient(1);

        String expectedErrorMessage = "header can't be expired";

        // Act & Assert
        AssertionError e = assertThrows(AssertionError.class,
                () -> updateClient(3, 1));
        assertTrue(e.getMessage().contains(expectedErrorMessage));
    }
}