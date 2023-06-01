package ibc.ics23.commitment.types;

import ibc.icon.score.util.StringUtil;
import icon.proto.clients.tendermint.MerkleRoot;
import icon.proto.core.commitment.MerklePath;
import icon.proto.core.commitment.MerkleProof;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

import static ibc.ics23.commitment.types.Merkle.*;
import static org.junit.jupiter.api.Assertions.*;

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

    private static Stream<Arguments> provideDataForMembershipVerification() {
        var firstCase = Arguments.of(
                StringUtil.hexToBytes("5b3233372c3235312c34392c3133382c3135342c3134382c38392c3230312c3133342c3130352c39302c31302c3139372c3138382c31352c37382c3134372c3232382c34322c3233392c39352c33312c35332c3232342c32392c3131392c34362c3139312c3133322c3136312c36322c3232325d"),
                StringUtil.hexToBytes("0a207526c2b51c1fdccd86bd4fab4f0af762242c50b321829b11d04e81b52db83bbf"),
                StringUtil.hexToBytes("0af8040af5040a4e0378167721f3f0bd57c20c4c783db10b95cc1207d5b980c02fc252b4825b9c87b2000b636f6d6d69746d656e747349bacce2ceb6c4de2100c6eb6e0487d950611f6b7030528d52c775cdbd5b37fb12745b3233372c3235312c34392c3133382c3135342c3134382c38392c3230312c3133342c3130352c39302c31302c3139372c3138382c31352c37382c3134372c3232382c34322c3233392c39352c33312c35332c3232342c32392c3131392c34362c3139312c3133322c3136312c36322c3232325d1a0d0801180120012a050002c4ee01222b080112270204c4ee0120fc46e8bb509f1951a65959401dde9a68bad607c87e8d0f6152647d7a684f81f020222d080112060408c4ee01201a21207b8dd72e718a3434708ec7731783a5f7624007eb8e0b1e40afdb767daa7e9047222d080112060610c4ee01201a212057d44f314fa3705f675d651d13f57fe586c891194784e6522470d7981d18d3cd222b08011227081ac4ee0120e2b966fe98f6e6af31349c6df7c4bea3816cca4ff18f340932fa0117c680f7d520222b080112270a389c8902204d6041a194c02a54fa2876959f028cf06bac19fe103f4a3ebf0455ff9f7c17f920222b080112270c649c890220e191359cb6264f682153f56f009493e4ec0b560dabc91a5253a49e789d647f5a20222e080112070ea2019c8902201a2120a3b649e597860cdaf044c4135d47f94253f60f5df49cd7e715d14fc75892ad45222c0801122810ea019c890220d78791d539e2c8ab7b603ca827439de6df2a47d61b018d6ec86763289b849de520222e0801120712e4029c8902201a2120d66da5d116a0e94a3d022e498047385b8087f6c4c360ad1a86fcf6158ec76e230a84010a81010a047761736d1220484365d71a5f276ffe5e701837ca721019e8b26e291e713ec602b36b46c64f7f1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b92225080112210132f9741df364c46bcd79fc3fdcbe21e75b82a801ee669ef2a048d8daf57fbee1"),
                "0378167721f3f0bd57c20c4c783db10b95cc1207d5b980c02fc252b4825b9c87b2" + "000b" + "636f6d6d69746d656e7473" + "49bacce2ceb6c4de2100c6eb6e0487d950611f6b7030528d52c775cdbd5b37fb");

//        var secondCase = Arguments.of(
//                IBCCommitment.keccak256(StringUtil.hexToBytes("0a0f30372d74656e6465726d696e742d3012230a0131120d4f524445525f4f524445524544120f4f524445525f554e4f5244455245441802222e0a0f30372d74656e6465726d696e742d30120c636f6e6e656374696f6e2d301a0d0a0b636f6d6d69746d656e7473")),
//                StringUtil.hexToBytes("0a2099306eba529fb6416b0984146b97c9c76386f226e9541a47197fa7ada530eda3"),
//                StringUtil.hexToBytes("0a8f040a8c040a4e03f04a313a7349b120c55c99788f12f712176bb3e5926d012d0ea72fa2bbb85051000b636f6d6d69746d656e7473b02d3f34d9c78cf5e5749f771178a8caf04e71042ccf3f69aeed05b080f3367712715b38302c31302c3233382c31382c3132332c3134352c3231372c3232342c3235322c35382c3130382c31352c3138352c3232372c3232392c3133322c3139332c39332c3132312c3139362c37312c37352c3230302c39322c3231332c3135372c3139372c302c39312c302c302c3133385d1a0c0801180120012a0400028c0d222a0801122602048c0d20c8f072590066f2d84558ce017b592e3402c8bbbb740466fffef024af3bf184b220222c0801120504088c0d201a212087b604d47d23a090496c5f96c36dcf0b20e471c3d0c5cd642d7d8b651dc5af6e222c0801120506108c0d201a2120a6e3f207000527040b5e333095a48c26cb4c18cb8158f11b524735fbf39892e7222a08011226081a8c0d208cfa9f85ff4b8fe3ae187fc0b2b135323a6c10e025bf16873fe4f9cdd35b2db820222a080112260a368c0d20a2af1926b596fe575536f4aa0c3ab13b3212ba33ad6f5cf37be45eadfc022a5720222c080112050c568c0d201a21208a546448e0ca699fc71d189b10625cf0bf06448acd757547b96eac848b351d9f222b080112270e86018c0d201a32bca44f05b567d13e642b45a4e95990676b28c0e30b01d4898f9c6206402b200a84010a81010a047761736d122017c28d08ed6b35ac4f59b079d55895ed560efa5b2631458672d81ffb7f44a48b1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b9222508011221018dad4411d3be8232d92b3266211f6c3211f3e8800ca2e4d22d1ce14d22be91b2"),
//                "03f04a313a7349b120c55c99788f12f712176bb3e5926d012d0ea72fa2bbb85051" + "000b" + "636f6d6d69746d656e7473" + "b02d3f34d9c78cf5e5749f771178a8caf04e71042ccf3f69aeed05b080f33677");

        var thirdCase = Arguments.of(
                "\"Sabin\"".getBytes(),
                StringUtil.hexToBytes("0a207526c2b51c1fdccd86bd4fab4f0af762242c50b321829b11d04e81b52db83bbf"),
                StringUtil.hexToBytes("0af1030aee030a3003294bc3c38158bb11acceb7bf00b32f56eafd1b3efe94ac8d119863d03e88eb8d0008746573745f6d617068656c6c6f120722536162696e221a0d0801180120012a0500029c8902222b0801122702049c890220c1857c76f2123d08d07eb845a6aabe5d778dd4fff663feb541adad36c2843be520222b0801122704089c8902201619b99c8fc723af6cb09611a3ac56c5eabb766fa188906357e7c01afa01a71a20222d08011206060c9c8902201a2120b1a6002caf29d74f3a00f9885a28e8d028a0340627a770752ea3c156038b532a222d08011206081c9c8902201a212027f4239c31fcbdf181028c3ab0e8d5bb07001987b6125017bab7e2c9df79bc86222d080112060a2c9c8902201a2120ce12a762ac42ffd56abecbc7a1dffc2f3753445e6dc6a4db00981d55acd13b13222d080112060c649c8902201a2120e60c9e199c466bf68cb74af9a74f7d5b9ed4b43a00f993dcb238ae44cbf26e0a222e080112070ea2019c8902201a2120a3b649e597860cdaf044c4135d47f94253f60f5df49cd7e715d14fc75892ad45222c0801122810ea019c890220d78791d539e2c8ab7b603ca827439de6df2a47d61b018d6ec86763289b849de520222e0801120712e4029c8902201a2120d66da5d116a0e94a3d022e498047385b8087f6c4c360ad1a86fcf6158ec76e230a84010a81010a047761736d1220484365d71a5f276ffe5e701837ca721019e8b26e291e713ec602b36b46c64f7f1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b92225080112210132f9741df364c46bcd79fc3fdcbe21e75b82a801ee669ef2a048d8daf57fbee1"),
                "03" + "294bc3c38158bb11acceb7bf00b32f56eafd1b3efe94ac8d119863d03e88eb8d" + "0008" + "746573745f6d617068656c6c6f"
        );

        return Stream.of(firstCase, thirdCase);
    }

    @ParameterizedTest
    @MethodSource("provideDataForMembershipVerification")
    public void verifyMembership(byte[] value, byte[] root, byte[] proof, String path) {

        var merkleRoot = MerkleRoot.decode(root);
        var merkleProof = MerkleProof.decode(proof);
        List<String> keyPath = new ArrayList<>();
        keyPath.add(StringUtil.bytesToHex("wasm".getBytes()));
        keyPath.add(path);
        var merklePath = new MerklePath();
        merklePath.setKeyPath(keyPath);

        Merkle.verifyMembership(merkleProof, getSDKSpecs(), merkleRoot, merklePath, value);
    }

    @Test
    public void testPrefixLengthInBigEndian() {
        // Test with an array of length 10
        byte[] input = new byte[]{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
        byte[] expected = new byte[]{0, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
        assertArrayEquals(prefixLengthInBigEndian(input), expected);

        // Test with an empty array
        input = new byte[]{};
        expected = new byte[]{0, 0};
        assertArrayEquals(prefixLengthInBigEndian(input), expected);

        // Test with an array of length 1
        input = new byte[]{1};
        expected = new byte[]{0, 1, 1};
        assertArrayEquals(prefixLengthInBigEndian(input), expected);

        // Test with an array of length 256
        input = new byte[256];
        for (int i = 0; i < 256; i++) {
            input[i] = (byte) i;
        }
        expected = new byte[258];
        expected[0] = 1;
        expected[1] = 0;
        for (int i = 0; i < 256; i++) {
            expected[i + 2] = (byte) i;
        }
        assertArrayEquals(prefixLengthInBigEndian(input), expected);
    }
}
