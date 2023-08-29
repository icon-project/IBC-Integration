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

package foundation.icon.xcall;

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
import java.util.Arrays;
import java.util.List;
import java.util.Map;

public class CallServiceImpl implements CallService, FeeManage {
    public static final int MAX_DATA_SIZE = 2048;
    public static final int MAX_ROLLBACK_SIZE = 1024;
    public static String NID;

    private final VarDB<BigInteger> sn = Context.newVarDB("sn", BigInteger.class);
    private final VarDB<BigInteger> reqId = Context.newVarDB("reqId", BigInteger.class);

    private final DictDB<BigInteger, CallRequest> requests = Context.newDictDB("requests", CallRequest.class);
    private final DictDB<BigInteger, CSMessageRequest> proxyReqs = Context.newDictDB("proxyReqs", CSMessageRequest.class);
    private final BranchDB<byte[], DictDB<String, Boolean>> pendingReqs = Context.newBranchDB("pendingReqs", Boolean.class);
    private final BranchDB<byte[], DictDB<String, Boolean>> pendingResponses = Context.newBranchDB("pendingResponses", Boolean.class);
    private final DictDB<BigInteger, Boolean> successfulResponses = Context.newDictDB("successfulResponses", Boolean.class);

    private final DictDB<String, Address> defaultConnection = Context.newDictDB("defaultConnection", Address.class);

    // for fee-related operations
    private final VarDB<Address> admin = Context.newVarDB("admin", Address.class);
    private final VarDB<BigInteger> protocolFee = Context.newVarDB("protocolFee", BigInteger.class);
    private final VarDB<Address> feeHandler = Context.newVarDB("feeHandler", Address.class);

    public CallServiceImpl(String networkId) {
        NID = networkId;
        if (admin.get() == null) {
            admin.set(Context.getCaller());
            feeHandler.set(Context.getCaller());
        }
    }

    /* Implementation-specific external */
    @External(readonly = true)
    public String getNetworkAddress() {
        return new NetworkAddress(NID, Context.getAddress()).toString();
    }

    @External(readonly = true)
    public String getNetworkId() {
        return NID;
    }

    private void checkCallerOrThrow(Address caller, String errMsg) {
        Context.require(Context.getCaller().equals(caller), errMsg);
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
    public BigInteger sendCallMessage(String _to,
                                      byte[] _data,
                                      @Optional byte[] _rollback,
                                      @Optional String[] sources,
                                      @Optional String[] destinations) {
        Address caller = Context.getCaller();
        // check if caller is a contract or rollback data is null in case of EOA
        Context.require(_rollback == null || caller.isContract(), "RollbackNotPossible");
        // check size of payloads to avoid abusing
        Context.require(_rollback == null || _rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded");

        boolean needResponse = _rollback != null && _rollback.length > 0;
        NetworkAddress dst = NetworkAddress.valueOf(_to);

        BigInteger sn = getNextSn();
        if (needResponse) {
            CallRequest req = new CallRequest(caller, dst.net(), sources, _rollback);
            requests.set(sn, req);
        }

        String from = new NetworkAddress(NID, caller.toString()).toString();
        CSMessageRequest msgReq = new CSMessageRequest(from, dst.account(), sn, needResponse, _data, destinations);

        byte[] msgBytes = msgReq.toBytes();
        Context.require(msgBytes.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded");

        if (sources == null || sources.length == 0) {
            Address src = defaultConnection.get(dst.net());
            Context.require(src != null, "NoDefaultConnection");
            BigInteger fee = Context.call(BigInteger.class, src, "getFee", dst.net(), needResponse);
            sendMessage(src, fee, dst.net(), CSMessage.REQUEST,
                    needResponse ? sn : BigInteger.ZERO, msgBytes);
        } else {
            for (String _src : sources) {
                Address src = Address.fromString(_src);
                BigInteger fee = Context.call(BigInteger.class, src, "getFee", dst.net(), needResponse);
                sendMessage(src, fee, dst.net(), CSMessage.REQUEST,
                        needResponse ? sn : BigInteger.ZERO, msgBytes);
            }

        }

        BigInteger protocolFee = getProtocolFee();
        BigInteger balance = Context.getBalance(Context.getAddress());
        Context.require(balance.compareTo(protocolFee) >= 0);
        Context.transfer(feeHandler.get(), balance);

        CallMessageSent(caller, dst.toString(), sn);

        return sn;
    }

    @Override
    @External
    public void executeCall(BigInteger _reqId, byte[] _data) {
        CSMessageRequest req = proxyReqs.get(_reqId);
        Context.require(req != null, "InvalidRequestId");
        // cleanup
        proxyReqs.set(_reqId, null);
        // compare the given data hash with the saved one
        Context.require(Arrays.equals(getDataHash(_data), req.getData()), "DataHashMismatch");

        NetworkAddress from = NetworkAddress.valueOf(req.getFrom());
        CSMessageResponse msgRes = null;
        String msg = "";

        try {
            Address to = Address.fromString(req.getTo());
            sendToDapp(to, req.getFrom(), _data, req.getProtocols());
            msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.SUCCESS);
        } catch (UserRevertedException e) {
            int code = e.getCode();
            msg = "UserReverted(" + code + ")";
            msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.FAILURE);
        } catch (IllegalArgumentException | RevertedException e) {
            msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.FAILURE);
            msg = e.toString();
        } finally {
            if (msgRes == null) {
                msgRes = new CSMessageResponse(req.getSn(), CSMessageResponse.FAILURE);
                msg = "UnknownFailure";
            }
            CallExecuted(_reqId, msgRes.getCode(), msg);
            // send response only when there was a rollback
            if (!req.needRollback()) {
                return;
            }

            BigInteger sn = req.getSn().negate();
            if (req.getProtocols().length == 0) {
                Address src = defaultConnection.get(from.net());
                Context.require(src != null, "NoDefaultConnection");
                sendMessage(src, BigInteger.ZERO, from.net(), CSMessage.RESPONSE, sn, msgRes.toBytes());
            } else {
                for (String protocol : req.getProtocols()) {
                    sendMessage(Address.fromString(protocol), BigInteger.ZERO, from.net(), CSMessage.RESPONSE, sn, msgRes.toBytes());
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

        sendToDapp(req.getFrom(), getNetworkAddress(), req.getRollback(), req.getProtocols());

        RollbackExecuted(_sn);
    }

    @External(readonly = true)
    public boolean verifySuccess(BigInteger sn) {
        return successfulResponses.getOrDefault(sn, false);
    }

    @Override
    @EventLog(indexed = 3)
    public void CallMessage(String _from, String _to, BigInteger _sn, BigInteger _reqId, byte[] _data) {
    }

    @Override
    @EventLog(indexed = 1)
    public void CallExecuted(BigInteger _reqId, int _code, String _msg) {
    }

    @Override
    @EventLog(indexed = 1)
    public void ResponseMessage(BigInteger _sn, int _code) {
    }

    @Override
    @EventLog(indexed = 1)
    public void RollbackMessage(BigInteger _sn) {
    }

    @Override
    @EventLog(indexed = 1)
    public void RollbackExecuted(BigInteger _sn) {
    }

    @Override
    @EventLog(indexed = 3)
    public void CallMessageSent(Address _from, String _to, BigInteger _sn) {
    }

    /* ========== Interfaces with BMC ========== */
    @External
    public void handleBTPMessage(String _from, String _svc, BigInteger _sn, byte[] _msg) {
        handleMessage(_from, _msg);
    }

    @External
    public void handleBTPError(String _src, String _svc, BigInteger _sn, long _code, String _msg) {
        handleError(_sn);
    }
    /* ========================================= */


    @Override
    @External
    public void handleMessage(String _from, byte[] _msg) {
        CSMessage msg = CSMessage.fromBytes(_msg);
        switch (msg.getType()) {
            case CSMessage.REQUEST:
                handleRequest(_from, msg.getData());
                break;
            case CSMessage.RESPONSE:
                handleResponse(msg.getData());
                break;
            default:
                Context.revert("UnknownMsgType(" + msg.getType() + ")");
        }
    }

    @Override
    @External
    public void handleError(BigInteger _sn) {
        CSMessageResponse res = new CSMessageResponse(_sn, CSMessageResponse.FAILURE);
        handleResponse(res.toBytes());
    }

    private BigInteger sendMessage(Address _connection, BigInteger value, String netTo, int msgType, BigInteger sn, byte[] data) {
        CSMessage msg = new CSMessage(msgType, data);
        ConnectionScoreInterface connection = new ConnectionScoreInterface(_connection);
        return connection.sendMessage(value, netTo, NAME, sn, msg.toBytes());
    }

    private void sendToDapp(Address dapp, String from, byte[] data, String[] protocols) {
        if (protocols.length == 0) {
            Context.call(dapp, "handleCallMessage", from, data);
        } else {
            Context.call(dapp, "handleCallMessage", from, data, protocols);
        }
    }

    private void handleRequest(String netFrom, byte[] data) {
        CSMessageRequest msgReq = CSMessageRequest.fromBytes(data);
        String from = msgReq.getFrom();
        Context.require(NetworkAddress.valueOf(from).net().equals(netFrom));
        Address caller = Context.getCaller();

        if (!verifyProtocols(pendingReqs, netFrom,msgReq.getProtocols(), caller, data)) {
            return;
        }

        String to = msgReq.getTo();
        BigInteger reqId = getNextReqId();

        // emit event to notify the user
        CallMessage(from, to, msgReq.getSn(), reqId, msgReq.getData());

        msgReq.hashData();
        proxyReqs.set(reqId, msgReq);
    }

    private void handleResponse(byte[] data) {
        CSMessageResponse msgRes = CSMessageResponse.fromBytes(data);
        BigInteger resSn = msgRes.getSn();
        CallRequest req = requests.get(resSn);
        Address caller = Context.getCaller();

        if (req == null) {
            Context.println("handleResponse: no request for " + resSn);
            return; // just ignore
        }

        if (!verifyProtocols(pendingResponses, req.getTo(), req.getProtocols(), caller, data)) {
            return;
        }

        ResponseMessage(resSn, msgRes.getCode());
        switch (msgRes.getCode()) {
            case CSMessageResponse.SUCCESS:
                cleanupCallRequest(resSn);
                successfulResponses.set(resSn, true);
                break;
            case CSMessageResponse.FAILURE:
            default:
                // emit rollback event
                Context.require(req.getRollback() != null, "NoRollbackData");
                req.setEnabled();
                requests.set(resSn, req);
                RollbackMessage(resSn);
        }
    }

    private boolean verifyProtocols(BranchDB<byte[], DictDB<String, Boolean>> db, String net, String[] protocols, Address caller, byte[] data) {
        if (protocols.length > 1) {
            byte[] hash = Context.hash("sha-256", data);
            DictDB<String, Boolean> pending = db.at(hash);
            pending.set(caller.toString(), true);
            for (String protocol : protocols) {
                if (!pending.getOrDefault(protocol, false)) {
                    return false;
                }
            }

            for (String protocol : protocols) {
                pending.set(protocol, null);
            }
        } else if (protocols.length == 1) {
            Context.require(caller.toString().equals(protocols[0]));
        } else {
            Context.require(caller.equals(defaultConnection.get(net)));
        }
        return true;
    }

    private byte[] getDataHash(byte[] data) {
        return Context.hash("keccak-256", data);
    }

    @External(readonly = true)
    public Address admin() {
        return admin.get();
    }

    @External
    public void setAdmin(Address _address) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        admin.set(_address);
    }

    @External
    public void setProtocolFee(BigInteger _value) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        Context.require(_value.signum() >= 0, "ValueShouldBePositive");
        protocolFee.set(_value);
    }

    @External
    public void setProtocolFeeHandler(Address address) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        feeHandler.set(address);
    }

    @External
    public void setDefaultConnection(String nid, Address connection) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        defaultConnection.set(nid, connection);
    }

    @External(readonly = true)
    public BigInteger getProtocolFee() {
        return protocolFee.getOrDefault(BigInteger.ZERO);
    }

    @External(readonly = true)
    public BigInteger getFee(String _net, boolean _rollback, @Optional String[] sources) {
        BigInteger fee = getProtocolFee();
        if (sources == null || sources.length == 0) {
            Address src = defaultConnection.get(_net);
            Context.require(src != null, "NoDefaultConnection");

            return fee.add(Context.call(BigInteger.class, src, "getFee", _net, _rollback));
        }

        for (String protocol : sources) {
            Address address = Address.fromString(protocol);
            fee = fee.add(Context.call(BigInteger.class, address, "getFee", _net, _rollback));
        }

        return fee;
    }
}
