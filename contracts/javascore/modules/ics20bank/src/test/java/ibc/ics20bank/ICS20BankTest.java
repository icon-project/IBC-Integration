package ibc.ics20bank;


import com.iconloop.score.test.Account;
import com.iconloop.score.test.Score;
import com.iconloop.score.test.ServiceManager;
import com.iconloop.score.test.TestBase;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import score.Context;

import java.math.BigInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.CALLS_REAL_METHODS;
import static org.mockito.Mockito.spy;


public class ICS20BankTest extends TestBase {
    public static final ServiceManager sm = getServiceManager();
    public static final Account owner = sm.createAccount();
    public static final Account testingAccount = sm.createAccount();
    public static final Account testingAccount2 = sm.createAccount();
    public static final String TAG = "ICS20Bank";
    public Score ics20Bank;
    ICS20Bank ICS20BankSpy;

    public static MockedStatic<Context> contextMock;

    @BeforeEach
    public void setup() throws Exception {
        ics20Bank = sm.deploy(owner, ICS20Bank.class);

        ICS20Bank instance = (ICS20Bank) ics20Bank.getInstance();
        ICS20BankSpy = spy(instance);
        ics20Bank.setInstance(ICS20BankSpy);
        ics20Bank.invoke(owner, "setupRole", 1, owner.getAddress());
        ics20Bank.invoke(owner, "setupRole", 2, owner.getAddress());
        contextMock.reset();
    }

    @BeforeAll
    public static void init() {
        contextMock = Mockito.mockStatic(Context.class, CALLS_REAL_METHODS);
    }


    public void expectErrorMessage(Executable contractCall, String errorMessage) {
        AssertionError e = Assertions.assertThrows(AssertionError.class, contractCall);
        assertEquals(errorMessage, e.getMessage());
    }

    @Test
    void setupRole() {
        ics20Bank.invoke(owner, "setupRole", 2, testingAccount.getAddress());
        assertEquals(2, ics20Bank.call("getRole", testingAccount.getAddress()));
    }

    @Test
    void mint() {
        ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100));
        assertEquals(BigInteger.valueOf(100), ics20Bank.call("balanceOf", new Object[]{testingAccount.getAddress(), "testDenom"}));
    }

    @Test
    void mintNoAccess() {
        expectErrorMessage(() -> ics20Bank.invoke(testingAccount, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100)), "Reverted(0): ICS20Bank: must have minter role to mint");
    }

    @Test
    void mintZeroAmount() {
        expectErrorMessage(() -> ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(0)), "Reverted(0): ICS20Bank: mint amount must be greater than zero");
    }

    @Test
    void burn() {
        ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100));
        ics20Bank.invoke(owner, "burn", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(50));
        assertEquals(BigInteger.valueOf(50), ics20Bank.call("balanceOf", new Object[]{testingAccount.getAddress(), "testDenom"}));
    }

    @Test
    void burnNoAccess() {
        expectErrorMessage(() -> ics20Bank.invoke(testingAccount, "burn", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100)), "Reverted(0): ICS20Bank: must have burn role to burn");
    }

    @Test
    void burnGreaterAmount() {
        ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100));
        expectErrorMessage(() -> ics20Bank.invoke(owner, "burn", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(150)), "Reverted(0): ICS20Bank: burn amount exceeds balance");
    }

    @Test
    void burnZeroAmount() {
        expectErrorMessage(() -> ics20Bank.invoke(owner, "burn", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(0)), "Reverted(0): ICS20Bank: burn amount must be greater than zero");
    }

    @Test
    void transferFrom() {
        ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100));
        ics20Bank.invoke(owner, "transferFrom", testingAccount.getAddress(), owner.getAddress(), "testDenom", BigInteger.valueOf(50));
        assertEquals(BigInteger.valueOf(50), ics20Bank.call("balanceOf", testingAccount.getAddress(), "testDenom"));
        assertEquals(BigInteger.valueOf(50), ics20Bank.call("balanceOf", owner.getAddress(), "testDenom"));
    }

    @Test
    void transferFromSameAddress() {
        ics20Bank.invoke(owner, "mint", testingAccount.getAddress(), "testDenom", BigInteger.valueOf(100));
        expectErrorMessage(() -> ics20Bank.invoke(owner, "transferFrom", testingAccount.getAddress(), testingAccount.getAddress(), "testDenom", BigInteger.valueOf(50)), "Reverted(0): ICS20Bank: sender and receiver is same");
    }

    @Test
    void transferFromNoAccess() {
        expectErrorMessage(() -> ics20Bank.invoke(testingAccount, "transferFrom", owner.getAddress(), testingAccount2.getAddress(), "testDenom", BigInteger.valueOf(50)), "Reverted(0): ICS20Bank: caller is not owner nor approved");
    }

    public MockedStatic.Verification caller() {
        return () -> Context.getCaller();
    }


}