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

import foundation.icon.btp.xcall.data.CSMessage;
import foundation.icon.btp.xcall.data.CSMessageRequest;
import foundation.icon.btp.xcall.data.CSMessageResponse;
import foundation.icon.btp.xcall.data.CallRequest;
import ibc.icon.interfaces.ICallServiceReceiver;

import ibc.icon.interfaces.ICallServiceReceiverScoreInterface;
import icon.proto.core.channel.Channel.Counterparty;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import java.math.BigInteger;
import score.Address;
import score.Context;
import score.RevertedException;
import score.UserRevertException;
import score.UserRevertedException;
import score.annotation.EventLog;
import score.annotation.External;
import score.annotation.Optional;
import score.annotation.Payable;


public class CallServiceImpl extends AbstractCallService {

    public CallServiceImpl(Address _ibc, BigInteger _timeoutHeight) {
        this.ibcHandler.set(_ibc);
        admin.set(Context.getOwner());
        this.timeoutHeight.set(_timeoutHeight);
    }

    @External
    public void setTimeoutHeight(BigInteger _timeoutHeight) {
        onlyAdmin();
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

    private void onlyAdmin() {
        checkCallerOrThrow(this.admin(), "Only admin is allowed to call method");
    }

    private void onlyIBCHandler() {
        checkCallerOrThrow(ibcHandler.get(), "Only IBCHandler allowed");
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


    @Payable
    @External
    public BigInteger sendCallMessage(String _to, byte[] _data, @Optional byte[] _rollback) {
        BigInteger sn = getNextSn();
        Address caller = Context.getCaller();
        Context.require(caller.isContract() || _rollback == null, "RollbackNotPossible");
        Context.require(_data.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded");
        Context.require(_rollback == null || _rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded");

        boolean needResponse = _rollback != null && _rollback.length > 0;

        if (needResponse) {
            CallRequest req = new CallRequest(caller, _to, _rollback);
            requests.set(sn, req);
        }

        CSMessageRequest msgReq = new CSMessageRequest(caller.toString(), _to, sn, needResponse, _data);
        BigInteger seqNum = (BigInteger) Context.call(this.ibcHandler.get(), "getNextSequenceSend", getSourcePort(), getSourceChannel());

        Packet pct = new Packet();
        pct.setSequence(seqNum);
        pct.setData(createMessage(CSMessage.REQUEST, msgReq.toBytes()));
        pct.setDestinationPort(getDestinationPort());
        pct.setDestinationChannel(getDestinationChannel());
        pct.setSourcePort(getSourcePort());
        pct.setSourceChannel(getSourceChannel());

        BigInteger _timoutHeight = this.timeoutHeight.get();
        Height hgt = new Height();
        BigInteger timeoutHeight = BigInteger.valueOf(Context.getBlockHeight()).add(_timoutHeight);
        hgt.setRevisionHeight(timeoutHeight);

        pct.setTimeoutHeight(hgt);
        pct.setTimeoutTimestamp(BigInteger.ZERO);

        Context.call(this.ibcHandler.get(), "sendPacket", new Object[]{pct.encode()});

        CallMessageSent(caller, _to, sn, seqNum);
        return sn;

    }

    @External(readonly = true)
    public Address getIBCHandler() {
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
            ICallServiceReceiver proxy = new ICallServiceReceiverScoreInterface(Address.fromString(req.getTo()));
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
//            }
        }
    }

    @External
    public void executeRollback(BigInteger _sn) {
        CallRequest req = requests.get(_sn);
        Context.require(req != null, "InvalidSerialNum");
        Context.require(req.isEnabled(), "RollbackNotEnabled");
        cleanupCallRequest(_sn);
        String caller = Context.getCaller().toString();

        CSMessageResponse msgRes = null;
        try {
            ICallServiceReceiver proxy = new ICallServiceReceiverScoreInterface(req.getFrom());
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

    @EventLog(indexed = 3)
    public void CallMessage(String _from, String _to, BigInteger _sn, BigInteger _reqId) {
    }

    @EventLog(indexed = 1)
    public void CallExecuted(BigInteger _reqId, int _code, String _msg) {
    }

    @EventLog(indexed = 1)
    public void ResponseMessage(BigInteger _sn, int _code, String _msg) {
    }

    @EventLog(indexed = 1)
    public void RollbackMessage(BigInteger _sn) {
    }

    @EventLog(indexed = 1)
    public void RollbackExecuted(BigInteger _sn, int _code, String _msg) {
    }

    @EventLog(indexed = 3)
    public void CallMessageSent(Address _from, String _to, BigInteger _sn, BigInteger _nsn) {
    }


    /* ========== Interfaces with BMC ========== */
    @External
    public void handleBTPMessage(String _from, String _svc, BigInteger _sn, byte[] _msg) {
        onlyIBCHandler();
        handleReceivedPacket(_from, _svc, _sn, _msg);
    }

    private void handleReceivedPacket(String _from, String _svc, BigInteger _sn, byte[] _msg) {
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

        BigInteger reqId = getNextReqId();
        CSMessageRequest req = new CSMessageRequest(netFrom, to, msgReq.getSn(), msgReq.needRollback(),
                msgReq.getData());
        proxyReqs.set(reqId, req);

        // emit event to notify the user
        CallMessage(netFrom, to, msgReq.getSn(), reqId);
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
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version) {
        onlyIBCHandler();
        sourcePort.set(portId);
        sourceChannel.set(channelId);
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        destinationChannel.set(counterparty.getChannelId());
        destinationPort.set(counterparty.getPortId());
        Context.println("onChanOpenInit");
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version, String counterpartyVersion) {
        onlyIBCHandler();
        sourcePort.set(portId);
        sourceChannel.set(channelId);
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        destinationChannel.set(counterparty.getChannelId());
        destinationPort.set(counterparty.getPortId());
        Context.println("onChanOpenTry");
    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId,
            String counterpartyVersion) {
        onlyIBCHandler();
        Context.require(portId.equals(sourcePort.get()), "port not matched");
        Context.require(channelId.equals(sourceChannel.get()), "Channel not matched");
        destinationChannel.set(counterpartyChannelId);
        Context.println("onChanOpenAck");
    }

    @External
    public void onChanOpenConfirm(String portId, String channelId) {
        onlyIBCHandler();
        Context.require(portId.equals(sourcePort.get()), "port not matched");
        Context.require(channelId.equals(sourceChannel.get()), "Channel not matched");
        Context.println("onChanOpenConfirm");
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
    public byte[] onRecvPacket(byte[] calldata, Address relayer) {
        onlyIBCHandler();

        BigInteger newRecvCount = recvCount.getOrDefault(BigInteger.ZERO).add(BigInteger.ONE);
        recvCount.set(newRecvCount);

        Packet packet = Packet.decode(calldata);
        byte[] _msg = packet.getData();
        String _from = packet.getSourcePort() + "/" + packet.getSourceChannel();

        Context.println(packet.getSourcePort()+"-->"+packet.getSourceChannel());
        Context.println(destinationPort.get()+"<--"+destinationChannel.get());

        Context.require(packet.getSourcePort().equals(destinationPort.get()),"source port not matched");
        Context.require(packet.getSourceChannel().equals(destinationChannel.get()),"source channel not matched");

        BigInteger _sn = packet.getSequence();

        handleReceivedPacket(_from, _from, _sn, _msg);

        return new byte[0];
    }


    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
        Context.println("onAcknowledgementPacket");
    }

    @Override
    public void onTimeoutPacket(byte[] calldata, Address relayer) {
        throw new UserRevertException("Method not implemented");
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