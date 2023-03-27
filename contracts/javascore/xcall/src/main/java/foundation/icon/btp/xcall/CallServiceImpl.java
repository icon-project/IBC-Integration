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

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Counterparty;
import ibc.icon.structs.proto.core.channel.Packet;
import ibc.icon.structs.proto.core.client.Height;
import ibc.ics25.handler.IBCHandler;
import score.Address;
import score.Context;
import score.DictDB;
import score.RevertedException;
import score.UserRevertedException;
import score.VarDB;
import score.annotation.EventLog;
import score.annotation.External;
import score.annotation.Optional;
import score.annotation.Payable;

import java.math.BigInteger;


public class CallServiceImpl implements IIBCModule {
    public static final int MAX_DATA_SIZE = 2048;
    public static final int MAX_ROLLBACK_SIZE = 1024;

    private final VarDB<Address> ibcHandler = Context.newVarDB("ibcHandler", Address.class);
    private final VarDB<BigInteger> sn = Context.newVarDB("sn", BigInteger.class);
    private final VarDB<BigInteger> reqId = Context.newVarDB("reqId", BigInteger.class);
    private final VarDB<BigInteger> timeoutHeight = Context.newVarDB("timeoutHeight", BigInteger.class);

    private final DictDB<BigInteger, CallRequest> requests = Context.newDictDB("requests", CallRequest.class);
    private final DictDB<BigInteger, CSMessageRequest> proxyReqs = Context.newDictDB("proxyReqs", CSMessageRequest.class);

    // for fee-related operations
    private final VarDB<Address> admin = Context.newVarDB("admin", Address.class);
    private final VarDB<Address> feeHandler = Context.newVarDB("feeHandler", Address.class);
    private final VarDB<BigInteger> protocolFee = Context.newVarDB("protocolFee", BigInteger.class);
    private final VarDB<String> sourcePort = Context.newVarDB("sourcePort", String.class);
    private final VarDB<String> sourceChannel = Context.newVarDB("sourceChannel", String.class);
    private final VarDB<String> destinationPort = Context.newVarDB("destinationPort", String.class);
    private final VarDB<String> destinationChannel = Context.newVarDB("destinationChannel", String.class);

    public CallServiceImpl(Address _ibc) {
        this.ibcHandler.set(_ibc);


    }

    public void setTimeoutHeight(BigInteger _timeoutHeight) {
        onlyOwner();
        this.timeoutHeight.set(_timeoutHeight);
    }

    @External(readonly = true)
    public BigInteger getTimeoutHeight() {
        return this.timeoutHeight.getOrDefault(BigInteger.ZERO);
    }

    private void checkCallerOrThrow(Address caller, String errMsg) {
        Context.require(Context.getCaller().equals(caller), errMsg);
    }

    private void onlyOwner() {
        checkCallerOrThrow(Context.getOwner(), "OnlyOwner");
    }

    private void onlyIBCHandler() {
        checkCallerOrThrow(ibcHandler.get(), "OnlyIBCHandler");
    }

//    private void checkService(String _svc) {
//        Context.require(NAME.equals(_svc), "InvalidServiceName");
//    }

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


    @Payable
    @External
    public BigInteger sendCallMessage(String _to, byte[] _data, @Optional byte[] _rollback) {
        BigInteger sn = getNextSn();
        Address caller = Context.getCaller();
        Context.require(caller.isContract() || _rollback == null, "RollbackNotPossible");
        Context.require(_data.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded");
        Context.require(_rollback == null || _rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded");

        boolean needResponse = _rollback != null;

        if (needResponse) {
            CallRequest req = new CallRequest(caller, _to, _rollback);
            requests.set(sn, req);
        }

        CSMessageRequest msgReq = new CSMessageRequest(caller.toString(), _to, sn, needResponse, _data);

        Packet pct = new Packet();
        pct.setSequence(getNextSn());
        pct.setData(msgReq.toString());
        pct.setDestinationPort(getDestinationPort());
        pct.setDestinationChannel(getDestinationChannel());
        pct.setSourcePort(getSourcePort());
        pct.setSourceChannel(getSourceChannel());

        Height hgt = new Height();
        hgt.setRevisionHeight(timeoutHeight.get());
        hgt.setRevisionNumber(BigInteger.ZERO);
        pct.setTimeoutHeight(hgt);

        pct.setTimeoutTimestamp(BigInteger.ZERO);

        IBCHandler ibc = new IBCHandler();
        ibc.sendPacket(pct);

        CallMessageSent(caller, _to, sn, sn);
        return sn;

    }

    @External(readonly = true)
    public Address getIBCAddress() {
        return ibcHandler.get();
    }

    @External
    public void executeCall(BigInteger _reqId) {
        CSMessageRequest req = proxyReqs.get(_reqId);
        Context.require(req != null, "InvalidRequestId");
        // cleanup
        proxyReqs.set(_reqId, null);

        CSMessageResponse msgRes = null;
        try {
            DAppProxy proxy = new DAppProxy(Address.fromString(req.getTo()));
            proxy.handleCallMessage(req.getFrom(), req.getData());
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
//            if (req.needRollback()) {
//                BigInteger sn = req.getSn().negate();
//                sendBTPMessage(BigInteger.ZERO, from.net(), CSMessage.RESPONSE, sn, msgRes.toBytes());
            }
        }
    }

    @External
    public void executeRollback(BigInteger _sn) {
        CallRequest req = requests.get(_sn);
        Context.require(req != null, "InvalidSerialNum");
        Context.require(req.enabled(), "RollbackNotEnabled");
        cleanupCallRequest(_sn);
        String caller = Context.getCaller().toString();

        CSMessageResponse msgRes = null;
        try {
            DAppProxy proxy = new DAppProxy(req.getFrom());
            proxy.handleCallMessage(caller, req.getRollback());
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

    @EventLog(indexed=3)
    public void CallMessage(String _from, String _to, BigInteger _sn, BigInteger _reqId) {}

    @EventLog(indexed=1)
    public void CallExecuted(BigInteger _reqId, int _code, String _msg) {}

    @EventLog(indexed=1)
    public void ResponseMessage(BigInteger _sn, int _code, String _msg) {}

    @EventLog(indexed=1)
    public void RollbackMessage(BigInteger _sn) {}

    @EventLog(indexed=1)
    public void RollbackExecuted(BigInteger _sn, int _code, String _msg) {}

    @EventLog(indexed=3)
    public void CallMessageSent(Address _from, String _to, BigInteger _sn, BigInteger _nsn) {}


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
        onlyIBCHandler();

        String errMsg = "IBCError{code=" + _code + ", msg=" + _msg + "}";
        CSMessageResponse res = new CSMessageResponse(_sn, CSMessageResponse.IBC_ERROR, errMsg);
        handleResponse(_src, _sn, res.toBytes());
    }
    /* ========================================= */

    private void handleRequest(String netFrom, BigInteger sn, byte[] data) {
        CSMessageRequest msgReq = CSMessageRequest.fromBytes(data);
        String to = msgReq.getTo();
        String caller = Context.getCaller().toString();

        BigInteger reqId = getNextReqId();
        CSMessageRequest req = new CSMessageRequest(caller, to, msgReq.getSn(), msgReq.needRollback(), msgReq.getData());
        proxyReqs.set(reqId, req);

        // emit event to notify the user
        CallMessage(caller, to, msgReq.getSn(), reqId);
    }

    private void handleResponse(String netFrom, BigInteger sn, byte[] data) {
        CSMessageResponse msgRes = CSMessageResponse.fromBytes(data);
        BigInteger resSn = msgRes.getSn();
        CallRequest req = requests.get(resSn);
        if (req == null) {
            Context.println("handleResponse: no request for " + resSn);
            return; // just ignore
        }
        String errMsg = msgRes.getMsg();
        ResponseMessage(resSn, msgRes.getCode(), errMsg != null ? errMsg : "");
        switch (msgRes.getCode()) {
            case CSMessageResponse.SUCCESS:
                cleanupCallRequest(resSn);
                break;
            case CSMessageResponse.FAILURE:
            case CSMessageResponse.IBC_ERROR:
            default:
                // emit rollback event
                Context.require(req.getRollback() != null, "NoRollbackData");
                req.setEnabled();
                requests.set(resSn, req);
                RollbackMessage(resSn);
        }
    }

    @External(readonly = true)
    public Address admin() {
        return admin.getOrDefault(Context.getOwner());
    }

    @External
    public void setAdmin(Address _address) {
        onlyOwner();
        admin.set(_address);
    }

    @External
    public void setProtocolFeeHandler(@Optional Address _addr) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        feeHandler.set(_addr);
        if (_addr != null) {
            var accruedFees = Context.getBalance(Context.getAddress());
            if (accruedFees.signum() > 0) {
                Context.transfer(_addr, accruedFees);
            }
        }
    }

    @External(readonly = true)
    public Address getProtocolFeeHandler() {
        return feeHandler.get();
    }

    @External
    public void setProtocolFee(BigInteger _value) {
        checkCallerOrThrow(admin(), "OnlyAdmin");
        Context.require(_value.signum() >= 0, "ValueShouldBePositive");
        protocolFee.set(_value);
    }

    @External(readonly = true)
    public BigInteger getProtocolFee() {
        return protocolFee.getOrDefault(BigInteger.ZERO);
    }

    @External
    public void onChanOpenInit(Channel.Order order, String[] connectionHops, String portId, String channelId, Counterparty counterparty, String version) {

    }

    @External
    public void onChanOpenTry(Channel.Order order, String[] connectionHops, String portId, String channelId, Counterparty counterparty, String version, String counterpartyVersion) {

    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyVersion) {

    }

    @External
    public void onChanOpenConfirm(String portId, String channelId) {

    }

    @External
    public void onChanCloseInit(String portId, String channelId) {
        Context.revert("CannotCloseChannel");

    }

    @External
    public void onChanCloseConfirm(String portId, String channelId) {
        Context.revert("CannotCloseChannel");

    }

    @External
    public byte[] onRecvPacket(Packet calldata, Address relayer) {
        return new byte[0];
    }

    @External
    public void onAcknowledgementPacket(Packet calldata, byte[] acknowledgement, Address relayer) {

    }

    private void setSourceChannelAndPort(String channel, String port) {
        sourcePort.set(port);
        sourceChannel.set(channel);
    }

    private void setDestinationChannelAndPort(String channel, String port) {
        destinationPort.set(port);
        destinationChannel.set(channel);

    }

    private String getDestinationPort() {
        return destinationPort.get();
    }

    private String getDestinationChannel() {
        return destinationChannel.get();
    }

    private String getSourcePort() {
        return sourcePort.get();
    }

    private String getSourceChannel() {
        return sourceChannel.get();
    }

    private byte[] _newAcknowledgement(boolean success) {
        byte[] acknowledgement = new byte[1];
        if (success) {
            acknowledgement[0] = 0x01;
        }
        return acknowledgement;
    }

    private boolean _isSuccessAcknowledgement(byte[] acknowledgement) {
        Context.require(acknowledgement.length == 1);
        return acknowledgement[0] == 0x01;
    }

//    @External(readonly=true)
//    public BigInteger getFee(String _net, boolean _rollback) {
//        if (_net.isEmpty() || _net.indexOf('/') != -1 || _net.indexOf(':') != -1) {
//            Context.revert("InvalidNetworkAddress");
//        }
//        var relayFee = bmc.getFee(_net, _rollback);
//        return getProtocolFee().add(relayFee);
//    }
}
