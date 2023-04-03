Feature: xCall admin management
      In order to perform actions needed admin access in xCall
      As the xCall contract owner
      I need to be able to add a wallet as admin

      Actors:
      | Owner | Non Owner         | Admin               |
      | Alice | Eve, Frank, Grace | Bob, Diana, Charlie |

  Scenario: Adding an admin wallet to the xCall
    Given "Alice" is the "xcall" contract owner
    And "Bob" is an admin wallet who needs to be added to the list of xCall admins
    When "Alice" executes add_admin in xcall with "Bob" wallet address
    Then "Bob" wallet address should be added as admin

  Scenario: Non Owner Adding an admin wallet to the xCall
    Given "Alice" is the "xcall" contract owner
    And "Bob" is an admin wallet who needs to be added to the list of xCall admins
    And "Eve" is not the contract owner of the xCall smart contract
    When "Eve" executes add_admin in xcall with "Bob" wallet address
    Then xCall returns an error message that only the contract owner can perform this action
    And "Bob" wallet address should not be added as admin
