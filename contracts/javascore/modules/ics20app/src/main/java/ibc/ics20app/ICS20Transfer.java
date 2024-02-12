package ibc.ics20app;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.score.util.StringUtil;
import ibc.ics23.commitment.Ops;

import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import score.Address;
import score.Context;
import score.DictDB;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Arrays;

import static ibc.ics20app.ICS20TransferBank.bank;

public abstract class ICS20Transfer implements IIBCModule {
    public static final String ICS20_VERSION = "ics20-1";
    public static final Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");
    public static final DictDB<String, Address> channelEscrowAddresses = Context.newDictDB("channelEscrowAddresses", Address.class);
    protected final DictDB<String, String> destinationPort = Context.newDictDB("destinationPort", String.class);
    protected final DictDB<String, String> destinationChannel = Context.newDictDB("destinationChannel", String.class);

    @External(readonly = true)
    public Address getIBCAddress() {
        return ICS20TransferBank.ibcHandler.getOrDefault(ZERO_ADDRESS);
    }

    public void onlyIBC() {
        Context.require(Context.getCaller().equals(getIBCAddress()), "ICS20App: Caller is not IBC Contract");
    }

    @External(readonly = true)
    public String getDestinationPort(String channelId) {
        return destinationPort.get(channelId);
    }

    @External(readonly = true)
    public String getDestinationChannel(String channelId) {
        return destinationChannel.get(channelId);
    }

    @External
    public byte[] onRecvPacket(byte[] packet, Address relayer) {
        onlyIBC();
        Packet packetDb = Packet.decode(packet);
        ICS20Lib.PacketData data = ICS20Lib.unmarshalJSON(packetDb.getData());
        boolean success = _decodeReceiver(data.receiver);
        if (!success) {
            return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }
        Address receiver = Address.fromString(data.receiver);

        byte[] denomPrefix = getDenomPrefix(packetDb.getSourcePort(), packetDb.getSourceChannel());
        byte[] denom = data.denom.getBytes();

        if (denom.length >= denomPrefix.length && Ops.hasPrefix(denom, denomPrefix)) {
            byte[] unprefixedDenom = Arrays.copyOfRange(denom, denomPrefix.length, denom.length);
            String unprefixedDenomString = new String(unprefixedDenom);
            if (unprefixedDenomString.equals("icx")){
                success = _transferICX(receiver, data.amount);
            }
            else {
                success = _transferFrom(getEscrowAddress(packetDb.getDestinationChannel()), receiver, unprefixedDenomString, data.amount);
            }
        } else {
            if (ICS20Lib.isEscapeNeededString(denom)) {
                success = false;
            } else {
                denom = StringUtil.encodePacked(packetDb.getDestinationPort(), "/", packetDb.getDestinationChannel(), "/", data.denom);
                String denomText = new String(denom);
                success = _mint(receiver, denomText, data.amount);
            }
        }

        if (success) {
            return ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON;
        } else {
            return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }
    }


    @External
    public void onAcknowledgementPacket(byte[] packet, byte[] acknowledgement, Address relayer) {
        onlyIBC();
        Packet packetDb = Packet.decode(packet);
        if (!acknowledgement.equals(ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON)) {
            refundTokens(ICS20Lib.unmarshalJSON(packetDb.getData()), packetDb.getSourcePort(), packetDb.getSourceChannel());
        }
    }

    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
                               byte[] counterpartyPb, String version) {
        onlyIBC();
        Context.require(order == Channel.Order.ORDER_UNORDERED, "must be unordered");
        Context.require(version.equals(ICS20_VERSION), "version should be same with ICS20_VERSION");
        Channel.Counterparty counterparty = Channel.Counterparty.decode(counterpartyPb);
        destinationPort.set(channelId, counterparty.getPortId());
        channelEscrowAddresses.set(channelId, Context.getAddress());
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
                              byte[] counterpartyPb, String version, String counterPartyVersion) {
        onlyIBC();
        Context.require(order == Channel.Order.ORDER_UNORDERED, "must be unordered");
        Context.require(counterPartyVersion.equals(ICS20_VERSION), "version should be same with ICS20_VERSION");
        Channel.Counterparty counterparty = Channel.Counterparty.decode(counterpartyPb);
        destinationPort.set(channelId, counterparty.getPortId());
        destinationChannel.set(channelId, counterparty.getChannelId());
        channelEscrowAddresses.set(channelId, Context.getAddress());
    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId, String counterPartyVersion) {
        onlyIBC();
        Context.require(counterPartyVersion.equals(ICS20_VERSION), "version should be same with ICS20_VERSION");

    }

    @External
    public void onChanCloseInit(String portId, String channelId) {
        Context.revert("Not Allowed");
    }

    @External
    public void onTimeoutPacket(byte[] packet, Address relayer) {
        Packet packetDb = Packet.decode(packet);
        ICS20Lib.PacketData data = ICS20Lib.unmarshalJSON(packetDb.getData());
        refundTokens(data, packetDb.getSourcePort(), packetDb.getSourceChannel());
    }

    @External
    public void onChanCloseConfirm(String portId, String channelId) {
        onlyIBC();
        Context.println("onChanCloseConfirm");
    }

    @External
    public void onChanOpenConfirm(String portId, String channelId) {
        onlyIBC();
        Context.println("onChanOpenConfirm");
    }


    static Address getEscrowAddress(String sourceChannel) {
        Address escorw = channelEscrowAddresses.get(sourceChannel);
        Context.require(escorw != ZERO_ADDRESS);
        return escorw;
    }

    @External(readonly = true)
    public Address escrowAddress(String sourceChannel){
        return getEscrowAddress(sourceChannel);
    }

    private void refundTokens(ICS20Lib.PacketData data, String sourcePort, String sourceChannel) {
        byte[] denomPrefix = getDenomPrefix(sourcePort, sourceChannel);
        byte[] denom = data.denom.getBytes();

        if (denom.length >= denomPrefix.length && Ops.hasPrefix(denom, denomPrefix)) {
            Context.require(_mint(Address.fromString(data.sender), data.denom, data.amount), "ICS20: mint failed");
        } else {
            Context.require(_transferFrom(getEscrowAddress(sourceChannel), Address.fromString(data.sender), data.denom, data.amount), "ICS20: transfer failed");
        }
    }

    public static byte[] getDenomPrefix(String port, String channel) {
        return StringUtil.encodePacked(port, "/", channel, "/");
    }

    boolean _transferFrom(Address sender, Address receiver, String denom, BigInteger amount) {
        Context.call(bank.get(), "transferFrom", sender, receiver, denom, amount);
        return true;
    }

    private boolean _mint(Address account, String denom, BigInteger amount) {
        Context.call(bank.get(), "mint", account, denom, amount);
        return true;
    }

    boolean _burn(Address account, String denom, BigInteger amount) {
        Context.call(bank.get(), "burn", account, denom, amount);
        return true;
    }

    boolean _transferICX(Address receiver, BigInteger amount) {
        Context.require(Context.getBalance(Context.getAddress()).compareTo(amount) >= 0, "ICS20App: insufficient balance for transfer");
        Context.transfer(receiver, amount);
        return true;
    }

    /**
     * @dev _decodeReceiver decodes a hex string to an address.
     * `receiver` may be an invalid address format.
     */
    protected static boolean _decodeReceiver(String receiver) {
        boolean flag;
        try {
            Address.fromString(receiver);
            flag = true;
        } catch (Exception e) {
            flag = false;

        }
        return flag;
    }


}
