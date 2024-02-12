package ibc.ics20bank;

import score.*;
import score.annotation.External;

import java.math.BigInteger;

import scorex.util.HashMap;

import java.util.Map;

public class ICS20Bank {

    public static final String ICS20_VERSION = "ics20-1";
    public static final Address ZERO_ADDRESS = Address.fromString("hx0000000000000000000000000000000000000000");

    public static final String TAG = "ICS20Bank";

    private static final Integer ADMIN_ROLE_ID = 1;
    private static final Integer OPERATOR_ROLE_ID = 2;


    // Mapping from token ID to account balances
    private final BranchDB<String, DictDB<Address, BigInteger>> balances = Context.newBranchDB("BALANCES", BigInteger.class);
    private final DictDB<Address, Integer> roles = Context.newDictDB("ROLES", Integer.class);


    public ICS20Bank() {
        if (roles.get(Context.getOwner()) == null) {
            setupRole(ADMIN_ROLE_ID, Context.getOwner());
        }
    }

    @External
    public void setupRole(int role, Address account) {
        Context.require(Context.getCaller().equals(Context.getOwner()), "Only owner can set up role");
        roles.set(account, role);
    }

    @External
    public void setupOperator(Address account) {
        setupRole(OPERATOR_ROLE_ID, account);
    }

    private boolean hasRole(int role, Address account) {
        return (roles.getOrDefault(account, 0) == role);
    }

    @External(readonly = true)
    public int getRole(Address account) {
        return roles.getOrDefault(account, 0);
    }

    @External(readonly = true)
    public BigInteger balanceOf(Address account, String denom) {
        return balances.at(denom).getOrDefault(account, BigInteger.ZERO);
    }

    @External
    public void transferFrom(Address from, Address to, String denom, BigInteger amount) {
        Context.require(to != ZERO_ADDRESS, TAG + ": balance query for the zero address");
        Address caller = Context.getCaller();
        Context.require(from.equals(caller) || hasRole(OPERATOR_ROLE_ID, caller), TAG + ": caller is not owner nor approved");
        Context.require(!from.equals(to), TAG + ": sender and receiver is same");
        BigInteger fromBalance = balanceOf(from, denom);
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, TAG + ": transfer amount must be greater than zero");
        Context.require(fromBalance.compareTo(amount) >= 0, TAG + ": insufficient balance for transfer");

        balances.at(denom).set(from, fromBalance.subtract(amount));
        balances.at(denom).set(to, balanceOf(to, denom).add(amount));
    }

    @External
    public void mint(Address account, String denom, BigInteger amount) {
        Context.require(hasRole(OPERATOR_ROLE_ID, Context.getCaller()), TAG + ": must have minter role to mint");
        Context.require(account != ZERO_ADDRESS, TAG + ": mint to the zero address");
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, TAG + ": mint amount must be greater than zero");
        _mint(account, denom, amount);
    }

    @External
    public void burn(Address account, String denom, BigInteger amount) {
        Context.require(hasRole(OPERATOR_ROLE_ID, Context.getCaller()), TAG + ": must have burn role to burn");
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, TAG + ": burn amount must be greater than zero");
        _burn(account, denom, amount);
    }

    @External
    public void deposit(Address tokenContract, BigInteger amount, Address receiver) {
        Context.require(tokenContract.isContract(), TAG + ": tokenContract is not a contract");
        Context.call(tokenContract, "transferFrom", Context.getCaller(), Context.getAddress(), amount);
        _mint(receiver, tokenContract.toString(), amount);
    }

    @External
    public void withdraw(Address tokenContract, BigInteger amount) {
        Context.require(tokenContract.isContract(), TAG + ": tokenContract is not a contract");
        Address receiver = Context.getCaller();
        _burn(receiver, tokenContract.toString(), amount);
        Context.call(tokenContract, "transfer", receiver, amount);
    }

    private void _mint(Address account, String denom, BigInteger amount) {
        balances.at(denom).set(account, balanceOf(account, denom).add(amount));
    }

    private void _burn(Address account, String denom, BigInteger amount) {
        BigInteger accountBalance = balanceOf(account, denom);
        Context.require(accountBalance.compareTo(amount) >= 0, TAG + ": burn amount exceeds balance");
        BigInteger newBalance = accountBalance.subtract(amount);
        balances.at(denom).set(account, newBalance);
    }
}
