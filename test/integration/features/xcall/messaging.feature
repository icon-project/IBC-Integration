Feature: send call message to another person
            In order to perform cross chain communication
            as the dapp contract owner
            i need to be able to send message through xcall to other chain

            Actors:
            | Owner | Non Owner         | Admin               |
            | Alice | Eve, Frank, Grace | Bob, Diana, Charlie |

    Background:
        Given "BMC" contract deployed by "Alice" only when the chain is "icon"
        Given "Alice" is the "ibcCore" contract owner
        Given "Alice" is the "xcall" contract owner
        Given "Alice" is the "Dapp" contract owner
        And "Alice" should open channel to send and receive messages

    Scenario: 001 - Send packet fails if caller is not contract
        When "Alice" non contract executes "send_call_message" in xcall
        Then "xcall" contract throws an error that only the contract can perform this action

    Scenario: 002 - Contract panics when data size is greater than limit
        When "Alice" executes "send_call_message" in dapp with "data size greater" than limit
        Then xcall contract panic with an error MaxDataSizeExceeded

    Scenario: 003 - Contract panics when data size is equal to limit
        When "Alice" executes "send_call_message" in dapp with "data size equals" to limit
        Then xcall contract should emit a event with sequence id and request id

    Scenario: 004 - Contract panics when data size is less than limit
        When "Alice" executes "send_call_message" in dapp with "data size less" than limit
        Then xcall contract should emit a event with sequence id and request id

    Scenario: 005 - Contract panics when rollback size is greater than limit
        When "Alice" executes "send_call_message" in dapp with "rollback size greater" than limit
        Then xcall contract panic with an error MaxRollbackSizeExceeded

    Scenario: 006 - Contract panics when rollback size is equal to limit
        When "Alice" executes "send_call_message" in dapp with "rollback size equals" to limit
        Then xcall contract should emit a event with sequence id and request id

    Scenario: 007 - Contract panics when rollback size is less than limit
        When "Alice" executes "send_call_message" in dapp with "rollback size less" than limit
        Then xcall contract should emit a event with sequence id and request id

    Scenario: 008 - Contract panics when wrong request ID is passed for executeCall
        When "Alice" executes "execute_call" in "xcall" with "incorrect" request ID
        Then xcall contract panic with an error RequestNotFound

    Scenario: 009 - Contract executes call message with correct request ID
        When "Alice" executes "execute_call" in "xcall" with "correct" request ID
        Then xcall should execute call message successfully

    Scenario: 010 - Contract executes call message with null request ID
        When "Alice" executes "execute_call" in "xcall" with "null" request ID
        Then xcall contract panic with an error RequestNotFound

