package ibc.tendermint;


import score.Address;

import score.annotation.External;

import java.math.BigInteger;

import ibc.icon.score.util.StringUtil;
import ibc.tendermint.TendermintLightClient;
import icon.ibc.interfaces.ILightClient;


public abstract class ICS08LightClient extends TendermintLightClient implements ILightClient {
    public ICS08LightClient(Address ibcHandler) {
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

        super.verifyMembership(clientId, heightBytes, delayTimePeriod, delayBlockPeriod, proof, StringUtil.bytesToHex(prefix), StringUtil.bytesToHex(path), value);
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

        super.verifyNonMembership(clientId, heightBytes, delayTimePeriod, delayBlockPeriod, proof, StringUtil.bytesToHex(prefix), StringUtil.bytesToHex(path));
    }


}
