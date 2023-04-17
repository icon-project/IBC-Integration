package ibc.ics23.commitment;

import icon.proto.core.commitment.CommitmentProof;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

import static ibc.icon.score.util.StringUtil.bytesToHex;
import static ibc.ics23.commitment.Compress.compress;
import static ibc.ics23.commitment.Compress.decompress;
import static ibc.ics23.commitment.Ics23.verifyMembership;
import static ibc.ics23.commitment.Ics23.verifyNonMembership;
import static ibc.ics23.commitment.Proof.calculateRoot;
import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.fail;

public class Ics23Test {

    @Test
    public void testCalculateRootAndMembershipVerification() throws IOException {
        List<LoadVectorTestData.TestVectorsData> cases = LoadVectorTestData.getVectorsTestData();
        for (LoadVectorTestData.TestVectorsData tc : cases) {
            String name = tc.dir + "/" + tc.filename;
            System.out.println("Running test: " + name);
            LoadVectorTestData.TestVector vector = LoadVectorTestData.getTestVector(tc.dir, tc.filename);
            var proof = vector.getCommitmentProof();
            var ref = vector.getRefData();

            // Test Calculate method
            byte[] calculatedRoot = new byte[0];
            try {
                calculatedRoot = calculateRoot(proof);
            } catch (Exception e) {
                fail("calculateRoot() returned error: " + e.getMessage(), e);
            }

            if (!Arrays.equals(ref.rootHash, calculatedRoot)) {
                fail("Calculated root: " + bytesToHex(calculatedRoot) + " did not match expected root: " + bytesToHex(ref.rootHash));
            }

            // Test Verify method
            if (ref.value == null || Arrays.equals(ref.value, new byte[0])) {
                // non-existence
                assertDoesNotThrow(() -> verifyNonMembership(tc.spec, ref.rootHash, proof, ref.key), "Invalid proof");
            } else {
                assertDoesNotThrow(() -> verifyMembership(tc.spec, ref.rootHash, proof, ref.key, ref.value), "Invalid proof");
            }
        }
    }

    @Test
    public void testBatchVectors() throws IOException {
        Map<String, LoadVectorTestData.BatchVectorData> cases = LoadVectorTestData.loadBatchVectorsTestData();
        for (String name : cases.keySet()) {
            var tc = cases.get(name);

            Throwable err = null;
            if (tc.ref.value == null || Arrays.equals(tc.ref.value, new byte[0])) {
                try {
                    verifyNonMembership(tc.spec, tc.ref.rootHash, tc.proof, tc.ref.key);
                } catch (Throwable e) {
                    err = e;
                }
                boolean valid = err == null;
                if (valid == tc.invalid) {
                    fail("Expected proof validity: " + !tc.invalid);
                }
                System.out.println("VerifyNonMembership for batch proofs passed: " + name);
            } else {
                try {
                    verifyMembership(tc.spec, tc.ref.rootHash, tc.proof, tc.ref.key, tc.ref.value);
                } catch (Throwable e) {
                    err = e;
                }
                boolean valid = err == null;
                if (valid == tc.invalid) {
                    fail("Expected proof validity: " + !tc.invalid);
                }
                System.out.println("VerifyMembership for batch proofs passed: " + name);
            }
        }
    }

    @Test
    public void testDecompressBatchVectors() throws IOException {
        Map<String, CommitmentProof> cases = LoadVectorTestData.loadDecompressBatchVectorsData();
        for (String name : cases.keySet()) {
            var tc = cases.get(name);
            var small = tc.encode();

            var decompressed = decompress(tc);
            var big = decompressed.encode();

            if (Arrays.equals(big, small)) {
                fail("Decompression is a no operation");
            }

            if (small.length >= big.length) {
                fail("Compression doesn't reduce size");
            }

            var restore = compress(tc);
            var reSmall = restore.encode();

            if (reSmall.length != small.length) {
                fail("Decompressed len " + reSmall.length + ", original len " + small.length);
            }

            if (!Arrays.equals(reSmall, small)) {
                fail("Decompressed batch proof differs from original");
            }
        }
    }
}
