package ibc.ics23.commitment.types;

import ibc.icon.score.util.StringUtil;
import icon.proto.clients.tendermint.MerkleRoot;
import icon.proto.core.commitment.MerklePath;
import icon.proto.core.commitment.MerkleProof;
import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;

import static ibc.ics23.commitment.types.Merkle.getSDKSpecs;
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

    @Test
    public void verifyMembership() {
        String value = "\"Sabin\"";
        byte[] root = StringUtil.hexToBytes("0a203fd24935c86bf2c95cda9603320ead537de31b128582d3f758a0a3cb0725b6e1");
        byte[] proof = StringUtil.hexToBytes("0aa3010aa0010a3003ade4a5f5803a439835c636395a8d648dee57b2fc90d98dc17fa887159b69638b0008746573745f6d617068656c6c6f120722536162696e221a0b0801180120012a0300020c22290801122504060c2070e760162ca61a3f666eca51724eac2b2832da67ad638568e18e6888006a474d20222b0801120408140c201a2120cc3d5d4a190a622525d7e476e923ec2e42873089e61bd6be8975cd397bda0f0f0a84010a81010a047761736d12207ded2cbd7e31d89058350a47e76bbb543c64872a80d69f0b4990f656a13e5b631a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b922250801122101c339eb84ef036cff2ce525bf1a5ad77f091797ad6ae8616a3375aaac035af786");

        String path = "03" + "ade4a5f5803a439835c636395a8d648dee57b2fc90d98dc17fa887159b69638b" + "0008" +
                StringUtil.bytesToHex("test_maphello".getBytes());
        var merkleRoot = MerkleRoot.decode(root);
        var merkleProof = MerkleProof.decode(proof);
        List<String> keyPath = new ArrayList<>();
        keyPath.add(StringUtil.bytesToHex("wasm".getBytes()));
        keyPath.add(path);
        var merklePath = new MerklePath();
        merklePath.setKeyPath(keyPath);

        Merkle.verifyMembership(merkleProof, getSDKSpecs(), merkleRoot, merklePath, value.getBytes());
    }
}
