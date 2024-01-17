package ibc.tendermint;

import score.Address;
import score.annotation.External;

import java.math.BigInteger;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.StringUtil;
import ibc.ics24.host.IBCCommitment;
import icon.ibc.interfaces.ILightClient;


public abstract class WasmLightClient extends TendermintLightClient implements ILightClient {
    private static final String WASM_PREFIX = StringUtil.bytesToHex("wasm".getBytes());
    public WasmLightClient(Address ibcHandler) {
        super(ibcHandler);
    }

    @External(readonly = true)
    public void verifyMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path,
            byte[] value) {
        value = IBCCommitment.keccak256(value);
        String stringPath = new String(ByteUtil.join(prefix, StringUtil.bytesToHex(IBCCommitment.keccak256(path)).getBytes()));
        super.verifyMembership(clientId, heightBytes, delayTimePeriod, delayBlockPeriod, proof, WASM_PREFIX, stringPath, value);
    }

    @External(readonly = true)
    public void verifyNonMembership(
            String clientId,
            byte[] heightBytes,
            BigInteger delayTimePeriod,
            BigInteger delayBlockPeriod,
            byte[] proof,
            byte[] prefix,
            byte[] path) {
        String stringPath = new String(ByteUtil.join(prefix, StringUtil.bytesToHex(IBCCommitment.keccak256(path)).getBytes()));
        super.verifyNonMembership(clientId, heightBytes, delayTimePeriod, delayBlockPeriod, proof, WASM_PREFIX, stringPath);
    }
}
