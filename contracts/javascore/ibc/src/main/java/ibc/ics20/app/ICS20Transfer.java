package ibc.ics20.app;

import ibc.icon.interfaces.IIBCModule;
import ibc.icon.score.util.Logger;
import ibc.icon.score.util.StringUtil;
import ibc.ics05.port.ModuleManager;
import ibc.ics25.handler.IBCHandler;

import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import score.Address;
import score.Context;
import score.DictDB;
import score.VarDB;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Map;

public class ICS20Transfer {
    public static String ICS20_VERSION = "ics20-1";
    public static Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");


    public static final DictDB<String, Address> channelEscrowAddresses = Context.newDictDB("channelEscrowAddresses", Address.class);

    private static final VarDB<Address> IBC_ADDRESS = Context.newVarDB("IBC_ADDRESS", Address.class);

    public ICS20Transfer(Address ibcAddress) {
        Context.println("ICS20Transfer");
        IBC_ADDRESS.set(ibcAddress);
    }

    @External
    public void setIBCAddress(Address ibcAddress) {
        Context.require(Context.getCaller().equals(Context.getOwner()), "Only owner can set up the address");
        IBC_ADDRESS.set(ibcAddress);
    }

    @External(readonly = true)
    public Address getIBCAddress() {
        return IBC_ADDRESS.get();
    }


    @External
    public byte[] onRecvPacket(byte[] packet, Address relayer) {
//        TODO unmarshal json
        boolean success;
        Address receiver = ZERO_ADDRESS;
        receiver, success = _decodeReceiver(packet.data.receiver);
        if (!success) {
           return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }

        byte[] denomPrefix = getDenomPrefix(packet.data.sourcePort, packet.data.sourceChannel);
        byte[] denom = packet.data.denom.getBytes();
        if (denom.length >= denomPrefix.length) {
//            denom slicing todo
            success = _transferFrom(getEscrowAddress(packet.data.sourceChannel), receiver, packet.data.denom, packet.data.amount);
        } else{
            if(ICS20Lib.isEscapeNeededString(denom)){
                success = false;
            }else{
                success = _mint(receiver, String(getDenomPrefix(packet.destination_port, packet.destination_channel), denom), packet.data.amount);
            }
        }

        if (success) {
            return ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON;
        }else{
            return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }


        return packet;
    }


    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
        Context.println("onAcknowledgementPacket");
        Context.require(Context.getCaller().equals(getIBCAddress()), "caller is not handler");
        if (acknowledgement != ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON) {
                refundTokens(calldata, packet.source_port, packet.source_channel);
        }

    }

    @External
    public String onChanOpenInit(IIBCModule.onChanOpenInit msg) {
        return ICS20_VERSION;
    }

    @External
    public String onChanOpenTry(IIBCModule.onChanOpenTry msg) {
        return ICS20_VERSION;
    }

    @External
    public void onChanOpenAck(IIBCModule.MsgOnChanOpenAck msg) {

    }

    @External
    public void onChanCloseInit(IIBCModule.MsgOnChanCloseInit msg) {
        Context.revert("Not Allowed");
    }

    @External
    public void onTimeoutPacket(Packet packet, byte[] proofHeight, byte[] proof, BigInteger nextSequenceRecv) {
        refundTokens(packet.data, packet.source_port, packet.source_channel);
    }


    static Address getEscrowAddress(String sourceChannel) {
        Address escorw = channelEscrowAddresses.get(sourceChannel);
        Context.require(escorw != ZERO_ADDRESS);
        return escorw;
    }

    private void refundTokens(ICS20Lib.PacketData data, String sourcePort, String sourceChannel) {
        byte[] denomPrefix = getDenomPrefix(sourcePort, sourceChannel);
        byte[] denom = data.denom.getBytes();

        if (denom.length >= denomPrefix.length) {

            Context.println("if");
        } else {
            Context.println("else");
        }


    }

    public static byte[] getDenomPrefix(String port, String channel) {
        return StringUtil.encodePacked(port, "/", channel, "/");
    }

    protected boolean _transferFrom(Address sender, Address receiver, String denom, BigInteger amount) {
        // Implementation goes here
        // Return true if minting is successful, false otherwise
        return true;
    }

    protected boolean _mint(Address account, String denom, BigInteger amount) {
        // Implementation goes here
        // Return true if minting is successful, false otherwise
        return true;
    }

    /**
     * @dev _burn burns tokens from `account` in the bank.
     */
    protected boolean _burn(Address account, String denom, BigInteger amount) {
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
//        Map<Address, Boolean> result = ICS20Lib.hexStringToAddress(sender);
//        require(result.getSecond(), "invalid address");
//        return result.getFirst();
        return (ZERO_ADDRESS);
    }

    /**
     * @dev _decodeReceiver decodes a hex string to an address.
     * `receiver` may be an invalid address format.
     */
    protected Map<Address, Boolean> _decodeReceiver(String receiver) {
        boolean flag;
        try {
            Address.fromString(receiver);
            flag = true;
        } catch (Exception e) {
            flag = false;

        }
        return Map.of(Address.fromString(receiver), flag);
    }


}
