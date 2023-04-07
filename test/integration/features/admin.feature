Feature: xCall admin management
      In order to perform actions needed admin access in xCall
      As the xCall contract owner
      I need to be able to add a wallet as admin

      Actors:
      | Owner | Non Owner         | Admin               |
      | Alice | Eve, Frank, Grace | Bob, Diana, Charlie |

  Background:
    Given "Alice" is the "xcall" contract owner

  Scenario: 001 - Contract owner Adding an admin wallet to the xCall
    Given "Bob" is an admin wallet who needs to be added as admin
    When "Alice" executes add_admin in xcall with "Bob" wallet address
    Then "Bob" wallet address should be added as admin

  Scenario: 002 - Non Owner Adding an admin wallet to the xCall
    Given "Bob" is an admin wallet who needs to be added as admin
    And "Eve" is not the contract owner of the xCall smart contract
    When "Eve" executes add_admin in xcall with "Bob" wallet address
    Then xCall returns an error message that only the contract owner can perform this action
    And "Bob" wallet address should not be added as admin

  Scenario: 003 - An admin cannot add another admin to the xCall
    Given "Alice" has already added "Bob" wallet address as admin
    And "Diana" is an admin wallet who needs to be added as admin
    When "Bob" executes add_admin in xcall with "Diana" wallet address
    Then xCall returns an error message that only the contract owner can perform this action
    And "Diana" wallet address should not be added as admin

  Scenario: 004 - Preventing the addition of an existing admin wallet to xCall
    Given "Alice" has already added "Bob" wallet address as admin
    When "Alice" executes add_admin in xcall with "Bob" wallet address
    Then xCall returns an error message that the admin already exists
    And "Bob" wallet address should still be as admin

  Scenario: 005 - Preventing the addition of null value as an admin wallet to xCall
    When "Alice" executes add_admin in xcall with "Null" wallet address
    Then xCall returns an error message that the null value cannot be added as admin
    And no wallet address should be as admin

  Scenario: 006 - Preventing the addition of junk characters as an admin wallet to xCall
    When "Alice" executes add_admin in xcall with "Junk" wallet address
    Then xCall returns an error message that  wallet address of the new admin is not a valid address
    And no wallet address should be as admin

  Scenario: 007 Contract owner can update an existing admin wallet in xCall
    Given "Alice" has already added "Bob" wallet address as admin
    And "Diana" is an admin wallet who needs to be added as admin
    When "Alice" executes update_admin in xcall with "Diana" wallet address
    Then xCall should update xCall admin with "Diana" address

  Scenario: 008 Contract owner can remove an existing admin wallet in xCall
    Given "Alice" has already added "Bob" wallet address as admin
    When "Alice" executes remove_admin in xcall
    Then xCall should remove "Bob" wallet address as admin

  Scenario: 009 non-owner cannot update admin in xCall
    Given "Alice" has already added "Bob" wallet address as admin
    And "Diana" is an admin wallet who needs to be added as admin
    And "Eve" is not the contract owner of the xCall smart contract
    When "Eve" executes update_admin in xcall with "Diana" wallet address
    Then xCall returns an error message that only the contract owner can perform this action
    And "Diana" wallet address should not be added as admin

  Scenario: 010 owner cannot update admin with already existing admin in xCall
    Given "Alice" is the "xcall" contract owner
    And "Alice" has already added "Bob" wallet address as admin
    When "Alice" executes update_admin in xcall with "Bob" wallet address
    Then xCall returns an error message that the admin already exists
    And "Bob" wallet address should still be as admin

  Scenario: 011 owner cannot update admin with null value as an admin to xCall
    Given "Alice" has already added "Bob" wallet address as admin
    When "Alice" executes update_admin in xcall with "Null" wallet address
    Then xCall returns an error message that the null value cannot be added as admin
    And "Bob" wallet address should still be as admin

  Scenario: 012 Admin cannot update admin in xCall
    Given "Alice" has already added "Bob" wallet address as admin
    And "Diana" is an admin wallet who needs to be added as admin
    When "Bob" executes update_admin in xcall with "Diana" wallet address
    Then xCall returns an error message that only the contract owner can perform this action
    And "Bob" wallet address should still be as admin

  Scenario: 013 Non owner cannot remove admin
    And "Alice" has already added "Bob" wallet address as admin
    And "Eve" is not the contract owner of the xCall smart contract
    When "Eve" executes remove_admin in xcall
    Then xCall returns an error message that only the contract owner can perform this action
    And "Bob" wallet address should still be as admin

  Scenario: 014 Contract owner cannot set admin twice
    And "Alice" has already added "Bob" wallet address as admin
    And "Diana" is an admin wallet who needs to be added as admin
    When "Alice" executes set_admin in xcall with "Diana" wallet address
    Then xCall returns an error message that admin is already set
    And "Bob" wallet address should still be as admin