package ibc.ics23.commitment;

import icon.proto.core.commitment.ProofSpec;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

import static ibc.icon.score.util.StringUtil.bytesToHex;
import static ibc.ics23.commitment.Ops.checkAgainstSpec;
import static ibc.ics23.commitment.Proof.*;
import static org.junit.jupiter.api.Assertions.fail;

public class ProofTest {

    @Test
    public void testExistenceProof() throws IOException {
        Map<String, LoadProofTestData.ExistenceProofTestData> cases = LoadProofTestData.loadExistenceProofTestData();

        for (String name : cases.keySet()) {
            var testCase = cases.get(name);
            byte[] res = null;
            Throwable err = null;
            try {
                res = calculateRoot(testCase.proof);
            } catch (Throwable t) {
                err = t;
            }

            // short-circuit with error case
            if (testCase.isErr && err == null) {
                fail("Expected error, but got none");
            }
            if (!testCase.isErr && err != null) {
                fail(err.getMessage());
            }
            if (!Arrays.equals(res, testCase.expected)) {
                fail("Bad result: " + name + ": " + bytesToHex(res) + " vs " + bytesToHex(testCase.expected));
            }
            System.out.println("ExistenceProof Test Passed: " + name);
        }
    }

    @Test
    public void testCheckLeaf() throws IOException {
        Map<String, LoadProofTestData.CheckLeafTestData> cases = LoadProofTestData.loadLeafTestData();
        for (String name : cases.keySet()) {
            var testCase = cases.get(name);
            Throwable err = null;
            try {
                var proofSpec = new ProofSpec();
                proofSpec.setLeafSpec(testCase.spec);
                checkAgainstSpec(testCase.leaf, proofSpec);
            } catch (Throwable t) {
                err = t;
            }

            if (testCase.isErr && err == null) {
                fail("Expected error, but got null");
            } else if (!testCase.isErr && err != null) {
                fail("Unexpected error: " + err.getMessage());
            }
            System.out.println("Check Leaf Test Passed: " + name);
        }
    }

    @Test
    public void testCheckAgainstSpec() throws IOException {
        Map<String, LoadProofTestData.CheckAgainstSpecTestData> cases = LoadProofTestData.loadCheckAgainstSpecTestData();
        for (String name : cases.keySet()) {
            var testCase = cases.get(name);
            Throwable err = null;
            try {
                Proof.checkAgainstSpec(testCase.proof, testCase.spec);
            } catch (Throwable t) {
                err = t;
            }

            if (testCase.isErr && err == null) {
                fail("Expected error, but got null");
            } else if (!testCase.isErr && err != null) {
                fail("Unexpected error: " + err.getMessage());
            }
            System.out.println("Check Against Spec Test Passed: " + name);
        }
    }

    @Test
    public void testEmptyBranch() {
        List<LoadProofTestData.EmptyBranchTestData> cases = LoadProofTestData.loadEmptyBranchTestData();
        for (var testCase : cases) {
            Throwable err = null;
            try {
                checkAgainstSpec(testCase.op, testCase.spec);
            } catch (Throwable t) {
                err = t;
            }
            if (err != null) {
                fail("Invalid InnerOp " + err.getMessage());
            }

            if (leftBranchesAreEmpty(testCase.spec.getInnerSpec(), testCase.op) != testCase.isLeft) {
                fail("Expected leftBranchesAreEmpty to be " + testCase.isLeft + " but it wasn't");
            }

            if (rightBranchesAreEmpty(testCase.spec.getInnerSpec(), testCase.op) != testCase.isRight) {
                fail("Expected rightBranchesAreEmpty to be :" + testCase.isRight + " but it wasn't");
            }
        }
    }
}
