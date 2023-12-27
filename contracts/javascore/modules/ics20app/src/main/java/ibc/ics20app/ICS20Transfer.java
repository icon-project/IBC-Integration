package ibc.ics20app;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.score.util.StringUtil;
import ibc.ics23.commitment.Ops;
import ibc.ics24.host.IBCCommitment;

import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import score.Address;
import score.Context;
import score.DictDB;
import score.VarDB;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Arrays;

public abstract class ICS20Transfer implements IIBCModule{
    public static final String ICS20_VERSION = "ics20-1";
    public static final Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");
    public static final DictDB<String, Address> channelEscrowAddresses = Context.newDictDB("channelEscrowAddresses", Address.class);

    private static final VarDB<Address> IBC_ADDRESS = Context.newVarDB("IBC_ADDRESS", Address.class);

    @External
    public void setIBCAddress(Address ibcAddress) {
        Context.require(Context.getCaller().equals(Context.getOwner()), "Only owner can set up the address");
        IBC_ADDRESS.set(ibcAddress);
    }

    @External(readonly = true)
    public Address getIBCAddress() {
        return IBC_ADDRESS.getOrDefault(ZERO_ADDRESS);
    }

    public void onlyIBC() {
        Context.require(Context.getCaller().equals(getIBCAddress()), "ICS20App: Caller is not IBC Contract");
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
            success = _transferFrom(getEscrowAddress(packetDb.getDestinationChannel()), receiver, unprefixedDenom.toString(), data.amount);
        } else {
            if (ICS20Lib.isEscapeNeededString(denom)) {
                success = false;
            } else {
                success = _mint(receiver, StringUtil.encodePacked(getDenomPrefix(packetDb.getDestinationPort(), packetDb.getDestinationChannel()), denom).toString(), data.amount);
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
        if (acknowledgement != ICS20Lib.KECCAK256_SUCCESSFUL_ACKNOWLEDGEMENT_JSON) {
            refundTokens(ICS20Lib.unmarshalJSON(packet), packetDb.getSourcePort(), packetDb.getSourceChannel());
        }

    }

    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
                                 byte[] counterpartyPb, String version) {
        Context.require(order == Channel.Order.ORDER_UNORDERED, "must be unordered");
        byte[] versionBytes = version.getBytes();
        Context.require(versionBytes.length == 0 || IBCCommitment.keccak256(versionBytes) == IBCCommitment.keccak256(ICS20_VERSION.getBytes()), "version cannot be empty");
        channelEscrowAddresses.set(channelId, Context.getAddress());
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
                                byte[] counterpartyPb, String version, String counterPartyVersion) {
        Context.require(order == Channel.Order.ORDER_UNORDERED, "must be unordered");
        Context.require(IBCCommitment.keccak256(counterPartyVersion.getBytes()) == IBCCommitment.keccak256(ICS20_VERSION.getBytes()), "version should be same with ICS20_VERSION");
        channelEscrowAddresses.set(channelId, Context.getAddress());
    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId, String counterPartyVersion) {
        Context.require(IBCCommitment.keccak256(counterPartyVersion.getBytes()) == IBCCommitment.keccak256(ICS20_VERSION.getBytes()), "version should be same with ICS20_VERSION");

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
        Context.println("onChanCloseConfirm");
    }@External
    public void onChanOpenConfirm(String portId, String channelId) {
        Context.println("onChanCloseConfirm");
    }



    static Address getEscrowAddress(String sourceChannel) {
        Address escorw = channelEscrowAddresses.get(sourceChannel);
        Context.require(escorw != ZERO_ADDRESS);
        return escorw;
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

    private boolean _transferFrom(Address sender, Address receiver, String denom, BigInteger amount) {
        // Implementation goes here
        // Return true if minting is successful, false otherwise
        return true;
    }

    private boolean _mint(Address account, String denom, BigInteger amount) {
        // Implementation goes here
        // Return true if minting is successful, false otherwise
        return true;
    }

    /**
     * @dev _burn burns tokens from `account` in the bank.
     */
    private boolean _burn(Address account, String denom, BigInteger amount) {
        // Implementation goes here
        // Return true if burning is successful, false otherwise
        return true;
    }

    /**
     * @dev _encodeSender encodes an address to a hex string.
     * The encoded sender is used as `sender` field in the packet data.
     */
    protected static String _encodeSender(Address sender) {
        return ICS20Lib.addressToHexString(sender.toString());
    }


    protected Address _decodeSender(String sender) {
        boolean ok = _decodeReceiver(sender);
        Context.require(ok, "invalid address");
        return Address.fromString(sender);
    }

    /**
     * @dev _decodeReceiver decodes a hex string to an address.
     * `receiver` may be an invalid address format.
     */
    protected boolean _decodeReceiver(String receiver) {
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
