package ibc.xcall.connection;

import java.math.BigInteger;

import foundation.icon.score.client.ScoreInterface;
import score.annotation.External;

@ScoreInterface
public interface ICallservice {

    /**
     * Handles BTP Messages from other blockchains.
     * Accepts messages only from BMC.
     * If it fails, then BMC will generate a BTP Message that includes error information, then delivered to the source.
     *
     * @param _from String ( Network Address of source network )
     * @param _svc String ( name of the service )
     * @param _sn Integer ( serial number of the message )
     * @param _msg Bytes ( serialized bytes of ServiceMessage )
     */
    @External
    void handleBTPMessage(String _from, String _svc, BigInteger _sn, byte[] _msg);

    /**
     * Handle the error on delivering the message.
     * Accept the error only from the BMC.
     *
     * @param _src String ( Network Address of BMC that generated the error )
     * @param _svc String ( name of the service )
     * @param _sn Integer ( serial number of the original message )
     * @param _code Integer ( code of the error )
     * @param _msg String ( message of the error )
     */
    @External
    void handleBTPError(String _src, String _svc, BigInteger _sn, long _code, String _msg);
}