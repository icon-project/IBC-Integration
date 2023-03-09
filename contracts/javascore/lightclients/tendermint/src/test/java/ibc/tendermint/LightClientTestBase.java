package ibc.tendermint;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import foundation.icon.ee.util.Crypto;
import ibc.icon.structs.proto.lightclient.tendermint.*;
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

    public static Fraction trustLevel = new Fraction(BigInteger.TWO, BigInteger.valueOf(3));
    private static Duration trustingPeriod = new Duration(day.multiply(BigInteger.valueOf(1000)), BigInteger.ZERO);
    private static Duration unbondingPeriod = null;
    private static Duration maxClockDrift = new Duration(BigInteger.valueOf(10), BigInteger.ZERO);
    private static BigInteger frozenHeight = null;
    private static boolean allowUpdateAfterExpiry = false;
    private static boolean allowUpdateAfterMisbehaviour = false;
    protected final MockedStatic<Context> contextMock = Mockito.mockStatic(Context.class, Mockito.CALLS_REAL_METHODS);
    protected String blockSetPath = "src/test/java/ibc/tendermint/data/simple/";

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
        tmHeader.signedHeader = parseSignedHeader(blockOrder);
        tmHeader.validatorSet = parseValidatorSet(blockOrder);

        ClientState clientState = new ClientState();
        clientState.chainId = tmHeader.signedHeader.header.chainId;
        clientState.trustLevel = trustLevel;
        clientState.trustingPeriod = trustingPeriod;
        clientState.unbondingPeriod = unbondingPeriod;
        clientState.maxClockDrift = maxClockDrift;
        clientState.frozenHeight = frozenHeight;
        clientState.latestHeight = tmHeader.signedHeader.header.height;
        clientState.allowUpdateAfterExpiry = allowUpdateAfterExpiry;
        clientState.allowUpdateAfterMisbehaviour = allowUpdateAfterMisbehaviour;

        ConsensusState consensusState = new ConsensusState();
        consensusState.timestamp = tmHeader.signedHeader.header.time;
        consensusState.root = new MerkleRoot(tmHeader.signedHeader.header.appHash);
        consensusState.nextValidatorsHash = tmHeader.signedHeader.header.nextValidatorsHash;

        client.invoke(owner, "createClient", clientId, clientState.toBytes(), consensusState.toBytes());
    }

    private void updateClient(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = createHeader(blockOrder, referenceBlock);
        client.invoke(owner, "updateClient", clientId, tmHeader.toBytes());
    }

    private TmHeader createHeader(int blockOrder, int referenceBlock) throws Exception {
        TmHeader tmHeader = new TmHeader();
        tmHeader.signedHeader = parseSignedHeader(blockOrder);
        tmHeader.validatorSet = parseValidatorSet(blockOrder);
        tmHeader.trustedHeight = parseSignedHeader(referenceBlock).header.height;
        tmHeader.trustedValidators = parseValidatorSet(referenceBlock);
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
        lightHeader.version = new Consensus();
        lightHeader.version.block = BigInteger.valueOf(jsonHeader.get("version").get("block").asInt());

        lightHeader.chainId = jsonHeader.get("chain_id").asText();

        lightHeader.height = BigInteger.valueOf(jsonHeader.get("height").asInt());
        lightHeader.time = jsonToTimestamp(jsonHeader.get("time"));
        lightHeader.lastBlockId = parseBlockId(jsonHeader.get("last_block_id"));
        lightHeader.lastCommitHash = jsonToBytes(jsonHeader.get("last_commit_hash"));
        lightHeader.dataHash = jsonToBytes(jsonHeader.get("data_hash"));
        lightHeader.validatorsHash = jsonToBytes(jsonHeader.get("validators_hash"));
        lightHeader.nextValidatorsHash = jsonToBytes(jsonHeader.get("next_validators_hash"));
        lightHeader.consensusHash = jsonToBytes(jsonHeader.get("consensus_hash"));
        lightHeader.appHash = jsonToBytes(jsonHeader.get("app_hash"));
        lightHeader.lastResultsHash = jsonToBytes(jsonHeader.get("last_results_hash"));
        lightHeader.evidenceHash = jsonToBytes(jsonHeader.get("evidence_hash"));
        lightHeader.proposerAddress = jsonToBytes(jsonHeader.get("proposer_address"));

        Commit commit = new Commit();
        JsonNode jsonCommit = json.get("signed_header").get("commit");
        commit.height = BigInteger.valueOf(jsonCommit.get("height").asInt());
        BigInteger round = BigInteger.valueOf(jsonCommit.get("round").asInt());
        if (!round.equals(BigInteger.ZERO)) {
            commit.round = round;
        }

        commit.blockId = parseBlockId(jsonCommit.get("block_id"));
        commit.signatures = parseCommitSig(jsonCommit.get("signatures"));

        SignedHeader signedHeader = new SignedHeader();
        signedHeader.header = lightHeader;
        signedHeader.commit = commit;

        return signedHeader;
    }

    private ValidatorSet parseValidatorSet(int blockOrder) throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        String loc = getValidatorPath(blockOrder);
        File file = new File(loc);
        String content = new String(Files.readAllBytes(Paths.get(file.toURI())));
        JsonNode json = mapper.readTree(content);
        ValidatorSet validatorSet = new ValidatorSet();
        validatorSet.proposer = null;
        validatorSet.totalVotingPower = BigInteger.ZERO;
        List<Validator> validators = new ArrayList<>();
        json.get("validators").elements().forEachRemaining((node) -> {
            Validator validator = new Validator();
            validator.address = hexStringToByteArray(node.get("address").asText());
            PublicKey publicKey = new PublicKey();
            // TODO: support more key types
            publicKey.ed25519 = Base64.getDecoder().decode(node.get("pub_key").get("value").asText());
            validator.pubKey = publicKey;
            validator.votingPower = BigInteger.valueOf(node.get("voting_power").asLong());
            validator.proposerPriority = BigInteger.valueOf(node.get("proposer_priority").asLong());

            validators.add(validator);
        });

        validatorSet.validators = validators.toArray(new Validator[validators.size()]);

        return validatorSet;
    }

    private BlockID parseBlockId(JsonNode json) {
        BlockID blockID = new BlockID();
        blockID.hash = jsonToBytes(json.get("hash"));
        blockID.partSetHeader = new PartSetHeader();
        blockID.partSetHeader.hash = jsonToBytes(json.get("parts").get("hash"));
        blockID.partSetHeader.total = BigInteger.valueOf(json.get("parts").get("total").asInt());

        return blockID;
    }

    private CommitSig[] parseCommitSig(JsonNode json) {
        List<CommitSig> commitSigs = new ArrayList<>();

        json.elements().forEachRemaining((node) -> {
            CommitSig commitSig = new CommitSig();
            commitSig.blockIdFlag = node.get("block_id_flag").asInt();
            commitSig.validatorAddress = jsonToBytes(node.get("validator_address"));
            commitSig.timestamp = jsonToTimestamp(node.get("timestamp"));
            commitSig.signature = Base64.getDecoder().decode(node.get("signature").asText());

            commitSigs.add(commitSig);
        });

        return commitSigs.toArray(new CommitSig[commitSigs.size()]);
    }

    private byte[] jsonToBytes(JsonNode val) {
        return hexStringToByteArray(val.asText());
    }

    private Timestamp jsonToTimestamp(JsonNode val) {
        Instant time = Instant.from(INSTANT_FORMAT.parse(val.asText()));
        Timestamp timestamp = new Timestamp(BigInteger.valueOf(time.getEpochSecond()),
                BigInteger.valueOf(time.getNano()));

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

// -91, 16, 62, -51, 28, 102, 85, 57, -117, 99, -56, -36, 5, 23, 29, -126, -127,
// 116, 87, 46, -35, -14, -77, -57, 62, 91, -44, -59, -44, 122, 49, -2,
// -105, -17, 48, 86, 122, -96, -6, 98, 84, -86, -100, -90, -127, 114, -34, -87,
// -4, 104, -90, -45, -38, -127, -43, -86, 71, -46, -2, -103, 12, 33, 71, 46,
// -37, -28, 2, -83, 122, -107, 48, -43, -117, 48, -28, -93, -108, 113, 63, 119,
// 86, 122, 92, -18, -112, 35, -2, 24, 25, -4, 62, -7, 2, -65, 88, 124,
// -29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12, -56, -103, 111, -71, 36,
// 39, -82, 65, -28, 100, -101, -109, 76, -92, -107, -103, 27, 120, 82, -72, 85,
// -23, -58, 86, -124, 17, 25, -45, 48, 86, 5, 109, -2, 111, -52, 31, -125, 59,
// -43, 90, -14, 49, -62, 115, 38, 67, 37, 121, -23, -81, -82, -99, -88,
// -23, -58, 86, -124, 17, 25, -45, 48, 86, 5, 109, -2, 111, -52, 31, -125, 59,
// -43, 90, -14, 49, -62, 115, 38, 67, 37, 121, -23, -81, -82, -99, -88,
// 58, -76, -80, 100, -114, -60, 100, -105, -2, -127, 0, 9, -74, -85, 41, -91,
// -31, -119, -125, -80, -90, 21, -66, 23, 67, 5, -7, 73, 104, 72, 102, 2,
// 39, -70, 60, 6, 120, -65, 127, 92, 21, 103, 64, 94, 69, 17, -2, 68, -76, 19,
// 20, 104, -1, -7, 126, -25, -60, -8, -107, -85, -44, -18, -73, 109,
// -29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12, -56, -103, 111, -71, 36,
// 39, -82, 65, -28, 100, -101, -109, 76, -92, -107, -103, 27, 120, 82, -72, 85,
// -29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12, -56, -103, 111, -71, 36,
// 39, -82, 65, -28, 100, -101, -109, 76, -92, -107, -103, 27, 120, 82, -72, 85,
// 34, 121, 80, -63, 55, 51, -126, -73, -79, 43, 42, 26, 18, -1, -39, -4, 44,
// 74, 96, -3,
// {
// "version": {
// "block":11,
// "app": 0
// },
// "chain_id": "constantine-1",
// "height": 600000,
// "time": {
// "seconds": 1677493076,
// "nanos": 252368746
// },
// "last_block_id": {
// "hash": [-91, 16, 62, -51, 28, 102, 85, 57, -117, 99, -56, -36, 5, 23, 29,
// -126, -127, 116, 87, 46, -35, -14, -77, -57, 62, 91, -44, -59, -44, 122, 49,
// -2],
// "part_set_header": {
// "total": 1,
// "hash": [-105, -17, 48, 86, 122, -96, -6, 98, 84, -86, -100, -90, -127, 114,
// -34, -87, -4, 104, -90, -45, -38, -127, -43, -86, 71, -46, -2, -103, 12, 33,
// 71, 46]
// }
// },
// "last_commit_hash": [-37, -28, 2, -83, 122, -107, 48, -43, -117, 48, -28,
// -93, -108, 113, 63, 119, 86, 122, 92, -18, -112, 35, -2, 24, 25, -4, 62, -7,
// 2, -65, 88, 124],
// "data_hash": [-29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12, -56, -103,
// 111, -71, 36, 39, -82, 65, -28, 100, -101, -109, 76, -92, -107, -103, 27,
// 120, 82, -72, 85],
// "validators_hash": [-23, -58, 86, -124, 17, 25, -45, 48, 86, 5, 109, -2, 111,
// -52, 31, -125, 59, -43, 90, -14, 49, -62, 115, 38, 67, 37, 121, -23, -81,
// -82, -99, -88],
// "next_validators_hash": [-23, -58, 86, -124, 17, 25, -45, 48, 86, 5, 109, -2,
// 111, -52, 31, -125, 59, -43, 90, -14, 49, -62, 115, 38, 67, 37, 121, -23,
// -81, -82, -99, -88],
// "consensus_hash": [58, -76, -80, 100, -114, -60, 100, -105, -2, -127, 0, 9,
// -74, -85, 41, -91, -31, -119, -125, -80, -90, 21, -66, 23, 67, 5, -7, 73,
// 104, 72, 102, 2],
// "app_hash": [39, -70, 60, 6, 120, -65, 127, 92, 21, 103, 64, 94, 69, 17, -2,
// 68, -76, 19, 20, 104, -1, -7, 126, -25, -60, -8, -107, -85, -44, -18, -73,
// 109,],
// "last_results_hash": [-29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12,
// -56, -103, 111, -71, 36, 39, -82, 65, -28, 100, -101, -109, 76, -92, -107,
// -103, 27, 120, 82, -72, 85],
// "evidence_hash": [-29, -80, -60, 66, -104, -4, 28, 20, -102, -5, -12, -56,
// -103, 111, -71, 36, 39, -82, 65, -28, 100, -101, -109, 76, -92, -107, -103,
// 27, 120, 82, -72, 85],
// "proposer_address": [34, 121, 80, -63, 55, 51, -126, -73, -79, 43, 42, 26,
// 18, -1, -39, -4, 44, 74, 96, -3]
// }

// {
// "type": "SIGNED_MSG_TYPE_PRECOMMIT",
// "height": 600000,
// "round": 0,
// "block_id": {
// "hash": [-40, 95, -86, -18, 29, -97, -112, 34, 11, 46, -106, -117, 109, -10,
// -99, -63, 59, 37, -15, -23, -103, 85, 68, 54, -37, 51, -106, -13, -94, 107,
// -26, 38],
// "part_set_header": {
// "total": 1,
// "hash": [-61, 61, 75, -6, 92, 108, 97, 9, -113, 10, -8, 75, -57, 125, 69,
// -36, 50, 77, 119, 106, -83, -110, -85, 9, 93, 13, 104, -35, -63, -13, -117,
// -37]
// }
// },
// "timestamp": {
// "seconds": 1677493081,
// "nanos": 280744845
// },
// "chain_id": "constantine-1"
// }