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
import static ibc.ics23.commitment.Proof.*;

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
        String iavl = "iavl";
        String tendermint = "tendermint";
        String smt = "smt";

        List<TestVectorsData> cases = new ArrayList<>();
        cases.add(new TestVectorsData(iavl, "exist_left.json", getIavlSpec()));
        cases.add(new TestVectorsData(iavl, "exist_right.json", getIavlSpec()));
        cases.add(new TestVectorsData(iavl, "exist_middle.json", getIavlSpec()));
        cases.add(new TestVectorsData(iavl, "nonexist_left.json", getIavlSpec()));
        cases.add(new TestVectorsData(iavl, "nonexist_right.json", getIavlSpec()));
        cases.add(new TestVectorsData(iavl, "nonexist_middle.json", getIavlSpec()));
        cases.add(new TestVectorsData(tendermint, "exist_left.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "exist_right.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "exist_middle.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_left.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_right.json", getTendermintSpec()));
        cases.add(new TestVectorsData(tendermint, "nonexist_middle.json", getTendermintSpec()));
        cases.add(new TestVectorsData(smt, "exist_left.json", getSmtSpec()));
        cases.add(new TestVectorsData(smt, "exist_right.json", getSmtSpec()));
        cases.add(new TestVectorsData(smt, "exist_middle.json", getSmtSpec()));
        cases.add(new TestVectorsData(smt, "nonexist_left.json", getSmtSpec()));
        cases.add(new TestVectorsData(smt, "nonexist_right.json", getSmtSpec()));
        cases.add(new TestVectorsData(smt, "nonexist_middle.json", getSmtSpec()));


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
        var smt = "smt";

        List<String> filenames = List.of("exist_left.json",
                "exist_right.json",
                "exist_middle.json",
                "nonexist_left.json",
                "nonexist_right.json",
                "nonexist_middle.json");

        List<RefData> refsIAVL = new ArrayList<>();
        List<RefData> refsTM = new ArrayList<>();
        List<RefData> refsSMT = new ArrayList<>();

        CommitmentProof batchIAVL;
        CommitmentProof refsTML;
        CommitmentProof batchSMT;

        List<CommitmentProof> iavlProofs = new ArrayList<>();
        List<CommitmentProof> tendermintProofs = new ArrayList<>();
        List<CommitmentProof> smtProofs = new ArrayList<>();

        for (String filename : filenames) {
            var tendermintTestVector = getTestVector(tendermint, filename);
            var iavlTestVector = getTestVector(iavl, filename);
            var smtTestVector = getTestVector(smt, filename);

            refsIAVL.add(iavlTestVector.getRefData());
            refsTM.add(tendermintTestVector.getRefData());
            refsSMT.add(smtTestVector.getRefData());

            iavlProofs.add(iavlTestVector.getCommitmentProof());
            tendermintProofs.add(tendermintTestVector.getCommitmentProof());
            smtProofs.add(smtTestVector.getCommitmentProof());
        }
        batchIAVL = combineProofs(iavlProofs);
        refsTML = combineProofs(tendermintProofs);
        batchSMT = combineProofs(smtProofs);

        var batchExistVector = getBatchVector(tendermint, "batch_exist.json");
        var batchTMExist = batchExistVector.getCommitmentProof();
        var refsTMExist = batchExistVector.getRefs();

        var batchNonExistVector = getBatchVector(tendermint, "batch_nonexist.json");
        var batchTMNonexist = batchNonExistVector.getCommitmentProof();
        var refsTMNonexist = batchNonExistVector.getRefs();

        var batchIAVLExistVector = getBatchVector(iavl, "batch_exist.json");
        var batchIAVLExist = batchIAVLExistVector.getCommitmentProof();
        var refsIAVLExist = batchIAVLExistVector.getRefs();

        var batchIAVLNonExistVector = getBatchVector(iavl, "batch_nonexist.json");
        var batchIAVLNonExist = batchIAVLNonExistVector.getCommitmentProof();
        var refsIAVLNonExist = batchIAVLNonExistVector.getRefs();

        var batchSMTExistVector = getBatchVector(smt, "batch_exist.json");
        var batchSMTExist = batchSMTExistVector.getCommitmentProof();
        var refsSMTExist = batchSMTExistVector.getRefs();

        var batchSMTNonExistVector = getBatchVector(smt, "batch_nonexist.json");
        var batchSMTNonExist = batchSMTNonExistVector.getCommitmentProof();
        var refsSMTNonExist = batchSMTNonExistVector.getRefs();

        Map<String, BatchVectorData> result = new HashMap<>();
        result.put("iavl 0", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(0)));
        result.put("iavl 1", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(1)));
        result.put("iavl 2", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(2)));
        result.put("iavl 3", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(3)));
        result.put("iavl 4", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(4)));
        result.put("iavl 5", new BatchVectorData(getIavlSpec(), batchIAVL, refsIAVL.get(5)));
        // Note this spec only differs for non-existence proofs
        result.put("iavl invalid 1", new BatchVectorData(getTendermintSpec(), batchIAVL, refsIAVL.get(4), true));
        result.put("iavl invalid 2", new BatchVectorData(getIavlSpec(), batchIAVL, refsTM.get(0), true));
        result.put("iavl batch exist", new BatchVectorData(getIavlSpec(), batchIAVLExist, refsIAVLExist.get(17)));
        result.put("iavl batch nonexist", new BatchVectorData(getIavlSpec(), batchIAVLNonExist, refsIAVLNonExist.get(7)));

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

        result.put("smt 0", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(0)));
        result.put("smt 1", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(1)));
        result.put("smt 2", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(2)));
        result.put("smt 3", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(3)));
        result.put("smt 4", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(4)));
        result.put("smt 5", new BatchVectorData(getSmtSpec(), batchSMT, refsSMT.get(5)));
        // Note this spec only differs for non-existence proofs
        result.put("smt invalid 1", new BatchVectorData(getIavlSpec(), batchSMT, refsSMT.get(4), true));
        result.put("smt invalid 2", new BatchVectorData(getSmtSpec(), batchSMT, refsIAVL.get(0), true));
        result.put("smt batch exist", new BatchVectorData(getSmtSpec(), batchSMTExist, refsSMTExist.get(10)));
        result.put("smt batch nonexist", new BatchVectorData(getSmtSpec(), batchSMTNonExist, refsSMTNonExist.get(3)));

        return result;
    }

    public static Map<String, CommitmentProof> loadDecompressBatchVectorsData() throws IOException {
        var iavl = "iavl";
        var tendermint = "tendermint";
        var smt = "smt";

        var batchNonExistVector = getBatchVector(iavl, "batch_nonexist.json");
        var batchIAVL = batchNonExistVector.getCommitmentProof();

        batchNonExistVector = getBatchVector(tendermint, "batch_nonexist.json");
        var batchTM = batchNonExistVector.getCommitmentProof();

        batchNonExistVector = getBatchVector(smt, "batch_nonexist.json");
        var batchSMT = batchNonExistVector.getCommitmentProof();

        return Map.of(iavl, batchIAVL, tendermint, batchTM, smt, batchSMT);
    }

}
