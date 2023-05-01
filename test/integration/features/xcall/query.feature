Feature: Querying data
            Actors:
            | Owner | Non Owner         | Admin               |
            | Alice | Eve, Frank, Grace | Bob, Diana, Charlie |

    Background:
        Given "BMC" contract deployed by "Alice" only when the chain is "icon"
        And "Alice" is the "xcall" contract owner

    Scenario: 001 - Query admin after adding admin
        Given "Bob" is an admin wallet who needs to be added as admin
        And "Alice" executes set_admin in xcall with "Bob" wallet address
        When we query for admin "Bob" wallet address should be as admin