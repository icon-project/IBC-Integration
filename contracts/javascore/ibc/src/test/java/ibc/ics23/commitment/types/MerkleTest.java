package ibc.ics23.commitment.types;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.StringUtil;
import icon.proto.clients.tendermint.MerkleRoot;
import icon.proto.core.commitment.MerklePath;
import icon.proto.core.commitment.MerkleProof;
import score.Context;

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
                StringUtil.hexToBytes(
                        "5b3233372c3235312c34392c3133382c3135342c3134382c38392c3230312c3133342c3130352c39302c31302c3139372c3138382c31352c37382c3134372c3232382c34322c3233392c39352c33312c35332c3232342c32392c3131392c34362c3139312c3133322c3136312c36322c3232325d"),
                StringUtil.hexToBytes("0a207526c2b51c1fdccd86bd4fab4f0af762242c50b321829b11d04e81b52db83bbf"),
                StringUtil.hexToBytes(
                        "0af8040af5040a4e0378167721f3f0bd57c20c4c783db10b95cc1207d5b980c02fc252b4825b9c87b2000b636f6d6d69746d656e747349bacce2ceb6c4de2100c6eb6e0487d950611f6b7030528d52c775cdbd5b37fb12745b3233372c3235312c34392c3133382c3135342c3134382c38392c3230312c3133342c3130352c39302c31302c3139372c3138382c31352c37382c3134372c3232382c34322c3233392c39352c33312c35332c3232342c32392c3131392c34362c3139312c3133322c3136312c36322c3232325d1a0d0801180120012a050002c4ee01222b080112270204c4ee0120fc46e8bb509f1951a65959401dde9a68bad607c87e8d0f6152647d7a684f81f020222d080112060408c4ee01201a21207b8dd72e718a3434708ec7731783a5f7624007eb8e0b1e40afdb767daa7e9047222d080112060610c4ee01201a212057d44f314fa3705f675d651d13f57fe586c891194784e6522470d7981d18d3cd222b08011227081ac4ee0120e2b966fe98f6e6af31349c6df7c4bea3816cca4ff18f340932fa0117c680f7d520222b080112270a389c8902204d6041a194c02a54fa2876959f028cf06bac19fe103f4a3ebf0455ff9f7c17f920222b080112270c649c890220e191359cb6264f682153f56f009493e4ec0b560dabc91a5253a49e789d647f5a20222e080112070ea2019c8902201a2120a3b649e597860cdaf044c4135d47f94253f60f5df49cd7e715d14fc75892ad45222c0801122810ea019c890220d78791d539e2c8ab7b603ca827439de6df2a47d61b018d6ec86763289b849de520222e0801120712e4029c8902201a2120d66da5d116a0e94a3d022e498047385b8087f6c4c360ad1a86fcf6158ec76e230a84010a81010a047761736d1220484365d71a5f276ffe5e701837ca721019e8b26e291e713ec602b36b46c64f7f1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b92225080112210132f9741df364c46bcd79fc3fdcbe21e75b82a801ee669ef2a048d8daf57fbee1"),
                "0378167721f3f0bd57c20c4c783db10b95cc1207d5b980c02fc252b4825b9c87b2" + "000b" + "636f6d6d69746d656e7473"
                        + "49bacce2ceb6c4de2100c6eb6e0487d950611f6b7030528d52c775cdbd5b37fb");

        var secondCase = Arguments.of(
                ByteUtil.convertBytesToStringBytes("7315ba98a01f9bbf1a74be1de1e439c10c2c5336bb8dabe3f48283812f57b8e2"),
                StringUtil.hexToBytes("0a207c6f612b6870db9665d303e17a540089cab418e23ff138993dbe1357e6b6c6d1"),
                StringUtil.hexToBytes(
                        "0a9d050a9a050a4e037f150b01df9a8a01b94842286bd9c75bcd85ea42aebd5e0f0d350127f09f79f2000b636f6d6d69746d656e7473b02d3f34d9c78cf5e5749f771178a8caf04e71042ccf3f69aeed05b080f3367712765b3131352c32312c3138362c3135322c3136302c33312c3135352c3139312c32362c3131362c3139302c32392c3232352c3232382c35372c3139332c31322c34342c38332c35342c3138372c3134312c3137312c3232372c3234342c3133302c3133312c3132392c34372c38372c3138342c3232365d1a0c0801180120012a040002a264222a080112260204a26420e582d14383533f189a1d930386b0c6383717601fb03859992442d5d7eaf5f84220222c080112050408a264201a21208e849be8fb555e7b822755befbd583346d1a662ea39901bfd819b40c9dbc1dfc222a08011226060ca264206773f71b90419571908178a27583fe82b66d420a4dcc84dde48323e3516dd90a20222a080112260814a26420c15db530a055bd625e21dfa364ed2d444c7879d4bd8dce61caf2d50e3c02fcbb20222c080112050a2ca264201a2120094c7e4fa1dd53d7eb01286b57f31a7263f5f94a96c836b3f90cda845a4df46f222a080112260c3ea26420e64fa6c52cc25defa5917ff7bdd83925e3235ab8e5445877fbc6d2b774d5b56f20222d080112060e8601a264201a2120e8d3caeb0e58761010c45e40700af39000448f83e1f15d31e41f9875eedec28a222b0801122710f601a2642003299c2a17e70aae40d1c9610a75be2f03a4c3724826dccb32cf3b71c0c9a60d20222d0801120612ba03a264201a21203b4ad7554b833d7d6fbd17a66f994464c8ebea3c1efbb041f4e58d3abcdf33e2222b08011227148005a26420895177caa5b1042f8df31b8f8dcd6acf83c2955c91177f4148733025d76d818f200a84010a81010a047761736d12200846956b667e3eb0c9cfa1e8238f9f3312b155b0f77a240150bac6e0ef8a29f81a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b9222508011221012a327b6f80d6f88f9592cea8bb28de552ee88503fb1d48b23f5c3e5eeef7f367"),
                "037f150b01df9a8a01b94842286bd9c75bcd85ea42aebd5e0f0d350127f09f79f2000b636f6d6d69746d656e7473b02d3f34d9c78cf5e5749f771178a8caf04e71042ccf3f69aeed05b080f33677");

        var thirdCase = Arguments.of(
                "\"Sabin\"".getBytes(),
                StringUtil.hexToBytes("0a207526c2b51c1fdccd86bd4fab4f0af762242c50b321829b11d04e81b52db83bbf"),
                StringUtil.hexToBytes(
                        "0af1030aee030a3003294bc3c38158bb11acceb7bf00b32f56eafd1b3efe94ac8d119863d03e88eb8d0008746573745f6d617068656c6c6f120722536162696e221a0d0801180120012a0500029c8902222b0801122702049c890220c1857c76f2123d08d07eb845a6aabe5d778dd4fff663feb541adad36c2843be520222b0801122704089c8902201619b99c8fc723af6cb09611a3ac56c5eabb766fa188906357e7c01afa01a71a20222d08011206060c9c8902201a2120b1a6002caf29d74f3a00f9885a28e8d028a0340627a770752ea3c156038b532a222d08011206081c9c8902201a212027f4239c31fcbdf181028c3ab0e8d5bb07001987b6125017bab7e2c9df79bc86222d080112060a2c9c8902201a2120ce12a762ac42ffd56abecbc7a1dffc2f3753445e6dc6a4db00981d55acd13b13222d080112060c649c8902201a2120e60c9e199c466bf68cb74af9a74f7d5b9ed4b43a00f993dcb238ae44cbf26e0a222e080112070ea2019c8902201a2120a3b649e597860cdaf044c4135d47f94253f60f5df49cd7e715d14fc75892ad45222c0801122810ea019c890220d78791d539e2c8ab7b603ca827439de6df2a47d61b018d6ec86763289b849de520222e0801120712e4029c8902201a2120d66da5d116a0e94a3d022e498047385b8087f6c4c360ad1a86fcf6158ec76e230a84010a81010a047761736d1220484365d71a5f276ffe5e701837ca721019e8b26e291e713ec602b36b46c64f7f1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b92225080112210132f9741df364c46bcd79fc3fdcbe21e75b82a801ee669ef2a048d8daf57fbee1"),
                "03" + "294bc3c38158bb11acceb7bf00b32f56eafd1b3efe94ac8d119863d03e88eb8d" + "0008"
                        + "746573745f6d617068656c6c6f");
        var forthCase = Arguments.of(
                StringUtil.hexToBytes("606df30f661a3eafcd1302de1b4174773e26f58e079e3e99b3555de81bfa4f41"),
                StringUtil.hexToBytes("0a20fa66313d4b61e7029b32add2b4b85337a090840745c186e128ea6ba3620da5f8"),
                StringUtil.hexToBytes(
                        "0ab7050ab4050a4e0317ffd6070e0fb2a524acfae322927c4940f61b5b56b8cda60e3edb16cd29f8c7000b636f6d6d69746d656e747394f07bf7c4566b848f8019b4160f4aad607c49ef42fb470dd7c7dadb1f8c729a1220606df30f661a3eafcd1302de1b4174773e26f58e079e3e99b3555de81bfa4f411a0d0801180120012a050002c2d103222d080112060204c4d103201a2120014080cdd1cb8b7c4c6a159a691d05b48a737c29ca6cd94c1cc61f61cfb4e1ee222d080112060406c4d103201a212081609d28e894708d5e76e7e1c4a996bc55657c7b5d201f5b021b4f119f82ca82222d08011206060ac4d103201a2120313698aa255db162ab8f0df05fbdf8f36f80c14e86d9c0a6a2c09e9f0ed72ae2222d080112060818eed403201a2120f3df63970fd3bec77a3069332938a5ae28d3e2b306d40fa887ef9aaabdcccec7222b080112270a2ceed40320658abc8bcef3125d3fe7f6669393d88810ff1ca6d46356755d9cc4b9b1e7d43c20222b080112270c48eed40320c4c338d3b8c4837cf6ba0f420233f6e961f3cc379ea0fbc61e541c0f5e43079420222d080112060e6ceed403201a21207f44fe81170c8dd3aab5aa55a9b1333b236674630d4308ea589d25e31543b80e222e0801120710aa01eed403201a21207fadfa1bd48e099c373d321dcb49bdf14049086bd6680f36786a43462de94b75222c0801122812ec03eed40320dac81ef1f02e0389c3da167655c37a9b01619abce7c13cc02dae9e8d7184874120222e08011207149606eed403201a2120fbbcfbbd8a90265d58eb5c6d2b621e81f69a633455c97258e0f46adbfc9b73b1222e08011207168809eed403201a212006e383eb108ff4087ba00f2e7c341cb2e63f72003b12c1db5836b05b25ab67c6222e080112071ab815eed403201a2120d1918b99c3ac11ccf1b1eb23c720cd32289850206a306327021455f05de5c21b0a84010a81010a047761736d12204fc573d5a6f59b503d363cf2662115050566c06e27787c430864bc2ec1d80dfe1a090801180120012a0100222508011221011107704879ce264af2b8ca54a7ad461538067d296f22b7de0482e4fdf43314b922250801122101452e562b4bc5e07f883cefcf4ca9a9f944d9f5ed47b87903e3aa2fc0dfd63164"),
                "0317ffd6070e0fb2a524acfae322927c4940f61b5b56b8cda60e3edb16cd29f8c7000b636f6d6d69746d656e747394f07bf7c4566b848f8019b4160f4aad607c49ef42fb470dd7c7dadb1f8c729a");

        return Stream.of(firstCase, secondCase, thirdCase, forthCase);
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

        Merkle.verifyMembership(merkleProof, SDK_SPEC, merkleRoot, merklePath, value);
    }

    @Test
    public void testPrefixLengthInBigEndian() {
        // Test with an array of length 10
        byte[] input = new byte[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
        byte[] expected = new byte[] { 0, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
        assertArrayEquals(prefixLengthInBigEndian(input), expected);

        // Test with an empty array
        input = new byte[] {};
        expected = new byte[] { 0, 0 };
        assertArrayEquals(prefixLengthInBigEndian(input), expected);

        // Test with an array of length 1
        input = new byte[] { 1 };
        expected = new byte[] { 0, 1, 1 };
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
