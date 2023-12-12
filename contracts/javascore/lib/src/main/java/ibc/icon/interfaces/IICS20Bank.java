package ibc.icon.interfaces;

import score.Address;

import java.math.BigInteger;

public interface IICS20Bank {
    void balanceOf(Address account, String denom);

    void transferFrom(Address from, Address to, String denom, BigInteger amount);

    void mint(Address account, String denom, BigInteger amount);

    void burn(Address account, String denom, BigInteger amount);

    void deposit(Address tokenContract, BigInteger amount, Address receiver);
    void withdraw(Address tokenContract, BigInteger amount, Address receiver);


}
