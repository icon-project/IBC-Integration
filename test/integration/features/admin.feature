Feature: xCall admin management
    In order to perform actions needed admin access in xCall
    As the xCall contract owner
    I need to be able to add a wallet to the list of xCall admins

  Scenario: Adding an admin wallet to the xCall
    Given "Alice" is the "xcall" contract owner
    And "Bob" is an admin wallet who needs to be added to the list of xCall admins
    When "Alice" executes add_admin in xcall with "Bob" wallet address
    Then "Bob" wallet address should be added to the list of xCall admins

 