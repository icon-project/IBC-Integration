package ibc.ics23.commitment.types;

import ibc.icon.score.util.StringUtil;
import icon.proto.core.commitment.MerkleProof;
import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;

import static ibc.ics23.commitment.types.Merkle.isMerkleProofEmpty;
import static org.junit.jupiter.api.Assertions.assertFalse;

public class MerkleTest {

    @Test
    public void merkleProofDecodeTest() throws IOException {
        InputStream stream = this.getClass().getResourceAsStream("/merkleProof/merkleProof.txt");
        // create a reader for the input stream
        BufferedReader reader = new BufferedReader(new InputStreamReader(stream, StandardCharsets.UTF_8));

        // read the file contents into a string
        StringBuilder stringBuilder = new StringBuilder();
        String line;
        while ((line = reader.readLine()) != null) {
            stringBuilder.append(line);
        }
        String proof = stringBuilder.toString();
        byte[] merkleProto = StringUtil.hexToBytes(proof);
        var merkleProof = MerkleProof.decode(merkleProto);
        assertFalse(isMerkleProofEmpty(merkleProof));
    }
}
