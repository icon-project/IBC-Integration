/*
 * Copyright 2022 ICON Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package foundation.icon.btp.xcall;

import foundation.icon.score.client.ScoreInterface;
import score.annotation.External;
import score.annotation.Payable;

import java.math.BigInteger;

@ScoreInterface
public interface Connection {
    /**
     * Sends the message to a specific network.
     * Only allowed to be called by registered BSHs.
     * _sn : positive for two-way message, zero for one-way message, negative for response
     *
     * @param _to  String ( Network Address of destination network )
     * @param _svc String ( name of the service )
     * @param _sn  Integer ( serial number of the message )
     * @param _msg Bytes ( serialized bytes of Service Message )
     * @return Integer ( network serial number of message )
     */
    @Payable
    @External
    BigInteger sendMessage(String _to, String _svc, BigInteger _sn, byte[] _msg);

    /**
     * Gets the fee to the target network
     * _response should be true if it uses positive value for _sn of {@link #sendMessage}.
     * If _to is not reachable, then it reverts.
     * If _to does not exist in the fee table, then it returns zero.
     *
     * @param _to       String ( BTP Network Address of the destination BMC )
     * @param _response Boolean ( Whether the responding fee is included )
     * @return Integer (The fee of sending a message to a given destination network )
     */
    @External(readonly = true)
    BigInteger getFee(String _to, boolean _response);

}
