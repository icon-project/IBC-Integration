package ibc.tendermint;

import java.io.File;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import org.mockito.stubbing.Answer;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.google.protobuf.ByteString;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;

import ibc.tendermint.light.TendermintLight.*;
import score.Context;
import foundation.icon.ee.util.Crypto;

import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.any;

public class LightClientTestBase extends TestBase {
    protected final ServiceManager sm = getServiceManager();
    protected final Account owner = sm.createAccount();
    protected final Account relayer = sm.createAccount();
    protected Score client;
    protected String clientId = "client-1";
    private static final DateTimeFormatter INSTANT_FORMAT = DateTimeFormatter.ISO_INSTANT;
    private static final BigInteger day = BigInteger.valueOf(86400);

    public static Fraction trustLevel;
    private static Duration trustingPeriod;
    private static Duration maxClockDrift;
    private static boolean allowUpdateAfterExpiry = false;
    private static boolean allowUpdateAfterMisbehaviour = false;
    protected final MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS);
    protected String blockSetPath = "src/test/java/ibc/tendermint/data/simple/";

    static {
        trustLevel = Fraction.newBuilder()
                .setNumerator(BigInteger.TWO.longValue())
                .setDenominator(BigInteger.valueOf(3).longValue()).build();

        trustingPeriod = Duration.newBuilder()
                .setSeconds(day.multiply(BigInteger.valueOf(10000)).longValue())
                .setNanos(0).build();

        maxClockDrift = Duration.newBuilder()
                .setSeconds(10)
                .setNanos(0).build();

    }

    private String getCommitPath(int order) {
        return blockSetPath + "commit." + order + ".json";

    }

    private String getValidatorPath(int order) {
        return blockSetPath + "validators." + order + ".json";
    }

    @BeforeEach
    protected void setup() throws Exception {
        contextMock.when(() -> Context.getBlockTimestamp())
                .thenReturn(System.currentTimeMillis() * 1000 + (sm.getBlock().getHeight() * 2_000_000));

        client = sm.deploy(owner, TendermintLightClient.class, owner.getAddress());

        TendermintLightClient clientSpy = (TendermintLightClient) spy(client.getInstance());
        client.setInstance(clientSpy);

        Mockito.doAnswer((Answer<Boolean>) invocation -> {
            final Object[] args = invocation.getArguments();
            return Crypto.verifySignature((String) args[0], (byte[]) args[1], (byte[]) args[2], (byte[]) args[3]);
        }).when(clientSpy).verifySig(any(String.class), any(byte[].class), any(byte[].class), any(byte[].class));
    }

    @AfterEach
    protected void teardown() {
        contextMock.close();
    }

    @Test
    void update_NonAdjacentInOrder() throws Exception {
        initializeClient(1);
        updateClient(2, 1);
        updateClient(3, 2);
    }

    @Test
    void update_NonAdjacentOutOfOrder() throws Exception {
        initializeClient(1);
        updateClient(3, 1);
        updateClient(2, 1);
    }

    @Test
    void updateMultiValidator() throws Exception {
        blockSetPath = "src/test/java/ibc/tendermint/data/multi-validator/";
        initializeClient(1);
        updateClient(2, 1);
        updateClient(3, 2);
    }

    private void initializeClient(int blockOrder) throws Exception {
        TmHeader tmHeader = TmHeader.newBuilder()
                .setSignedHeader(parseSignedHeader(blockOrder))
                .setValidatorSet(parseValidatorSet(blockOrder)).build();

        ClientState clientState = ClientState.newBuilder()
                .setChainId(tmHeader.getSignedHeader().getHeader().getChainId())
                .setTrustLevel(trustLevel)
                .setTrustingPeriod(trustingPeriod)
                .setMaxClockDrift(maxClockDrift)
                .setLatestHeight(tmHeader.getSignedHeader().getHeader().getHeight())
                .setAllowUpdateAfterExpiry(allowUpdateAfterExpiry)
                .setAllowUpdateAfterMisbehaviour(allowUpdateAfterMisbehaviour).build();

        MerkleRoot root = MerkleRoot.newBuilder()
                .setHash(tmHeader.getSignedHeader().getHeader().getAppHash()).build();

        ConsensusState consensusState = ConsensusState.newBuilder()
                .setTimestamp(tmHeader.getSignedHeader().getHeader().getTime())
                .setRoot(root)
                .setNextValidatorsHash(tmHeader.getSignedHeader().getHeader().getNextValidatorsHash()).build();

        client.invoke(owner, "createClient", clientId, clientState.toByteArray(),
                consensusState.toByteArray());
    }

    private void updateClient(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = createHeader(blockOrder, referenceBlock);
        printBytes(Crypto.hash("sha-256", tmHeader.toByteArray()));
        printBytes(Crypto.hash("sha-256",
                TmHeader.parseFrom(tmHeader.toByteArray()).toByteArray()));
        client.invoke(owner, "updateClient", clientId, tmHeader.toByteArray());
    }

    private TmHeader createHeader(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = TmHeader.newBuilder()
                .setSignedHeader(parseSignedHeader(blockOrder))
                .setValidatorSet(parseValidatorSet(blockOrder))
                .setTrustedHeight(parseSignedHeader(referenceBlock).getHeader().getHeight())
                .setTrustedValidators(parseValidatorSet(referenceBlock)).build();
        return tmHeader;
    }

    private SignedHeader parseSignedHeader(int blockOrder) throws Exception {

        ObjectMapper mapper = new ObjectMapper();
        String loc = getCommitPath(blockOrder);
        File file = new File(loc);
        String content = new String(Files.readAllBytes(Paths.get(file.toURI())));
        JsonNode json = mapper.readTree(content);

        JsonNode jsonHeader = json.get("signed_header").get("header");
        Consensus version = Consensus.newBuilder()
                .setBlock(jsonHeader.get("version").get("block").asInt()).build();

        LightHeader lightHeader = LightHeader.newBuilder()
                .setVersion(version)
                .setChainId(jsonHeader.get("chain_id").asText())
                .setHeight(jsonHeader.get("height").asInt())
                .setTime(jsonToTimestamp(jsonHeader.get("time")))
                .setLastBlockId(parseBlockId(jsonHeader.get("last_block_id")))
                .setLastCommitHash(jsonToBytes(jsonHeader.get("last_commit_hash")))
                .setDataHash(jsonToBytes(jsonHeader.get("data_hash")))
                .setValidatorsHash(jsonToBytes(jsonHeader.get("validators_hash")))
                .setNextValidatorsHash(jsonToBytes(jsonHeader.get("next_validators_hash")))
                .setConsensusHash(jsonToBytes(jsonHeader.get("consensus_hash")))
                .setAppHash(jsonToBytes(jsonHeader.get("app_hash")))
                .setLastResultsHash(jsonToBytes(jsonHeader.get("last_results_hash")))
                .setEvidenceHash(jsonToBytes(jsonHeader.get("evidence_hash")))
                .setProposerAddress(jsonToBytes(jsonHeader.get("proposer_address"))).build();

        JsonNode jsonCommit = json.get("signed_header").get("commit");

        Commit commit = Commit.newBuilder()
                .setHeight(jsonCommit.get("height").asInt())
                .setRound(jsonCommit.get("round").asInt())
                .setBlockId(parseBlockId(jsonCommit.get("block_id")))
                .addAllSignatures(parseCommitSig(jsonCommit.get("signatures"))).build();

        SignedHeader signedHeader = SignedHeader.newBuilder()
                .setHeader(lightHeader)
                .setCommit(commit).build();

        return signedHeader;
    }

    private ValidatorSet parseValidatorSet(int blockOrder) throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        String loc = getValidatorPath(blockOrder);
        File file = new File(loc);
        String content = new String(Files.readAllBytes(Paths.get(file.toURI())));
        JsonNode json = mapper.readTree(content);
        ValidatorSet.Builder validatorSet = ValidatorSet.newBuilder();
        List<Validator> validators = new ArrayList<>();
        json.get("validators").elements().forEachRemaining((node) -> {
            PublicKey publicKey = PublicKey.newBuilder()
                    .setEd25519(
                            ByteString.copyFrom(Base64.getDecoder().decode(node.get("pub_key").get("value").asText())))
                    .build();

            Validator validator = Validator.newBuilder()
                    .setAddress(jsonToBytes(node.get("address")))
                    .setPubKey(publicKey)
                    .setVotingPower(node.get("voting_power").asLong())
                    .setProposerPriority(node.get("proposer_priority").asLong()).build();

            validators.add(validator);
        });
        validatorSet.addAllValidators(validators);

        return validatorSet.build();
    }

    private BlockID parseBlockId(JsonNode json) {
        PartSetHeader partSetHeader = PartSetHeader.newBuilder()
                .setHash(jsonToBytes(json.get("parts").get("hash")))
                .setTotal(json.get("parts").get("total").asInt()).build();
        BlockID blockID = BlockID.newBuilder()
                .setHash(jsonToBytes(json.get("hash")))
                .setPartSetHeader(partSetHeader).build();
        return blockID;
    }

    private List<CommitSig> parseCommitSig(JsonNode json) {
        List<CommitSig> commitSigs = new ArrayList<CommitSig>();

        json.elements().forEachRemaining((node) -> {
            CommitSig commitSig = CommitSig.newBuilder()
                    .setBlockIdFlagValue(node.get("block_id_flag").asInt())
                    .setValidatorAddress(jsonToBytes(node.get("validator_address")))
                    .setTimestamp(jsonToTimestamp(node.get("timestamp")))
                    .setSignature(ByteString.copyFrom(Base64.getDecoder().decode(node.get("signature").asText())))
                    .build();

            commitSigs.add(commitSig);
        });

        return commitSigs;
    }

    private ByteString jsonToBytes(JsonNode val) {
        return ByteString.copyFrom(hexStringToByteArray(val.asText()));
    }

    private Timestamp jsonToTimestamp(JsonNode val) {
        Instant time = Instant.from(INSTANT_FORMAT.parse(val.asText()));
        Timestamp timestamp = Timestamp.newBuilder()
                .setSeconds(time.getEpochSecond())
                .setNanos(time.getNano()).build();

        return timestamp;
    }

    public static byte[] hexStringToByteArray(String s) {
        int len = s.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(s.charAt(i), 16) << 4)
                    + Character.digit(s.charAt(i + 1), 16));
        }
        return data;
    }

    public static void printBytes(byte[] bytes) {
        for (int j = 0; j < bytes.length; j++) {
            System.out.format("%02X ", bytes[j]);
        }
        System.out.println();
    }

    public static void printBytesToDec(byte[] bytes) {
        for (int j = 0; j < bytes.length; j++) {
            System.out.format("%d, ", bytes[j]);
        }
        System.out.println();
    }
}