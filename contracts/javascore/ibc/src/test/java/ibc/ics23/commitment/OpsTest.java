package ibc.ics23.commitment;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.util.Arrays;
import java.util.Map;

import static ibc.icon.score.util.StringUtil.bytesToHex;
import static ibc.ics23.commitment.Ops.applyOp;
import static ibc.ics23.commitment.Ops.doHash;
import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.fail;

class OpsTest {

    @Test
    public void testLeafOp() throws IOException {
        Map<String, LoadOpsTestData.LeafOpTestData> cases = LoadOpsTestData.loadLeafOpTestData();

        for (Map.Entry<String, LoadOpsTestData.LeafOpTestData> entry : cases.entrySet()) {
            String name = entry.getKey();
            LoadOpsTestData.LeafOpTestData tc = entry.getValue();

            assertDoesNotThrow(() -> {
                byte[] res = applyOp(tc.op, tc.key, tc.value);
                boolean isErr = tc.isErr;
                byte[] expected = tc.expected;

                // short-circuit with error case
                if (isErr && res == null) {
                    fail("Expected error, but got none");
                }

                if (!isErr && res == null) {
                    fail("Expected result, but got none");
                }

                if (!isErr && !Arrays.equals(res, expected)) {
                    fail("Bad result: " + name + ":" + bytesToHex(res) + " vs " + bytesToHex(expected));
                }
                System.out.println("LeafOp Test Passed: " + name);
            });
        }
    }

    @Test
    public void testInnerOp() throws IOException {
        Map<String, LoadOpsTestData.InnerOpTestData> cases = LoadOpsTestData.loadInnerOpTestData();
        for (String name : cases.keySet()) {
            LoadOpsTestData.InnerOpTestData tc = cases.get(name);
            try {
                byte[] res = applyOp(tc.op, tc.child);
                // short-circuit with error case
                if (tc.isErr && res == null) {
                    fail("Expected error, but got none");
                }
                if (!tc.isErr && res == null) {
                    fail("Expected result, but got none");
                }
                if (!Arrays.equals(res, tc.expected)) {
                    fail("Bad result: " + name + ": " + bytesToHex(res) + " vs " + bytesToHex(tc.expected));
                }
            } catch (Exception e) {
                if (!tc.isErr) {
                    fail("Unexpected exception: " + e.getMessage());
                }
            }
            System.out.println("InnerOp Test Passed: " + name);
        }
    }

    @Test
    public void testDoHash() throws IOException {
        Map<String, LoadOpsTestData.DoHashTestData> cases = LoadOpsTestData.loadDoHashTestData();
        for (String name : cases.keySet()) {
            LoadOpsTestData.DoHashTestData tc = cases.get(name);
            try {
                byte[] res = doHash(tc.hashOp, tc.preimage.getBytes());
                String hexRes = bytesToHex(res);
                if (!hexRes.equals(tc.expectedHash)) {
                    fail("Expected " + tc.expectedHash + " got " + hexRes);
                }
            } catch (Exception e) {
                fail("Unexpected exception: " + e.getMessage());
            }
            System.out.println("DoHash Test Passed: " + name);
        }
    }
}