package ibc.ics20.app;

import ibc.ics24.host.IBCCommitment;
import score.Address;
import score.BranchDB;
import score.Context;
import score.DictDB;
import score.annotation.External;

import java.math.BigInteger;

public class ICS20Bank {

    private static final byte[] ADMIN_ROLE = IBCCommitment.keccak256("ADMIN_ROLE".getBytes());
    private static final byte[] OPERATOR_ROLE = IBCCommitment.keccak256("OPERATOR_ROLE".getBytes());

    private static final Integer ADMIN_ROLE_ID = 1;
    private static final Integer OPERATOR_ROLE_ID = 2;


    // Mapping from token ID to account balances
    private final BranchDB<String, DictDB<Address, BigInteger>> balances = Context.newBranchDB("BALANCES", BigInteger.class);
    private final DictDB<Address, Integer> roles = Context.newDictDB("ROLES", Integer.class);


    public ICS20Bank() {
        setupRole(ADMIN_ROLE_ID, Context.getCaller());
    }

    @External
    public void setupRole(Integer role, Address account) {
        Context.require(Context.getCaller().equals(Context.getOwner()), "Only owner can set up role");
        roles.set(account, role);
    }

    @External
    public void setupOperator(Address account) {
        setupRole(OPERATOR_ROLE_ID, account);
    }

    private boolean hasRole(Integer role, Address account) {
        return (role == roles.getOrDefault(account, 0));
    }

    @External(readonly = true)
    public BigInteger balanceOf(Address account, String denom) {
        Context.require(account != ICS20Transfer.ZERO_ADDRESS, "ICS20Bank: balance query for the zero address");

        // Assuming the denomination is a valid key in the balances mapping
        return balances.at(denom).getOrDefault(account, BigInteger.ZERO);
    }

    @External
    public void transferFrom(Address from, Address to, String denom, BigInteger amount) {
        Context.require(to != ICS20Transfer.ZERO_ADDRESS, "ICS20Bank: balance query for the zero address");
        Address caller = Context.getCaller();
        Context.require(from.equals(caller) || hasRole(OPERATOR_ROLE_ID, caller), "ICS20Bank: caller is not owner nor approved");
        BigInteger fromBalance = balanceOf(from, denom);
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, "ICS20Bank: transfer amount must be greater than zero");
        Context.require(fromBalance.compareTo(amount) >= 0, "ICS20Bank: insufficient balance for transfer");

        balances.at(denom).set(from, fromBalance.subtract(amount));
        balances.at(denom).set(to, balanceOf(to, denom).add(amount));
    }

    @External
    public void mint(Address account, String denom, BigInteger amount) {
        Context.require(hasRole(OPERATOR_ROLE_ID, Context.getCaller()), "ICS20Bank: must have minter role to mint");
        Context.require(account != ICS20Transfer.ZERO_ADDRESS, "ICS20Bank: mint to the zero address");
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, "ICS20Bank: mint amount must be greater than zero");
        _mint(account, denom, amount);
    }

    @External
    public void burn(Address account, String denom, BigInteger amount) {
        Context.require(hasRole(OPERATOR_ROLE_ID, Context.getCaller()), "ICS20Bank: must have minter role to mint");
        Context.require(amount.compareTo(BigInteger.ZERO) > 0, "ICS20Bank: mint amount must be greater than zero");
        _burn(account, denom, amount);
    }

    @External
    public void deposit(Address tokenContract, BigInteger amount, Address receiver){
        Context.require(tokenContract.isContract(), "ICS20Bank: tokenContract is not a contract");
        Context.call(tokenContract, "transferFrom", Context.getCaller(), Context.getAddress(), amount);
        _mint(receiver, tokenContract.toString(), amount);
    }

    @External
    public void withdraw(Address tokenContract, BigInteger amount, Address receiver){
        Context.require(tokenContract.isContract(), "ICS20Bank: tokenContract is not a contract");
        _burn(receiver, tokenContract.toString(), amount);
        Context.call(tokenContract, "transfer", receiver, amount);
    }

    private void _mint(Address account, String denom,  BigInteger amount){
        balances.at(denom).set(account, balanceOf(account, denom).add(amount));
    }

    private void _burn(Address account, String denom,  BigInteger amount){
        BigInteger accountBalance = balanceOf(account, denom);
        Context.require(accountBalance.compareTo(amount) >= 0, "ICS20Bank: burn amount exceeds balance");
        balances.at(denom).set(account, accountBalance.subtract(amount));
    }
}
