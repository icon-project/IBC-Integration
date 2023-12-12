package ibc.ics20.app;

import ibc.icon.interfaces.IIBCHandler;
import ibc.icon.interfaces.IICS20Bank;
import icon.proto.core.client.Height;
import score.Address;
import score.Context;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Arrays;

public class ICS20TransferBank {
    IIBCHandler ibcHandler;
    IICS20Bank bank;

    public ICS20TransferBank(IIBCHandler ibcHandler, IICS20Bank bank) {
        this.ibcHandler = ibcHandler;
        this.bank = bank;
    }

    @External
    public void sendTransfer(String denom, BigInteger amount, String receiver, String sourcePort, String sourceChannel, BigInteger timeoutHeight) {
        byte[] denomPrefix = ICS20Transfer.getDenomPrefix(sourcePort, sourceChannel);
        Address caller = Context.getCaller();
        if (!denom.startsWith(Arrays.toString(denomPrefix))) {
            Context.require(_transferFrom(caller, ICS20Transfer.getEscrowAddress(sourceChannel), denom, amount), "transfer failed");
        }else {
            Context.require(_burn(caller, denom, amount), "burn failed");
        }

        byte[] packetData = ICS20Lib.marshalJson(denom,amount, ICS20Transfer._encodeSender(caller),receiver);


//        ibcHandler.sendPacket(sourcePort, sourceChannel, Height., timeoutHeight);
    }

    private boolean _transferFrom(Address sender, Address receiver, String denom, BigInteger amount) {
        try {
            bank.transferFrom(sender, receiver, denom, amount);
        } catch (Exception e) {
            return false;
        }
        return false;
    }

    private boolean _mint(Address account, String denom, BigInteger amount) {
        try {
            bank.mint(account, denom, amount);
        } catch (Exception e) {
            return false;
        }
        return false;
    }

    private boolean _burn(Address account, String denom, BigInteger amount) {
        try {
            bank.burn(account, denom, amount);
        } catch (Exception e) {
            return false;
        }
        return false;
    }

}
