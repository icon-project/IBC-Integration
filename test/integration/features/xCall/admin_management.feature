Feature: xCall admin management
    In order to perform actions needed admin access in xCall
    As the xCall contract owner
    I need to be able to add a wallet to the list of xCall admins

  Background: 
    Given xCall contract is deployed and initialized

  Scenario: Adding an admin wallet to the xCall
    Given Alice is the xCall contract owner
    And Bob is an admin wallet who needs to be added to the list of xCall admins
    When Alice executes add_admin in xCall with Bob's wallet address
    Then Bob's wallet address should be added to the list of xCall admins

  Scenario: Adding an already existing admin wallet to the xCall
    Given Alice is the xCall contract owner
    And Bob is an existing admin wallet in the list of xCall admins
    When Alice executes add_admin in xCall with Bob's wallet address
    Then Bob's wallet address should still be in the list of xCall admins
    And no new entry should be created in the list of xCall admins

  Scenario: Adding an invalid wallet address to the xCall admin list
    Given Alice is the xCall contract owner
    And Bob provides an invalid wallet address
    When Alice executes add_admin in xCall with Bob's wallet address
    Then the add_admin function should fail
    And no new entry should be added to the list of xCall admins

  Scenario: Removing an admin wallet from the xCall
    Given Alice is the xCall contract owner
    And Bob is an admin wallet in the list of xCall admins
    When Alice executes remove_admin in xCall with Bob's wallet address
    Then Bob's wallet address should be removed from the list of xCall admins

  Scenario: Removing a non-existing admin wallet from the xCall
    Given Alice is the xCall contract owner
    And Bob is not an admin wallet in the list of xCall admins
    When Alice executes remove_admin in xCall with Bob's wallet address
    Then the remove_admin function should fail
    And no entry should be removed from the list of xCall admins

  Scenario: Adding an admin wallet by a non-owner
    Given Alice is the xCall contract owner
    And Eve is not the xCall contract owner
    And Bob is an admin wallet who needs to be added to the list of xCall admins
    When Eve executes add_admin in xCall with Bob's wallet address
    Then the add_admin function should fail
    And no new entry should be added to the list of xCall admins

  Scenario: Removing an admin wallet by a non-owner
    Given Alice is the xCall contract owner
    And Eve is not the xCall contract owner
    And Bob is an admin wallet in the list of xCall admins
    When Eve executes remove_admin in xCall with Bob's wallet address
    Then the remove_admin function should fail
    And no entry should be removed from the list of xCall admins
