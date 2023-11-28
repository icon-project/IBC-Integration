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
import score.annotation.External;

import java.math.BigInteger;

public class ICS20Transfer {
    public static String ICS20_VERSION = "ics20-1";
    public static Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");


    public static final DictDB<String, Address> channelEscrowAddresses = Context.newDictDB("channelEscrowAddresses", Address.class);


//    @External
//    public byte[] onRecvPacket(byte[] packet, Address relayer) {
//        boolean success = false;
//        Address receiver = null;
//        receiver, success = _decodeReceiver(packet.data.receiver)
//
//    }
//
//     _decodeReceiver(string memory receiver) internal pure virtual returns (address, bool) {
//        return ICS20Lib.hexStringToAddress(receiver);
//    }

    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {

    }

    @External
    public String onChanOpenInit(IIBCModule.onChanOpenInit msg) {
//        srcChan.set(channelId);
//        srcPort.set(portId);
//        Channel.Counterparty counterparty = Channel.Counterparty.decode(counterpartyPb);
//        dstPort.set(counterparty.getPortId());
//        Context.println("onChanOpenInit");
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
    }


    private Address getEscrowAddress(String sourceChannel) {
        Address escorw = channelEscrowAddresses.get(sourceChannel);
        Context.require(escorw != ZERO_ADDRESS);
        return escorw;
    }

    private void refundTokens(ICS20Lib.PacketData data, String sourcePort, String sourceChannel){
        byte[] denomPrefix = getDenomPrefix(sourcePort,sourceChannel);
        byte[] denom = data.denom.getBytes();

        if (denom.length>= denomPrefix.length){
            Context.println("if");
        }else {
            Context.println("else");
        }



    }
    private byte[] getDenomPrefix(String port, String channel){
        return  StringUtil.encodePacked(port,"/",channel,"/");
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
     *      The encoded sender is used as `sender` field in the packet data.
     */
    protected String _encodeSender(Address sender) {
//        return ICS20Lib.addressToHexString(sender);
        return "Something";
    }


    protected Address _decodeSender(String sender) {
//        Map<Address, Boolean> result = ICS20Lib.hexStringToAddress(sender);
//        require(result.getSecond(), "invalid address");
//        return result.getFirst();
        return(ZERO_ADDRESS);
    }

    /**
     * @dev _decodeReceiver decodes a hex string to an address.
     *      `receiver` may be an invalid address format.
     */
    protected Map<Address, Boolean> _decodeReceiver(String receiver) {
//        return ICS20Lib.hexStringToAddress(receiver);
    }




}
