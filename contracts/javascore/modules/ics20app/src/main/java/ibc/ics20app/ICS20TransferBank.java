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

    public ICS20TransferBank(Address _ibcHandler, Address _bank) {
        if (ibcHandler.get() == null) {
            ibcHandler.set(_ibcHandler);
            bank.set(_bank);
        }
    }

    @External
    public void sendTransfer(String denom, BigInteger amount, String receiver, String sourcePort, String sourceChannel, BigInteger timeoutHeight, BigInteger timeoutRevisionNumber) {
        byte[] denomPrefix = ICS20Transfer.getDenomPrefix(sourcePort, sourceChannel);
        Address caller = Context.getCaller();
        if (!denom.startsWith(denomPrefix.toString())) {
            Context.require(_transferFrom(caller, ICS20Transfer.getEscrowAddress(sourceChannel), denom, amount), "transfer failed");
        } else {
            Context.require(_burn(caller, denom, amount), "burn failed");
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

    private boolean _transferFrom(Address sender, Address receiver, String denom, BigInteger amount) {
        try {
            Context.call(bank.get(), "transferFrom", sender, receiver, denom, amount);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    private boolean _mint(Address account, String denom, BigInteger amount) {
        try {
            Context.call(bank.get(), "mint", account, denom, amount);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    private boolean _burn(Address account, String denom, BigInteger amount) {
        try {
            Context.call(bank.get(), "burn", account, denom, amount);
            return true;
        } catch (Exception e) {
            return false;
        }
    }
}
