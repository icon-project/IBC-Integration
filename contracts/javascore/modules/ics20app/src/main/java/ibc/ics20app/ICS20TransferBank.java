package ibc.ics20app;

import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import score.Address;
import score.Context;
import score.VarDB;
import score.annotation.External;

import java.math.BigInteger;

public class ICS20TransferBank extends ICS20Transfer {
    public static final VarDB<Address> ibcHandler = Context.newVarDB("ibcHandler", Address.class);
    public static final VarDB<Address> bank = Context.newVarDB("bank", Address.class);

    public static final String TAG = "ICS20App";

    public ICS20TransferBank(Address _ibcHandler, Address _bank) {
        if (ibcHandler.get() == null) {
            ibcHandler.set(_ibcHandler);
            bank.set(_bank);
        }
    }

    @External(readonly = true)
    public Address getBank() {
        return bank.getOrDefault(ZERO_ADDRESS);
    }

    @External
    public void sendTransfer(String denom, BigInteger amount, String receiver, String sourcePort, String sourceChannel, BigInteger timeoutHeight, BigInteger timeoutRevisionNumber) {
        byte[] denomPrefix = ICS20Transfer.getDenomPrefix(sourcePort, sourceChannel);
        Address caller = Context.getCaller();

        String denomText = new String(denomPrefix);
        if (!denom.startsWith(denomText)) {
            Context.require(_transferFrom(caller, ICS20Transfer.getEscrowAddress(sourceChannel), denom, amount), "ICS20App: transfer failed");
        } else {
            Context.require(_burn(caller, denom, amount), "ICS20App: Burn failed");
        }

        Height height = new Height();
        height.setRevisionNumber(timeoutRevisionNumber);
        height.setRevisionHeight(timeoutHeight);

        byte[] data = ICS20Lib.marshalJson(denom, amount, caller.toString(), receiver);

        BigInteger seq = (BigInteger) Context.call(ibcHandler.get(), "getNextSequenceSend", sourcePort, sourceChannel);
        Packet newPacket = new Packet();
        newPacket.setSequence(seq);
        newPacket.setSourcePort(sourcePort);
        newPacket.setSourceChannel(sourceChannel);
        newPacket.setDestinationPort(destinationPort.get(sourceChannel));
        newPacket.setDestinationChannel(destinationChannel.get(sourceChannel));
        newPacket.setTimeoutHeight(height);
        newPacket.setTimeoutTimestamp(BigInteger.ZERO);
        newPacket.setData(data);

        Context.call(ibcHandler.get(), "sendPacket", newPacket.encode());
    }
}
