## Terminologies

- [ibc packet](#ibc-packet)
  
The IbcPacket structure contains all information needed to process the receipt. This info has already been verified by the core IBC modules via light client and merkle proofs

- [ibc message](#ibc-message)

Sends bank tokens owned by the contract to the given address on another chain. The channel must already be established between the ibctransfer module on this chain and a matching module on the remote chain. 

- #### xCall Handshakes 

This will occurs between the two chains

- [OpenInit](#openinit)

 OpenInit initializes any connection which may occur, while still necessitating agreement from both sides

- [OpenTry](#opentry)

OpenInit is followed by an OpenTry response, in which chain B verifies the identity of chain A according to information that chain B has about chain A in its light client

- [OpenAck](#openack)

 OpenAck is very similar to the functionality of OpenInit, except that the information verification now occurs for chain A.

- [OpenConfirm](#openconfirm)

 OpenConfirm is the final handshake, in which chain B confirms that both self-identification and counterparty identification were successful.

---

### ibc packet

The IbcPacket structure contains all information needed to process the receipt. This info has already been verified by the core IBC modules via light client and merkle proofs

It has following fields:

| Name     | Type      | Description                                           |
|:---------|:----------|:------------------------------------------------------|
| data     | Binary    | The raw data send from the other side in the packet   |
| src      |IbcEndpoint| identifies the channel and port on the sending chain  |
| dest     |IbcEndpoint| identifies the channel and port on the receiving chain|
| sequence | u64       | The sequence number of the packet on the given channel|
|timeout   |IbcTimeout | Timeout object                                        |

### ibc message

Sends bank tokens owned by the contract to the given address on another chain. The channel must already be established between the ibctransfer module on this chain and a matching module on the remote chain.

| Name       | Type      | Description                                           |
|:---------  |:----------|:------------------------------------------------------|
|channel Id  | String    | exisiting channel to send the tokens over             |
| to_address | String    | address on the remote chain to receive these tokens   |
| amount     | Coin      | packet data only supports one coin                    |
| timeout    | IbcTimeout| when packet times out, measured on remote chain       |

### openInit

OpenInit initializes any connection which may occur, while still necessitating agreement from both sides

| Name         | Type          | Description                                           |
|:-------------|:--------------|:------------------------------------------------------|
|clientID      | String        | exisiting client to send the tokens over              |
| counterparty | Counterparty  | identifies the channel in the receiving chain         |
| version      | Version       | packet data only supports one coin                    |
| delayPeriod  | IbcTimeout    | Timeout                                               |

### openTry

openInit is followed by an OpenTry response, in which chain B verifies the identity of chain A according to information that chain B has about chain A in its light client

| Name         | Type          | Description                                                         |
|:-------------|:--------------|:--------------------------------------------------------------------|
|clientID      | String        | exisiting client to send the tokens over                            |
| counterparty | Counterparty  | identifies the channel in the receiving chain                       |
| version      | Version       | packet data only supports one coin                                  |
| delayPeriod  | IbcTimeout    | Timeout                                                             |
| proofInit    | u128          | proof that chainA stored connectionEnd in state                     |
|proofConsensus| u128          | packet data only supports one coin                                  |
| proofClient  | u128          | proof that chainA stored a light client of chainB                   |   
| proofHeight  | u128          | height at which relayer constructs proof of A storing connectionEnd |

### openAck 

OpenAck is very similar to the functionality of OpenInit, except that the information verification now occurs for chain A.

| Name         | Type          | Description                                                         |
|:-------------|:--------------|:--------------------------------------------------------------------|
|clientID      | String        | exisiting client to send the tokens over                            |
| counterparty | Counterparty  | identifies the channel in the receiving chain                       |
| version      | Version       | packet data only supports one coin                                  |
| delayPeriod  | IbcTimeout    | Timeout                                                             |
| proofInit    | u128          | proof that chainA stored connectionEnd in state                     |
|proofConsensus| u128          | packet data only supports one coin                                  |
| proofClient  | u128          | proof that chainA stored a light client of chainB                   |   
| proofHeight  | u128          | height at which relayer constructs proof of A storing connectionEnd |

### openConfirm

| Name         | Type          | Description                                                         |
|:-------------|:--------------|:--------------------------------------------------------------------|
|connectionId  | String        | exisiting connection to send the tokens over                        |
| proofAck     | u128          | proof that connection opened on Chain A during ConnOpenAck          |
| version      | Version       | packet data only supports one coin                                  |
|  proofHeight |u128           | height at which relayer constructs proof of A storing connectionEnd |






