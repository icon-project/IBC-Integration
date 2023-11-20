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

package xcall.sample.dapp;

import java.math.BigInteger;
import score.Address;
import score.Context;
import score.DictDB;
import score.UserRevertedException;
import score.VarDB;
import score.annotation.EventLog;
import score.annotation.External;
import score.annotation.Optional;
import score.annotation.Payable;
import foundation.icon.xcall.DefaultCallServiceReceiver;
import foundation.icon.xcall.NetworkAddress;

public class SimpleDapp implements DefaultCallServiceReceiver {
    private final Address callSvc;

    public SimpleDapp(Address _callService) {
        this.callSvc = _callService;
    }

    private void onlyCallService() {
        Context.require(Context.getCaller().equals(this.callSvc), "onlyCallService");
    }

    @Payable
    @External
    public void sendMessage(String _to, byte[] _data, @Optional byte[] _rollback) {
        _sendCallMessage(Context.getValue(), _to, _data, _rollback);
    }

    private BigInteger _sendCallMessage(BigInteger value, String to, byte[] data, byte[] rollback) {
        return Context.call(BigInteger.class, value, this.callSvc, "sendCallMessage", to, data, rollback);
    }

    @External
    public void handleCallMessage(String _from, byte[] _data) {
        onlyCallService();
        String rollbackAddress = Context.call(String.class, this.callSvc, "getNetworkAddress");
        Context.println("handleCallMessage: from=" + _from + ", data=" + new String(_data));
        if (rollbackAddress.equals(_from)) {
            return;
        } else {
            Context.require(!new String(_data).equals("rollback"), "failed");
            // normal message delivery
            MessageReceived(_from, _data);
        }
    }


    @EventLog
    public void MessageReceived(String _from, byte[] _data) {
    }

}
