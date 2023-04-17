package ibc.ics23.commitment;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import icon.proto.core.commitment.CommitmentProof;
import icon.proto.core.commitment.ProofSpec;

import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static ibc.icon.score.util.StringUtil.hexToBytes;
import static ibc.ics23.commitment.Ics23.combineProofs;
import static ibc.ics23.commitment.LoadProofTestData.getIavlSpec;
import static ibc.ics23.commitment.LoadProofTestData.getTendermintSpec;

public class LoadVectorTestData {

    public static class TestVector {
        public String root;
        public String proof;
        public String key;
        public String value;

        public CommitmentProof getCommitmentProof() {
            return CommitmentProof.decode(mustHex(proof));
        }

        public RefData getRefData() {
            RefData ref = new RefData();
            ref.setRootHash(mustHex(root));
            ref.setKey(mustHex(key));
            if (!value.isEmpty()) {
                ref.setValue(mustHex(value));
            }
            return ref;
        }
    }

    // RefData is parsed version of everything except the CommitmentProof itself
    public static class RefData {
        public byte[] rootHash;
        public byte[] key;
        public byte[] value;

        public void setRootHash(byte[] rootHash) {
            this.rootHash = rootHash;
        }

        public void setKey(byte[] key) {
            this.key = key;
        }

        public void setValue(byte[] value) {
            this.value = value;
        }
    }

    public static class TestVectorsData {
        public String dir;
        public String filename;
        public ProofSpec spec;

        public TestVectorsData(String dir, String filename, ProofSpec spec) {
            this.dir = dir;
            this.filename = filename;
            this.spec = spec;
        }
    }

    public static List<TestVectorsData> getVectorsTestData() {
        String tendermint = "tendermint";

        List<TestVectorsData> cases = new ArrayList<>();
        cases.add(new TestVectorsData(tendermint, "exist_left.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "exist_right.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "exist_middle.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_left.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_right.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_middle.json", getTendermintSpec()));

        return cases;
    }

    public static byte[] mustHex(String data) {
        if (data == null || data.isEmpty()) {
            return null;
        }
        try {
            return hexToBytes(data);
        } catch (IllegalArgumentException e) {
            throw new RuntimeException("Decoding hex failed: " + e.getMessage(), e);
        }
    }

    public static TestVector getTestVector(String dir, String filename) throws IOException {
        InputStream stream = LoadVectorTestData.class.getResourceAsStream("/" + dir + "/" + filename);
        return new ObjectMapper().readValue(stream, new TypeReference<>() {
        });
    }

    public static class BatchVector {
        public String root;
        public String proof;
        public List<Item> items;

        public static class Item {
            public String key;
            public String value;
        }

        // parse the protobuf object
        public CommitmentProof getCommitmentProof() {
            return CommitmentProof.decode(mustHex(proof));
        }

        public List<RefData> getRefs() {
            List<RefData> refs = new ArrayList<>();
            for (Item item : items) {
                RefData ref = new RefData();
                ref.setRootHash(mustHex(root));
                ref.setKey(mustHex(item.key));
                if (!item.value.isEmpty()) {
                    ref.setValue(mustHex(item.value));
                }
                refs.add(ref);
            }
            return refs;
        }
    }

    public static BatchVector getBatchVector(String dir, String filename) throws IOException {
        InputStream stream = LoadVectorTestData.class.getResourceAsStream("/" + dir + "/" + filename);
        return new ObjectMapper().readValue(stream, new TypeReference<>() {
        });
    }

    public static class BatchVectorData {
        public ProofSpec spec;
        public CommitmentProof proof;
        public RefData ref;
        public boolean invalid = false;

        public BatchVectorData(ProofSpec spec, CommitmentProof proof, RefData ref) {
            this.spec = spec;
            this.proof = proof;
            this.ref = ref;
        }

        public BatchVectorData(ProofSpec spec, CommitmentProof proof, RefData ref, boolean invalid) {
            this.spec = spec;
            this.proof = proof;
            this.ref = ref;
            this.invalid = invalid;
        }
    }

    public static Map<String, BatchVectorData> loadBatchVectorsTestData() throws IOException {
        var tendermint = "tendermint";
        var iavl = "iavl";

        List<String> filenames = List.of("exist_left.json",
                "exist_right.json",
                "exist_middle.json",
                "nonexist_left.json",
                "nonexist_right.json",
                "nonexist_middle.json");

        List<RefData> refsTM = new ArrayList<>();
        List<CommitmentProof> proofs = new ArrayList<>();
        List<RefData> refsIAVL = new ArrayList<>();

        for (String filename : filenames) {
            var tendermintTestVector = getTestVector(tendermint, filename);
            var iavlTestVector = getTestVector(iavl, filename);

            refsTM.add(tendermintTestVector.getRefData());
            proofs.add(tendermintTestVector.getCommitmentProof());
            refsIAVL.add(iavlTestVector.getRefData());
        }
        CommitmentProof refsTML = combineProofs(proofs);

        var batchExistVector = getBatchVector(tendermint, "batch_exist.json");
        var batchTMExist = batchExistVector.getCommitmentProof();
        var refsTMExist = batchExistVector.getRefs();

        var batchNonExistVector = getBatchVector(tendermint, "batch_nonexist.json");
        var batchTMNonexist = batchNonExistVector.getCommitmentProof();
        var refsTMNonexist = batchNonExistVector.getRefs();

        Map<String, BatchVectorData> result = new HashMap<>();
        result.put("tm 0", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(0)));
        result.put("tm 1", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(1)));
        result.put("tm 2", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(2)));
        result.put("tm 3", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(3)));
        result.put("tm 4", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(4)));
        result.put("tm 5", new BatchVectorData(getTendermintSpec(), refsTML, refsTM.get(5)));
        // Note this spec only differs for non-existence proofs
        result.put("tm invalid 1", new BatchVectorData(getIavlSpec(), refsTML, refsTM.get(4), true));
        result.put("tm invalid 2", new BatchVectorData(getTendermintSpec(), refsTML, refsIAVL.get(0), true));
        result.put("tm batch exist", new BatchVectorData(getTendermintSpec(), batchTMExist, refsTMExist.get(10)));
        result.put("tm batch nonexist", new BatchVectorData(getTendermintSpec(), batchTMNonexist, refsTMNonexist.get(3)));

        return result;
    }

    public static Map<String, CommitmentProof> loadDecompressBatchVectorsData() throws IOException {
        var tendermint = "tendermint";
        var batchNonExistVector = getBatchVector(tendermint, "batch_nonexist.json");
        var batchTM = batchNonExistVector.getCommitmentProof();
        return Map.of(tendermint, batchTM);
    }

}
