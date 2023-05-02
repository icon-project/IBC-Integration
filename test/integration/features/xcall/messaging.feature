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

    # Scenario: 001 - Send packet fails if caller is not contract
    #     When "Alice" non contract executes "send_call_message" in xcall
    #     Then "xcall" contract throws an error that only the contract can perform this action

    Scenario: 002 - Contract panics when data size is greater than limit
        When "Alice" contract executes "send_call_message" in dapp with "data size greater" than limit
        Then xcall contract panic with an error MaxDataSizeExceeded
