package ibc.ics20app;

import icon.proto.core.client.Height;
import score.Address;
import score.Context;
import score.VarDB;
import score.annotation.External;

import java.math.BigInteger;
import java.util.Arrays;

public class ICS20TransferBank extends ICS20Transfer {
    private final VarDB<Address> ibcHandler = Context.newVarDB("ibcHandler_ADDRESS", Address.class);
    private final VarDB<Address> bank = Context.newVarDB("bank_ADDRESS", Address.class);


    public ICS20TransferBank(Address ibcHandler, Address bank) {
        this.ibcHandler.set(ibcHandler);
        this.bank.set(bank);
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
        Height height = new Height();
        height.setRevisionNumber(BigInteger.ZERO);
        height.setRevisionHeight(timeoutHeight);
        Context.call(ibcHandler.get(), "sendPacket", sourcePort, sourceChannel, height, 0, packetData);

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
