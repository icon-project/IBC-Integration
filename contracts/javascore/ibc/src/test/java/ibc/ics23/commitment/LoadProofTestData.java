package ibc.ics23.commitment;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import ibc.icon.score.util.ByteUtil;
import icon.proto.core.commitment.*;

import java.io.IOException;
import java.io.InputStream;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

import static ibc.ics23.commitment.Proof.getTendermintSpec;

public class LoadProofTestData {

    public static class ExistenceProofTestData {
        public ExistenceProof proof;
        public boolean isErr;
        public byte[] expected;
    }

    public static Map<String, ExistenceProofTestData> loadExistenceProofTestData() throws IOException {
        InputStream inputStream = LoadProofTestData.class.getResourceAsStream("/TestExistenceProofData.json");
        ObjectMapper objectMapper = new ObjectMapper();
        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class CheckLeafTestData {
        public LeafOp leaf;
        public LeafOp spec;
        public boolean isErr;
    }

    public static Map<String, CheckLeafTestData> loadLeafTestData() throws IOException {
        InputStream inputStream = LoadProofTestData.class.getResourceAsStream("/TestCheckLeafData.json");
        ObjectMapper objectMapper = new ObjectMapper();
        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class CheckAgainstSpecTestData {
        public ExistenceProof proof;
        public ProofSpec spec;
        public boolean isErr;
    }

    public static Map<String, CheckAgainstSpecTestData> loadCheckAgainstSpecTestData() throws IOException {
        InputStream inputStream = LoadProofTestData.class.getResourceAsStream("/TestCheckAgainstSpecData.json");
        ObjectMapper objectMapper = new ObjectMapper();
        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static ProofSpec getSpecWithEmptyChild() {

        byte[] emptyChild = "32_empty_child_placeholder_bytes".getBytes();
        List<BigInteger> childOrder = List.of(BigInteger.ZERO, BigInteger.ONE);

        LeafOp leafSpec = new LeafOp();
        leafSpec.setPrefix(new byte[]{0});
        leafSpec.setHash(HashOp.SHA256);
        leafSpec.setPrehashValue(HashOp.SHA256);

        InnerSpec innerSpec = new InnerSpec();
        innerSpec.setChildOrder(childOrder);
        innerSpec.setChildSize(BigInteger.valueOf(32));
        innerSpec.setMinPrefixLength(BigInteger.ONE);
        innerSpec.setMaxPrefixLength(BigInteger.ONE);
        innerSpec.setEmptyChild(emptyChild);
        innerSpec.setHash(HashOp.SHA256);

        ProofSpec specWithEmptyChild = new ProofSpec();
        specWithEmptyChild.setLeafSpec(leafSpec);
        specWithEmptyChild.setInnerSpec(innerSpec);

        return specWithEmptyChild;
    }

    public static ProofSpec getIavlSpec() {
        LeafOp leafSpec = new LeafOp();
        leafSpec.setPrefix(new byte[]{0});
        leafSpec.setPrehashKey(HashOp.NO_HASH);
        leafSpec.setHash(HashOp.SHA256);
        leafSpec.setPrehashValue(HashOp.SHA256);
        leafSpec.setLength(LengthOp.VAR_PROTO);

        InnerSpec innerSpec = new InnerSpec();
        innerSpec.setChildOrder(List.of(BigInteger.ZERO, BigInteger.ONE));
        innerSpec.setMinPrefixLength(BigInteger.valueOf(4));
        innerSpec.setMaxPrefixLength(BigInteger.valueOf(12));
        innerSpec.setChildSize(BigInteger.valueOf(33));
        innerSpec.setEmptyChild(new byte[0]);
        innerSpec.setHash(HashOp.SHA256);

        var iavlSpec = new ProofSpec();
        iavlSpec.setLeafSpec(leafSpec);
        iavlSpec.setInnerSpec(innerSpec);
        return iavlSpec;
    }

    public static class EmptyBranchTestData {
        public InnerOp op;
        public ProofSpec spec;
        public boolean isLeft;
        public boolean isRight;

        public EmptyBranchTestData(InnerOp op, ProofSpec spec, boolean isLeft, boolean isRight) {
            this.op = op;
            this.spec = spec;
            this.isLeft = isLeft;
            this.isRight = isRight;
        }
    }

    public static List<EmptyBranchTestData> loadEmptyBranchTestData() {
        var emptyChild = getSpecWithEmptyChild().getInnerSpec().getEmptyChild();
        int hash = getSpecWithEmptyChild().getInnerSpec().getHash();

        List<EmptyBranchTestData> sampleData = new ArrayList<>();

        // 1st sample
        var innerOp = new InnerOp();
        innerOp.setPrefix(ByteUtil.join(new byte[]{1}, emptyChild));
        innerOp.setSuffix(new byte[0]);
        innerOp.setHash(hash);

        var emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), true, false);
        sampleData.add(emptyBranchTestData);

        // 2nd sample
        innerOp = new InnerOp();
        innerOp.setPrefix(new byte[]{1});
        innerOp.setSuffix(emptyChild);
        innerOp.setHash(hash);

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), false, true);
        sampleData.add(emptyBranchTestData);

        // Non empty cases
        // 3rd sample
        innerOp = new InnerOp();
        innerOp.setPrefix(ByteUtil.join(new byte[]{1}, new byte[32]));
        innerOp.setSuffix(new byte[0]);
        innerOp.setHash(hash);

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), false, false);
        sampleData.add(emptyBranchTestData);

        // 4th sample
        innerOp = new InnerOp();
        innerOp.setPrefix(new byte[]{1});
        innerOp.setSuffix(new byte[32]);
        innerOp.setHash(hash);

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), false, false);
        sampleData.add(emptyBranchTestData);

        // 5th sample
        innerOp = new InnerOp();
        innerOp.setPrefix(ByteUtil.join(
                ByteUtil.join(new byte[]{1}, Arrays.copyOfRange(emptyChild, 0, 28)),
                "xxxx".getBytes()));
        innerOp.setSuffix(new byte[0]);
        innerOp.setHash(hash);

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), false, false);
        sampleData.add(emptyBranchTestData);

        // 6th sample
        innerOp = new InnerOp();
        innerOp.setPrefix(new byte[]{1});
        innerOp.setSuffix(ByteUtil.join(
                ByteUtil.join(new byte[]{}, Arrays.copyOfRange(emptyChild, 0, 28)),
                "xxxx".getBytes()));
        innerOp.setHash(hash);

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getSpecWithEmptyChild(), false, false);
        sampleData.add(emptyBranchTestData);

        // some cases using a spec with no empty child
        // 7th sample
        innerOp = new InnerOp();
        innerOp.setPrefix(ByteUtil.join(new byte[]{1}, new byte[32]));
        innerOp.setSuffix(new byte[0]);
        innerOp.setHash(getTendermintSpec().getInnerSpec().getHash());

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getTendermintSpec(), false, false);
        sampleData.add(emptyBranchTestData);

        // 8th sample
        innerOp = new InnerOp();
        innerOp.setPrefix(new byte[]{1});
        innerOp.setSuffix(new byte[32]);
        innerOp.setHash(getTendermintSpec().getInnerSpec().getHash());

        emptyBranchTestData = new EmptyBranchTestData(innerOp, getTendermintSpec(), false, false);
        sampleData.add(emptyBranchTestData);

        return sampleData;
    }
}
