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
import icon.proto.core.channel.Channel.Order;
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
    public static String PORT = "mock";
    protected final VarDB<Address> ibc = Context.newVarDB("ibcHandler", Address.class);
    protected final VarDB<Address> xCall = Context.newVarDB("callService", Address.class);
    protected final VarDB<Address> admin = Context.newVarDB("admin", Address.class);

    protected final BranchDB<String, DictDB<String, String>> configuredNetworkIds =  Context.newBranchDB("configuredNetworkIds", String.class);
    protected final DictDB<String, String> configuredClients = Context.newDictDB("configuredClients", String.class);
    protected final DictDB<String, BigInteger> configuredTimeoutHeight = Context.newDictDB("configuredTimeoutHeight", BigInteger.class);

    protected final DictDB<String, String> lightClients = Context.newDictDB("lightClients", String.class);
    protected final DictDB<String, BigInteger> timeoutHeights = Context.newDictDB("timeoutHeights", BigInteger.class);

    protected final DictDB<String, String> channels = Context.newDictDB("channels", String.class);
    protected final DictDB<String, String> networkIds = Context.newDictDB("counterPartyNetworkId", String.class);
    protected final DictDB<String, String> destinationPort = Context.newDictDB("destinationChannel", String.class);
    protected final DictDB<String, String> destinationChannel = Context.newDictDB("destinationPort", String.class);

    protected final BranchDB<String, DictDB<BigInteger, byte[]>> incomingPackets = Context.newBranchDB("incomingPackets", byte[].class);
    protected final BranchDB<String, DictDB<BigInteger, BigInteger>> outgoingPackets = Context.newBranchDB("outgoingPackets", BigInteger.class);

    protected final DictDB<String, BigInteger> sendPacketFee = Context.newDictDB("sendPacketFee", BigInteger.class);
    protected final DictDB<String, BigInteger> ackFee = Context.newDictDB("ackFee", BigInteger.class);

    protected final BranchDB<String, DictDB<BigInteger, BigInteger>> unclaimedAckFees = Context.newBranchDB("unclaimedAckFees", BigInteger.class);
    protected final BranchDB<String, DictDB<Address, BigInteger>> unclaimedPacketFees = Context.newBranchDB("unclaimedPacketFees", BigInteger.class);

    public IBCConnection(Address _xCall, Address _ibc, String port) {
        ibc.set(_ibc);
        xCall.set(_xCall);
        admin.set(Context.getCaller());
        PORT = port;
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

    public BigInteger getValue() {
        return Context.getValue();
    }

    @External
    public void transferAdmin(Address admin) {
        onlyAdmin();
        this.admin.set(admin);
    }

    @External
    public void setFee(String nid, BigInteger packetFee, BigInteger ackFee) {
        onlyAdmin();
        sendPacketFee.set(nid, packetFee);
        this.ackFee.set(nid, ackFee);
    }

    @External
    public void configureConnection(String connectionId, String counterpartyPortId, String counterpartyNid, String clientId, BigInteger timeoutHeight) {
        onlyAdmin();
        Context.require(configuredNetworkIds.at(connectionId).get(counterpartyPortId) == null);
        Context.require(channels.get(counterpartyNid) == null);
        configuredNetworkIds.at(connectionId).set(counterpartyPortId, counterpartyNid);
        configuredClients.set(connectionId, clientId);
        configuredTimeoutHeight.set(connectionId, timeoutHeight);
    }

    @Payable
    @External
    public void sendMessage(String _to, String _svc, BigInteger _sn, byte[] _msg) {
        onlyXCall();
        String channel = channels.get(_to);
        if (_sn.compareTo(BigInteger.ZERO) < 0) {
           writeAcknowledgement(_to, _sn.negate(), _msg);
           return;
        }

        BigInteger seqNum = (BigInteger) Context.call(ibc.get(), "getNextSequenceSend", PORT, channel);
        BigInteger packetFee = sendPacketFee.getOrDefault(_to, BigInteger.ZERO);
        BigInteger _ackFee = BigInteger.ZERO;
        if (_sn.compareTo(BigInteger.ZERO) > 0 ) {
            _ackFee = ackFee.getOrDefault(_to, BigInteger.ZERO);
            unclaimedAckFees.at(_to).set(seqNum, _ackFee);
        }

        Context.require(packetFee.add(_ackFee).compareTo(getValue()) >= 0, "Fee is not sufficient");
        if (!_sn.equals(BigInteger.ZERO)) {
            outgoingPackets.at(channel).set(seqNum, _sn);
        }

        Message msg = new Message(_sn, packetFee, _msg);
        Packet pct = new Packet();
        pct.setSequence(seqNum);
        pct.setData(msg.toBytes());
        pct.setSourcePort(PORT);
        pct.setSourceChannel(channel);
        pct.setDestinationPort(destinationPort.get(channel));
        pct.setDestinationChannel(destinationChannel.get(channel));
        pct.setTimeoutHeight(getTimeoutHeight(channel));
        pct.setTimeoutTimestamp(BigInteger.ZERO);

        Context.call(ibc.get(), "sendPacket", (Object)pct.encode());
    }

    @External
    public byte[] onRecvPacket(byte[] calldata, Address relayer) {
        onlyIBCHandler();

        Packet packet = Packet.decode(calldata);
        Message msg = Message.fromBytes(packet.getData());
        String nid = networkIds.get(packet.getDestinationChannel());
        Context.require(nid != null);

        BigInteger unclaimedFees = unclaimedPacketFees.at(nid).getOrDefault(relayer, BigInteger.ZERO);
        unclaimedPacketFees.at(nid).set(relayer, unclaimedFees.add(msg.getFee()));

        if (msg.getSn() == null)  {
            Context.transfer(new Address(msg.getData()), msg.getFee());
            return new byte[0];
        }

        if (msg.getSn().compareTo(BigInteger.ZERO) > 0) {
            incomingPackets.at(packet.getDestinationChannel()).set(msg.getSn(), calldata);
        }



        Context.call(xCall.get(), "handleMessage", nid, msg.getSn(), msg.getData());
        return new byte[0];
    }

    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
        onlyIBCHandler();

        Packet packet = Packet.decode(calldata);
        BigInteger sn = outgoingPackets.at(packet.getSourceChannel()).get(packet.getSequence());
        outgoingPackets.at(packet.getSourceChannel()).set(packet.getSequence(), null);
        String nid = networkIds.get(packet.getSourceChannel());

        Context.require(nid != null);
        Context.require(sn != null);

        Context.transfer(relayer, unclaimedAckFees.at(nid).get(packet.getSequence()));
        unclaimedAckFees.at(nid).set(packet.getSequence(), null);

        Context.call(xCall.get(), "handleMessage", nid, sn, acknowledgement);
    }

    @External
    public void onTimeoutPacket(byte[] calldata, Address relayer) {
        onlyIBCHandler();
        Packet packet = Packet.decode(calldata);
        Message msg = Message.fromBytes(packet.getData());
        BigInteger sn = outgoingPackets.at(packet.getSourceChannel()).get(packet.getSequence());
        outgoingPackets.at(packet.getSourceChannel()).set(packet.getSequence(), null);
        String nid = networkIds.get(packet.getSourceChannel());

        Context.require(sn != null);

        BigInteger fee = msg.getFee();
        fee = fee.add(unclaimedAckFees.at(nid).get(packet.getSequence()));
        unclaimedAckFees.at(nid).set(packet.getSequence(), null);

        Context.call(xCall.get(), "handleError", sn, -1, "Timeout");
        Context.transfer(relayer, fee);
    }

    private void writeAcknowledgement(String _to, BigInteger _sn, byte[] _msg) {
        String channel = channels.get(_to);
        byte[] packet = incomingPackets.at(channel).get(_sn);
        incomingPackets.at(channel).set(_sn, null);
        Context.call(ibc.get(), "writeAcknowledgement", (Object)packet, _msg);
    }

    private Height getTimeoutHeight(String channelId) {
        byte[] heightBytes = Context.call(byte[].class, ibc.get(), "getLatestHeight", lightClients.get(channelId));
        Height height = Height.decode(heightBytes);
        height.setRevisionHeight(height.getRevisionHeight().add(timeoutHeights.get(channelId)));
        return height;
    }

    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version) {
        onlyIBCHandler();

        Context.require(order == Order.ORDER_UNORDERED, "Channel order has to be unordered");
        // TODO verify version

        String connectionId = connectionHops[0];
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        String counterpartyPortId = counterparty.getPortId();
        String counterPartyNid = configuredNetworkIds.at(connectionId).get(counterpartyPortId);
        Context.require(portId.equals(PORT), "Invalid port");
        Context.require(channels.get(counterPartyNid) == null, "Network id is already configured");

        lightClients.set(channelId, configuredClients.get(connectionId));
        destinationPort.set(channelId, counterpartyPortId);
        networkIds.set(channelId, counterPartyNid);
        timeoutHeights.set(channelId, configuredTimeoutHeight.get(connectionId));
        channels.set(counterPartyNid, channelId);
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
        byte[] counterpartyPb, String version, String counterpartyVersion) {
        onlyIBCHandler();

        Context.require(order == Order.ORDER_UNORDERED, "Channel order has to be unordered");
        // TODO verify version

        String connectionId = connectionHops[0];
        Counterparty counterparty = Counterparty.decode(counterpartyPb);
        String counterpartyPortId = counterparty.getPortId();
        String counterPartyNid = configuredNetworkIds.at(connectionId).get(counterpartyPortId);
        Context.require(portId.equals(PORT), "Invalid port");
        Context.require(channels.get(counterPartyNid) == null, "Network id is already configured");
        lightClients.set(channelId, configuredClients.get(connectionId));
        destinationPort.set(channelId, counterpartyPortId);
        destinationChannel.set(channelId, counterparty.getChannelId());
        channels.set(counterPartyNid, channelId);
        networkIds.set(channelId, counterPartyNid);
        timeoutHeights.set(channelId, configuredTimeoutHeight.get(connectionId));
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
        BigInteger fee =sendPacketFee.getOrDefault(_to, BigInteger.ZERO);
        if (_response) {
            fee = fee.add(ackFee.getOrDefault(_to, BigInteger.ZERO));
        }

        return fee;
    }

    @External
    public void claimFees(String nid, byte[] address) {
        BigInteger amount = getUnclaimedFees(nid, Context.getCaller());
        unclaimedPacketFees.at(nid).set(Context.getCaller(), null);
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, "No fees available");
        String channel = channels.get(nid);
        BigInteger seqNum = (BigInteger) Context.call(ibc.get(), "getNextSequenceSend", PORT, channel);

        Message msg = new Message(null, amount, address);
        Packet pct = new Packet();

        pct.setSequence(seqNum);
        pct.setData(msg.toBytes());
        pct.setSourcePort(PORT);
        pct.setSourceChannel(channel);
        pct.setDestinationPort(destinationPort.get(channel));
        pct.setDestinationChannel(destinationChannel.get(channel));
        pct.setTimeoutHeight(getTimeoutHeight(channel));
        pct.setTimeoutTimestamp(BigInteger.ZERO);

        Context.call(ibc.get(), "sendPacket", (Object)pct.encode());
    }

    @External(readonly = true)
    public BigInteger getUnclaimedFees(String nid, Address relayer) {
        return unclaimedPacketFees.at(nid).getOrDefault(relayer, BigInteger.ZERO);
    }

}