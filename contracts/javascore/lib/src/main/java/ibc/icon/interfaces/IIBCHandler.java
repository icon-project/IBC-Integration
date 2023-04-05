package ibc.icon.interfaces;

import java.math.BigInteger;

import foundation.icon.score.client.ScoreClient;
import score.Address;
import score.annotation.External;

@ScoreClient
public interface IIBCHandler {
    @External(readonly = true)
    String name();

    /**
     * bindPort binds to an unallocated port, failing if the port has already
     * been allocated.
     */
    @External
    void bindPort(String portId, Address moduleAddress);

    /**
     * setExpectedTimePerBlock sets expected time per block.
     */
    @External
    void setExpectedTimePerBlock(BigInteger expectedTimePerBlock);
}
