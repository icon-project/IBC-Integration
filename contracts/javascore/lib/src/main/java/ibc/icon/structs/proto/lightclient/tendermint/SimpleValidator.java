package ibc.icon.structs.proto.lightclient.tendermint;

import ibc.icon.score.util.ByteUtil;
import ibc.icon.score.util.Proto;

import java.math.BigInteger;

public class SimpleValidator {
    public PublicKey pubKey;
    public BigInteger votingPower;

    public SimpleValidator(PublicKey pubKey, BigInteger votingPower) {
        this.pubKey = pubKey;
        this.votingPower = votingPower;
    }

    public byte[] encode() {
        byte[] _pubKey = Proto.encode(1, this.pubKey.encode());
        byte[] _votingPower = Proto.encode(2, this.votingPower);

        return ByteUtil.join(_pubKey, _votingPower);
    }
}