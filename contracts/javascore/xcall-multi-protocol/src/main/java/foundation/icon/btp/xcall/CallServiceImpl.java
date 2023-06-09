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

import score.Address;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.RevertedException;
import score.UserRevertedException;
import score.VarDB;
import score.annotation.EventLog;
import score.annotation.External;
import score.annotation.Optional;
import score.annotation.Payable;
import scorex.util.HashMap;

import java.math.BigInteger;
import java.util.List;
import java.util.Map;

public class CallServiceImpl implements CallService, FeeManage {
    public static final int MAX_DATA_SIZE = 2048;
    public static final int MAX_ROLLBACK_SIZE = 1024;
    public static String nid;

    private final VarDB<BigInteger> sn = Context.newVarDB("sn", BigInteger.class);
    private final VarDB<BigInteger> reqId = Context.newVarDB("reqId", BigInteger.class);

    private final DictDB<BigInteger, CallRequest> requests = Context.newDictDB("requests", CallRequest.class);
    private final DictDB<BigInteger, CSMessageRequest> proxyReqs = Context.newDictDB("proxyReqs", CSMessageRequest.class);
    private final BranchDB<byte[], DictDB<String, Boolean>> pendingReqs = Context.newBranchDB("pendingReqs", Boolean.class);
    private final BranchDB<BigInteger, DictDB<String, Boolean>> pendingResponses = Context.newBranchDB("pendingResponses", Boolean.class);

    // for fee-related operations
    private final VarDB<Address> admin = Context.newVarDB("admin", Address.class);
    private final VarDB<BigInteger> protocolFee = Context.newVarDB("protocolFee", BigInteger.class);

    public CallServiceImpl(String networkId) {
        nid = networkId;
    }

    /* Implementation-specific external */
    @External(readonly=true)
    public String getNetworkAddress() {
        return new NetworkAddress(nid, Context.getAddress()).toString();
    }

    private void checkCallerOrThrow(Address caller, String errMsg) {
        Context.require(Context.getCaller().equals(caller), errMsg);
    }

    private void onlyOwner() {
        checkCallerOrThrow(Context.getOwner(), "OnlyOwner");
    }

    private BigInteger getNextSn() {
        BigInteger _sn = this.sn.getOrDefault(BigInteger.ZERO);
        _sn = _sn.add(BigInteger.ONE);
        this.sn.set(_sn);
        return _sn;
    }

    private BigInteger getNextReqId() {
        BigInteger _reqId = this.reqId.getOrDefault(BigInteger.ZERO);
        _reqId = _reqId.add(BigInteger.ONE);
        this.reqId.set(_reqId);
        return _reqId;
    }

    private void cleanupCallRequest(BigInteger sn) {
        requests.set(sn, null);
    }

    @Override
    @Payable
    @External
    public BigInteger sendCallMessage(String _to, String[] sources, String[] destinations, byte[] _data,  @Optional byte[] _rollback) {
        Address caller = Context.getCaller();
        // check if caller is a contract or rollback data is null in case of EOA
        Context.require(_rollback == null || caller.isContract(), "RollbackNotPossible");
        // check size of payloads to avoid abusing
        Context.require(_data.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded");
        Context.require(_rollback == null || _rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded");

        boolean needResponse = _rollback != null;
        NetworkAddress dst = NetworkAddress.valueOf(_to);
        BigInteger value = Context.getValue();

        BigInteger requiredFee = BigInteger.ZERO;
        Map<Address, BigInteger> fees = new HashMap<>();
        int nrConnections = sources.length;
        Context.require(nrConnections == destinations.length);
        for (int i = 0; i < nrConnections; i++) {
            Address src = Address.fromString(sources[i]);
            BigInteger fee = getFee(src, dst.net(), needResponse);
            requiredFee = requiredFee.add(fee);
            fees.put(src, fee);
        }

        BigInteger protocolFee = getProtocolFee();
        requiredFee = requiredFee.add(protocolFee);
        Context.require(value.compareTo(requiredFee) >= 0, "InsufficientFee");

        BigInteger sn = getNextSn();
        if (needResponse) {
            CallRequest req = new CallRequest(caller, dst.toString(), sources, _rollback);
            requests.set(sn, req);
        }
        String from = new NetworkAddress(nid, caller.toString()).toString();
        CSMessageRequest msgReq = new CSMessageRequest(from, dst.account(), destinations,  sn, needResponse, _data);
        byte[] msgBytes = msgReq.toBytes();
        for (String _src : sources) {
            Address src = Address.fromString(_src);
                sendBTPMessage(src, fees.get(src), dst.net(), CSMessage.REQUEST,
                needResponse ? sn : BigInteger.ZERO, msgBytes);
        }
        CallMessageSent(caller, dst.toString(), sn);
        return sn;
    }

    @Override
    @External
    public void executeCall(BigInteger _reqId) {
        CSMessageRequest req = proxyReqs.get(_reqId);
        Context.require(req != null, "InvalidRequestId");
        // cleanup
        proxyReqs.set(_reqId, null);

        NetworkAddress from = NetworkAddress.valueOf(req.getFrom());
        CSMessageResponse msgRes = null;
        try {
            CallServiceReceiver proxy = new CallServiceReceiverScoreInterface(Address.fromString(req.getTo()));
            proxy.handleCallMessage(req.getFrom(), req.getData(), req.getProtocols());
            msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.SUCCESS, "");
        } catch (UserRevertedException e) {
            int code = e.getCode();
            String msg = "UserReverted(" + code + ")";
            msgRes = new CSMessageResponse(req.getSn(), code == 0 ? CSMessageResponse.FAILURE : code, msg);
        } catch (IllegalArgumentException | RevertedException e) {
            msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.FAILURE, e.toString());
        } finally {
            if (msgRes == null) {
                msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.FAILURE, "UnknownFailure");
            }
            CallExecuted(_reqId, msgRes.getCode(), msgRes.getMsg());
            // send response only when there was a rollback
            if (req.needRollback()) {
                BigInteger sn = req.getSn().negate();
                for (String protocol : req.getProtocols()) {
                    sendBTPMessage(Address.fromString(protocol), BigInteger.ZERO, from.net(), CSMessage.RESPONSE, sn, msgRes.toBytes());

                }
            }
        }
    }

    @Override
    @External
    public void executeRollback(BigInteger _sn) {
        CallRequest req = requests.get(_sn);
        Context.require(req != null, "InvalidSerialNum");
        Context.require(req.enabled(), "RollbackNotEnabled");
        cleanupCallRequest(_sn);

        CSMessageResponse msgRes = null;
        try {
            CallServiceReceiver proxy = new CallServiceReceiverScoreInterface(req.getFrom());
            proxy.handleCallMessage(getNetworkAddress(), req.getRollback(), req.getProtocols());
            msgRes = new CSMessageResponse(_sn, CSMessageResponse.SUCCESS, "");
        } catch (UserRevertedException e) {
            int code = e.getCode();
            String msg = "UserReverted(" + code + ")";
            msgRes = new CSMessageResponse(_sn, code == 0 ? CSMessageResponse.FAILURE : code, msg);
        } catch (IllegalArgumentException | RevertedException e) {
            msgRes = new CSMessageResponse(_sn, CSMessageResponse.FAILURE, e.toString());
        } finally {
            if (msgRes == null) {
                msgRes = new CSMessageResponse(_sn, CSMessageResponse.FAILURE, "UnknownFailure");
            }
            RollbackExecuted(_sn, msgRes.getCode(), msgRes.getMsg());
        }
    }

    @Override
    @EventLog(indexed=3)
    public void CallMessage(String _from, String _to, BigInteger _sn, BigInteger _reqId) {}

    @Override
    @EventLog(indexed=1)
    public void CallExecuted(BigInteger _reqId, int _code, String _msg) {}

    @Override
    @EventLog(indexed=1)
    public void ResponseMessage(BigInteger _sn, int _code, String _msg) {}

    @Override
    @EventLog(indexed=1)
    public void RollbackMessage(BigInteger _sn) {}

    @Override
    @EventLog(indexed=1)
    public void RollbackExecuted(BigInteger _sn, int _code, String _msg) {}

    @Override
    @EventLog(indexed=3)
    public void CallMessageSent(Address _from, String _to, BigInteger _sn) {}

    /* ========== Interfaces with BMC ========== */
    @External
    public void handleBTPMessage(String _from, String _svc, BigInteger _sn, byte[] _msg) {
        CSMessage msg = CSMessage.fromBytes(_msg);
        switch (msg.getType()) {
            case CSMessage.REQUEST:
                handleRequest(_from, _sn, msg.getData());
                break;
            case CSMessage.RESPONSE:
                handleResponse(_from, _sn, msg.getData());
                break;
            default:
                Context.revert("UnknownMsgType(" + msg.getType() + ")");
        }
    }

    @External
    public void handleBTPError(String _src, String _svc, BigInteger _sn, long _code, String _msg) {
        String errMsg = "BTPError{code=" + _code + ", msg=" + _msg + "}";
        CSMessageResponse res = new CSMessageResponse(_sn, CSMessageResponse.BTP_ERROR, errMsg);
        handleResponse(_src, _sn, res.toBytes());
    }
    /* ========================================= */

    private BigInteger sendBTPMessage(Address _connection, BigInteger value, String netTo, int msgType, BigInteger sn, byte[] data) {
        CSMessage msg = new CSMessage(msgType, data);
        ConnectionScoreInterface connection = new ConnectionScoreInterface(_connection);
        return connection.sendMessage(value, netTo, NAME, sn, msg.toBytes());
    }

    private void handleRequest(String netFrom, BigInteger sn, byte[] data) {
        CSMessageRequest msgReq = CSMessageRequest.fromBytes(data);
        String[] protocols = msgReq.getProtocols();
        String from = msgReq.getFrom();
        Context.require(NetworkAddress.valueOf(from).net().equals(netFrom));
        String source = Context.getCaller().toString();

        if (protocols.length > 1) {
            byte[] hash = Context.hash("sha-256", data);
            DictDB<String,Boolean> pendingRequests = pendingReqs.at(hash);
            pendingRequests.set(source, true);
            for (String protocol : protocols) {
                if (!pendingRequests.getOrDefault(protocol, false)) {
                    return;
                }
            }

            for (String protocol : protocols) {
                pendingRequests.set(protocol, null);
            }
        } else {
            Context.require(source.equals(protocols[0]));
        }
        String to = msgReq.getTo();
        BigInteger reqId = getNextReqId();
        proxyReqs.set(reqId, msgReq);

        // emit event to notify the user
        CallMessage(msgReq.getFrom(), to, msgReq.getSn(), reqId);
    }

    private void handleResponse(String netFrom, BigInteger sn, byte[] data) {
        CSMessageResponse msgRes = CSMessageResponse.fromBytes(data);
        BigInteger resSn = msgRes.getSn();
        CallRequest req = requests.get(resSn);
        String source = Context.getCaller().toString();

        if (req == null) {
            Context.println("handleResponse: no request for " + resSn);
            return; // just ignore
        }

        String[] protocols = req.getProtocols();
        if (protocols.length > 1) {
            DictDB<String,Boolean> pendingResponse = pendingResponses.at(resSn);
            pendingResponse.set(source, true);
            for (String protocol : protocols) {
                if (!pendingResponse.getOrDefault(protocol, false)) {
                    return;
                }
            }

            for (String protocol : protocols) {
                pendingResponse.set(protocol, null);
            }
        } else {
            Context.require(source.equals(protocols[0]));
        }

        String errMsg = msgRes.getMsg();
        ResponseMessage(resSn, msgRes.getCode(), errMsg != null ? errMsg : "");
        switch (msgRes.getCode()) {
            case CSMessageResponse.SUCCESS:
                cleanupCallRequest(resSn);
                break;
            case CSMessageResponse.FAILURE:
            case CSMessageResponse.BTP_ERROR:
            default:
                // emit rollback event
                Context.require(req.getRollback() != null, "NoRollbackData");
                req.setEnabled();
                requests.set(resSn, req);
                RollbackMessage(resSn);
        }
    }

    @External(readonly=true)
    public Address admin() {
        return admin.getOrDefault(Context.getOwner());
    }

    @External
    public void setAdmin(Address _address) {
        onlyOwner();
        admin.set(_address);
    }

    @External
    public void setProtocolFee(BigInteger _value) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        Context.require(_value.signum() >= 0, "ValueShouldBePositive");
        protocolFee.set(_value);
    }

    @External(readonly=true)
    public BigInteger getProtocolFee() {
        return protocolFee.getOrDefault(BigInteger.ZERO);
    }

    @External(readonly=true)
    public BigInteger getFee(Address _connection, String _net, boolean _rollback) {
        ConnectionScoreInterface connection = new ConnectionScoreInterface(_connection);
        return connection.getFee(_net, _rollback);
    }
}
