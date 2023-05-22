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

package ibc.xcall.connection;


import java.math.BigInteger;

import icon.proto.core.channel.Channel.Counterparty;
import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import score.Address;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.VarDB;
import score.annotation.External;
import score.annotation.Payable;

public class IBCConnection {
    public static final String PORT = "mock";
    protected final VarDB<Address> ibc = Context.newVarDB("ibcHandler", Address.class);
    protected final VarDB<Address> xCall = Context.newVarDB("callService", Address.class);
    protected final VarDB<String> networkId = Context.newVarDB("networkId", String.class);
    protected final VarDB<Address> admin = Context.newVarDB("admin", Address.class);

    protected final VarDB<BigInteger> timeoutHeight = Context.newVarDB("timeoutHeight", BigInteger.class);

    protected final DictDB<String, String> channels = Context.newDictDB("channels", String.class);
    protected final DictDB<String, String> counterPartyNetworkId = Context.newDictDB("counterPartyNetworkId", String.class);
    protected final DictDB<String, String> destinationChannel = Context.newDictDB("destinationChannel", String.class);

    protected final BranchDB<String, DictDB<BigInteger, BigInteger>> incomingPackets = Context.newBranchDB("incomingPackets", BigInteger.class);
    protected final BranchDB<String, DictDB<BigInteger, BigInteger>> outgoingPackets = Context.newBranchDB("outgoingPackets", BigInteger.class);

    public IBCConnection(Address _xCall, Address _ibc, String _nid, BigInteger _timeoutHeight) {
        ibc.set(_ibc);
        xCall.set(_xCall);
        networkId.set(_nid);
        admin.set(Context.getOwner());
        this.timeoutHeight.set(_timeoutHeight);
    }

    private void checkCallerOrThrow(Address caller, String errMsg) {
        Context.require(Context.getCaller().equals(caller), errMsg);
    }

    private void onlyIBCHandler() {
        checkCallerOrThrow(ibc.get(), "Only IBCHandler allowed");
    }


    private void onlyXCall() {
        checkCallerOrThrow(xCall.get(), "Only XCall allowed");
    }

    private void onlyAdmin() {
        checkCallerOrThrow(admin.get(), "Only Admin allowed");
    }

    @External
    public void transferAdmin(Address admin) {
        onlyAdmin();
        this.admin.set(admin);
    }

    @External
    public void configureChannel(String channelId, String counterpartyNid) {
        onlyAdmin();
        Context.require(channels.get(counterpartyNid) == null);
        Context.require(destinationChannel.get(channelId) != null);
        channels.set(counterpartyNid, channelId);
        counterPartyNetworkId.set(channelId, counterpartyNid);
    }

    @Payable
    @External
    public BigInteger sendMessage(String _to, String _svc, BigInteger _sn, byte[] _msg) {
        onlyXCall();
        if (_sn.compareTo(BigInteger.ZERO) < 0) {
           return writeAcknowledgement(_to, _sn.negate(), _msg);
        }

        // TODO fee logic
        String channel = channels.get(_to);
        String destinationChannel = this.destinationChannel.get(channel);
        BigInteger seqNum = (BigInteger) Context.call(ibc.get(), "getNextSequenceSend", PORT, channel);
        if (!_sn.equals(BigInteger.ZERO)) {
            outgoingPackets.at(channel).set(seqNum, _sn);
        }

        Height hgt = new Height();
        BigInteger timeoutHeight = BigInteger.valueOf(Context.getBlockHeight()).add(this.timeoutHeight.get());
        hgt.setRevisionHeight(timeoutHeight);
        //TODO use correct revision height
        hgt.setRevisionNumber(BigInteger.ZERO);

        Packet pct = new Packet();
        pct.setSequence(seqNum);
        pct.setData(new Message(_sn, _msg).toBytes());
        pct.setSourcePort(PORT);
        pct.setSourceChannel(channel);
        pct.setDestinationPort(PORT);
        pct.setDestinationChannel(destinationChannel);
        pct.setTimeoutHeight(hgt);
        pct.setTimeoutTimestamp(BigInteger.ZERO);

        Context.call(ibc.get(), "sendPacket", (Object)pct.encode());
        return BigInteger.ONE;
    }

    @External
    public byte[] onRecvPacket(byte[] calldata, Address relayer) {
        onlyIBCHandler();
        Packet packet = Packet.decode(calldata);
        Message msg = Message.fromBytes(packet.getData());
        String nid = counterPartyNetworkId.get(packet.getDestinationChannel());
        Context.require(nid != null);
        if (!msg.getSn().equals(BigInteger.ZERO)) {
            incomingPackets.at(packet.getDestinationChannel()).set(msg.getSn(), packet.getSequence());
        }

        Context.call(xCall.get(), "handleBTPMessage", nid, "xcall", msg.getSn(), msg.getData());
        return new byte[0];
    }

    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
        onlyIBCHandler();
        Packet packet = Packet.decode(calldata);
        BigInteger sn = outgoingPackets.at(packet.getSourceChannel()).get(packet.getSequence());
        outgoingPackets.at(packet.getSourceChannel()).set(packet.getSequence(), null);
        String nid = counterPartyNetworkId.get(packet.getSourceChannel());
        Context.require(nid != null);
        Context.require(sn != null);
        Context.call(xCall.get(), "handleBTPMessage", nid, "xcall", sn, acknowledgement);
    }

    @External
    public void onTimeoutPacket(byte[] calldata, Address relayer) {
        onlyIBCHandler();
        Packet packet = Packet.decode(calldata);
        BigInteger sn = outgoingPackets.at(packet.getSourceChannel()).get(packet.getSequence());
        outgoingPackets.at(packet.getSourceChannel()).set(packet.getSequence(), null);

        Context.require(sn != null);
        Context.call(xCall.get(), "handleBTPError", "", "xcall", sn, -1, "Timeout");
    }

    private BigInteger writeAcknowledgement(String _to, BigInteger _sn, byte[] _msg) {
        String channel = channels.get(_to);
        Packet pct = new Packet();
        pct.setSequence(incomingPackets.at(channel).get(_sn));
        incomingPackets.at(channel).set(_sn, null);

        pct.setDestinationPort(PORT);
        pct.setDestinationChannel(channel);

        Context.call(ibc.get(), "writeAcknowledgement", (Object)pct.encode(), _msg);
        return BigInteger.ONE;
    }

    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version) {
        onlyIBCHandler();
        // TODO verify order
        // TODO verify version

        Context.require(portId.equals(PORT));
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        Context.require(counterparty.getPortId().equals(PORT));
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
        byte[] counterpartyPb, String version, String counterpartyVersion) {
        onlyIBCHandler();

        // TODO verify order
        // TODO verify version

        Context.require(portId.equals(PORT));
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        Context.require(counterparty.getPortId().equals(PORT));
        destinationChannel.set(channelId, counterparty.getChannelId());
    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId,
            String counterpartyVersion) {
        onlyIBCHandler();
        destinationChannel.set(channelId, counterpartyChannelId);
    }

    @External
    public void onChanOpenConfirm(String portId, String channelId) {
        onlyIBCHandler();
        Context.require(portId.equals(PORT));
    }

    //TODO
    @External
    public void onChanCloseInit(String portId, String channelId) {
        onlyIBCHandler();
        Context.revert("CannotCloseChannel");
    }

    //TODO
    @External
    public void onChanCloseConfirm(String portId, String channelId) {
        onlyIBCHandler();
        Context.revert("CannotCloseChannel");
    }


    @External(readonly = true)
    public BigInteger getFee(String _to, boolean _response) {
        return BigInteger.ZERO;
    }

}