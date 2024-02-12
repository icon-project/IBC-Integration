package ibc.icon.interfaces;

import foundation.icon.score.client.ScoreInterface;
import score.Address;

import java.math.BigInteger;

@ScoreInterface
public interface IICS20Bank {
    /**
     * balanceOf returns the balance of the given account on the specified denom.
     */
    void balanceOf(Address account, String denom);

    /**
     * transferFrom transfers amount of denom from sender to recipient.
     */
    void transferFrom(Address from, Address to, String denom, BigInteger amount);

    /**
     * mint creates amount of denom and adds it to the balance of the given account.
     */
    void mint(Address account, String denom, BigInteger amount);

    /**
     * burn subtracts amount of denom from the balance of the given account.
     */
    void burn(Address account, String denom, BigInteger amount);

    /**
     * deposit transfers amount of tokenContract from caller to the bank and mints the same amount of tokenContract to the caller.
     */
    void deposit(Address tokenContract, BigInteger amount, Address receiver);

    /**
     * withdraw transfers amount of tokenContract from the bank to receiver and burns the same amount of tokenContract from the bank.
     */
    void withdraw(Address tokenContract, BigInteger amount, Address receiver);


}
