package ibc.tendermint;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import foundation.icon.ee.util.Crypto;
import icon.proto.clients.tendermint.*;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import org.mockito.stubbing.Answer;
import score.Context;

import java.io.File;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;

import static org.mockito.Mockito.any;
import static org.mockito.Mockito.spy;

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
        trustLevel = new Fraction();
        trustLevel.setNumerator(BigInteger.TWO);
        trustLevel.setDenominator(BigInteger.valueOf(3));

        trustingPeriod = new Duration();
        trustingPeriod.setSeconds(day.multiply(BigInteger.valueOf(10000)));
        trustingPeriod.setNanos(BigInteger.ZERO);

        maxClockDrift = new Duration();
        maxClockDrift.setSeconds(BigInteger.TEN);
        maxClockDrift.setNanos(BigInteger.ZERO);

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
        TmHeader tmHeader = new TmHeader();
        tmHeader.setSignedHeader(parseSignedHeader(blockOrder));
        tmHeader.setValidatorSet(parseValidatorSet(blockOrder));

        ClientState clientState = new ClientState();
        clientState.setChainId(tmHeader.getSignedHeader().getHeader().getChainId());
        clientState.setTrustLevel(trustLevel);
        clientState.setTrustingPeriod(trustingPeriod);
        clientState.setMaxClockDrift(maxClockDrift);
        clientState.setLatestHeight(tmHeader.getSignedHeader().getHeader().getHeight());
        clientState.setAllowUpdateAfterExpiry(allowUpdateAfterExpiry);
        clientState.setAllowUpdateAfterMisbehaviour(allowUpdateAfterMisbehaviour);

        ConsensusState consensusState = new ConsensusState();
        consensusState.setTimestamp(tmHeader.getSignedHeader().getHeader().getTime());
        MerkleRoot root = new MerkleRoot();
        root.setHash(tmHeader.getSignedHeader().getHeader().getAppHash());
        consensusState.setRoot(root);
        consensusState.setNextValidatorsHash(tmHeader.getSignedHeader().getHeader().getNextValidatorsHash());

        client.invoke(owner, "createClient", clientId, clientState.encode(), consensusState.encode());
    }

    private void updateClient(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = createHeader(blockOrder, referenceBlock);
        printBytes(Crypto.hash("sha-256", tmHeader.encode()));
        printBytes(Crypto.hash("sha-256", TmHeader.decode(tmHeader.encode()).encode()));
        client.invoke(owner, "updateClient", clientId, tmHeader.encode());
    }

    private TmHeader createHeader(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = new TmHeader();
        tmHeader.setSignedHeader(parseSignedHeader(blockOrder));
        tmHeader.setValidatorSet(parseValidatorSet(blockOrder));
        tmHeader.setTrustedHeight(parseSignedHeader(referenceBlock).getHeader().getHeight());
        tmHeader.setTrustedValidators(parseValidatorSet(referenceBlock));
        return tmHeader;

    }

    private SignedHeader parseSignedHeader(int blockOrder) throws Exception {

        ObjectMapper mapper = new ObjectMapper();
        String loc = getCommitPath(blockOrder);
        File file = new File(loc);
        String content = new String(Files.readAllBytes(Paths.get(file.toURI())));
        JsonNode json = mapper.readTree(content);

        LightHeader lightHeader = new LightHeader();
        JsonNode jsonHeader = json.get("signed_header").get("header");
        Consensus version = new Consensus();
        version.setBlock(BigInteger.valueOf(jsonHeader.get("version").get("block").asInt()));
        lightHeader.setVersion(version);
        lightHeader.setChainId(jsonHeader.get("chain_id").asText());

        lightHeader.setHeight(BigInteger.valueOf(jsonHeader.get("height").asInt()));
        lightHeader.setTime(jsonToTimestamp(jsonHeader.get("time")));
        lightHeader.setLastBlockId(parseBlockId(jsonHeader.get("last_block_id")));
        lightHeader.setLastCommitHash(jsonToBytes(jsonHeader.get("last_commit_hash")));
        lightHeader.setDataHash(jsonToBytes(jsonHeader.get("data_hash")));
        lightHeader.setValidatorsHash(jsonToBytes(jsonHeader.get("validators_hash")));
        lightHeader.setNextValidatorsHash(jsonToBytes(jsonHeader.get("next_validators_hash")));
        lightHeader.setConsensusHash(jsonToBytes(jsonHeader.get("consensus_hash")));
        lightHeader.setAppHash(jsonToBytes(jsonHeader.get("app_hash")));
        lightHeader.setLastResultsHash(jsonToBytes(jsonHeader.get("last_results_hash")));
        lightHeader.setEvidenceHash(jsonToBytes(jsonHeader.get("evidence_hash")));
        lightHeader.setProposerAddress(jsonToBytes(jsonHeader.get("proposer_address")));

        Commit commit = new Commit();
        JsonNode jsonCommit = json.get("signed_header").get("commit");
        commit.setHeight(BigInteger.valueOf(jsonCommit.get("height").asInt()));
        BigInteger round = BigInteger.valueOf(jsonCommit.get("round").asInt());
        if (!round.equals(BigInteger.ZERO)) {
            commit.setRound(round);
        }

        commit.setBlockId(parseBlockId(jsonCommit.get("block_id")));
        commit.setSignatures(parseCommitSig(jsonCommit.get("signatures")));

        SignedHeader signedHeader = new SignedHeader();
        signedHeader.setHeader(lightHeader);
        signedHeader.setCommit(commit);

        return signedHeader;
    }

    private ValidatorSet parseValidatorSet(int blockOrder) throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        String loc = getValidatorPath(blockOrder);
        File file = new File(loc);
        String content = new String(Files.readAllBytes(Paths.get(file.toURI())));
        JsonNode json = mapper.readTree(content);
        ValidatorSet validatorSet = new ValidatorSet();
        validatorSet.setTotalVotingPower(BigInteger.ZERO);
        List<Validator> validators = new ArrayList<>();
        json.get("validators").elements().forEachRemaining((node) -> {
            Validator validator = new Validator();
            validator.setAddress(hexStringToByteArray(node.get("address").asText()));
            PublicKey publicKey = new PublicKey();
            // TODO: support more key types
            publicKey.setEd25519(Base64.getDecoder().decode(node.get("pub_key").get("value").asText()));
            validator.setPubKey(publicKey);
            validator.setVotingPower(BigInteger.valueOf(node.get("voting_power").asLong()));
            validator.setProposerPriority(BigInteger.valueOf(node.get("proposer_priority").asLong()));

            validators.add(validator);
        });

        validatorSet.setValidators(validators);
        return validatorSet;
    }

    private BlockID parseBlockId(JsonNode json) {
        BlockID blockID = new BlockID();
        blockID.setHash(jsonToBytes(json.get("hash")));
        PartSetHeader partSetHeader = new PartSetHeader();
        partSetHeader.setHash(jsonToBytes(json.get("parts").get("hash")));
        partSetHeader.setTotal(BigInteger.valueOf(json.get("parts").get("total").asInt()));
        blockID.setPartSetHeader(partSetHeader);
        return blockID;
    }

    private List<CommitSig> parseCommitSig(JsonNode json) {
        List<CommitSig> commitSigs = new ArrayList<CommitSig>();

        json.elements().forEachRemaining((node) -> {
            CommitSig commitSig = new CommitSig();
            commitSig.setBlockIdFlag(node.get("block_id_flag").asInt());
            commitSig.setValidatorAddress(jsonToBytes(node.get("validator_address")));
            commitSig.setTimestamp(jsonToTimestamp(node.get("timestamp")));
            commitSig.setSignature(Base64.getDecoder().decode(node.get("signature").asText()));

            commitSigs.add(commitSig);
        });

        return commitSigs;
    }

    private byte[] jsonToBytes(JsonNode val) {
        return hexStringToByteArray(val.asText());
    }

    private Timestamp jsonToTimestamp(JsonNode val) {
        Instant time = Instant.from(INSTANT_FORMAT.parse(val.asText()));
        Timestamp timestamp = new Timestamp();
        timestamp.setSeconds(BigInteger.valueOf(time.getEpochSecond()));
        timestamp.setNanos(BigInteger.valueOf(time.getNano()));

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