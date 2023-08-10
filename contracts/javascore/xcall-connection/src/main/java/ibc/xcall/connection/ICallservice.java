package ibc.xcall.connection;

import java.math.BigInteger;

import foundation.icon.score.client.ScoreInterface;
import score.annotation.External;

@ScoreInterface
public interface ICallservice {

    /**
     * Handles Messages from other blockchains.
     *
     * @param _from String ( Network Address of source network )
     * @param _msg Bytes ( serialized bytes of ServiceMessage )
     */
    @External
    void handleMessage(String _from, byte[] _msg);

    /**
     * Handle the error on delivering the message.
     *
     * @param _sn Integer ( serial number of the original message )
     */
    @External
    void handleError(BigInteger _sn);
}