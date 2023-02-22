# XCall ADR

## Introduction
This document describes the design to implement xcall as IBC Module to support IBC.

## Terminologies

| Term  | Definition                       | Link                                                                     |
|:------|:---------------------------------|:-------------------------------------------------------------------------|
| xcall | Arbitrary Call Service           | [IIP52](https://github.com/icon-project/IIPs/blob/master/IIPS/iip-52.md) |
| IBC   | Inter Blockchain Communication   |                                                                          |
| BTP   | Blockchain Transmission Protocol |                                                                          |


## Considerations
XCall is a application level contract which does not need to know any complexities of IBC. 
When relayer does a transaction on the destination chain, relayer need incur the cost required to execute the transaction, which can get expensive for the relay.

On the xcall specification, relayer saves the transaction on destination chain. The user needs to execute the transaction themselves, so the burden of transaction fee for transaction execution does not need to go to the relay.

This is a WIP, some specs might be changed later with the development. The logic for fees are not included in this document and will be added later.

## Design
XCall is a application module to implement IBCModule interface. The xcall is defined by [IIP52](https://github.com/icon-project/IIPs/blob/master/IIPS/iip-52.md). To be used in ICON-IBC, some of the specs defined on the IIP52 is modified to support IBC standards.

This was designed taking an example of IBC Module and xcall contract.

#### Implementations:
- [xcall](https://github.com/icon-project/btp/blob/iconloop-v2/solidity/xcall/contracts/CallService.sol)
- [IBC Module](https://github.com/hyperledger-labs/yui-ibc-solidity/blob/main/contracts/apps/20-transfer/ICS20Transfer.sol)

## Data Structures

### Constants
```go
const ( 
    CS_REQUEST = 1
    CS_RESPONSE = 2

    CS_RESP_SUCCESS = 0;
    CS_RESP_FAILURE = -1;
    CS_RESP_IBC_ERROR = -2;
)
```
### Structs:
```go
type CSMessageRequest struct {
	from address
	to string
	sn uint
	rollback []byte
	data []byte
}

type CSMessage struct {
    msgType int 
    payload []byte
}

type CSMessageResponse struct {
	sn uint;
	code int;
	msg string;
}

type PacketData struct {
    sequence uint
    source_port string
    source_channel string
    destination_port string
    destination_channel string
    data []byte // CSMessageRequest while sending, CSMessage while receiving
    timeout_height Height
    timeout_timestamp uint
  }
```

### Flow Diagram
![ICON xCall Design (1)](https://user-images.githubusercontent.com/25897013/220556832-4fe50011-a80f-417a-ad3e-bc4ada234413.png)



### Events
These events are to be used as per the specifications as defined in IIP 52.

#### 1. CallMessageSent()
#### 2. CallMessage()
#### 3. RollbackMessage(sn, rollback, message)
#### 4. CallRequestCleared(sn)


### Methods required
#### 1. SetDestinationChannelAndPort
```go
func setDestinationChannelAndPort(channel string, port string) {
	destination_port = port
	destination_channel = channel

}
```
#### 2. SetSourceChannelAndPort
Should be a private method, and set after OnChanOpenConfirm/Ack handshaking.
```go
func setSourceChannelAndPort(channel string, port string) {
	source_port = port
	source_channel = channel
}
```


### IBCModule and XCall Interface Methods:

#### 1. SendPacket
This is equivalent to `sendCallMessage` method on IIP 52. 
This method is called to send IBC message from one chain to another.
It generates a **CallMessageSent** eventlog.

```go
func sendPacket(data []byte, rollback []byte, timeout_height int) {

	sn ++

	if rollback {
		// save to a sn
	}

	// this section is to replace sendBTPMessage() in IIP52
	PacketData d = {
		sequence: ibcHandler.getNextSequence(),
		source_port: source_port,
		source_channel: source_channel,
		destination_port: destination_port,
		data: CSMessageRequest({
			from: msg.sender,
			to: destinationAddress (destinationBalancedAddress).
			sn: sn
			rollback: rollback,
			data: data
		}).toBytes()
		timeout_height: ..
		timeout_timestamp: 0
	}

	ibcHandler.sendPacket(d)
	XCallMessageSent(from, sn, reqId, data)
}
```
rollback parameter is used for error handling. Works as defined on IIP 52. 

#### 2. RecvPacket
Equivalent to `handleBTPMessage` on xcall.

```go
func recvPacket(msg PacketData) {
	require(msg.sender == address(ibcHandler))
	
	CSMessage csMsg = msg.data

	if csMsg.msgType == CS.REQUEST {
		handleRequest() // as per implementation of xcall in btp
	} else id csMsg.msgType == CS.RESPONSE {
		handleResponse() // as per implementation of xcall in btp
	} else {
		revert()
	}
}
```

#### 3. ExecuteCall
Executes transaction saved in reqId
```go
// executes transaction corresponding to requestId
func executeCall(reqId int) {
	CSMessageRequest msgReq = proxyReqs[reqId]
	// try executing transaction defined on msgReq as defined on IIP52.
}
```

#### 4. ExecuteRollback
Generates **CallRequestCleared** eventlog
```go
func executeRollback(sn int) {
	// same as IIP52
}
```

### 5. OnAcknowledgementPacket
This part ideally would be handled by handleResponse(). 
```go
func onAcknowledgementPacket (
	callData PacketData,
	acknowledgement []byte
) {

}
```

**The following are the channel handshaking interfaces as defined by IBCModule interface.
Referenced from [yui-ibc-solidity](https://github.com/hyperledger-labs/yui-ibc-solidity/blob/main/contracts/core/05-port/IIBCModule.sol). The implementation of these methods might change.**

```go
func onChanOpenInit(
        Channel.Order,
        connectionHops string[] ,
        portId string ,
        channelId string ,
        counterparty ChannelCounterparty.Data ,
        version string 
    ) {
    	// any logic, will be decided during development
    };
```

```go
func onChanOpenTry(
        Channel.Order,
        connectionHops string[],
        portId string,
        channelId string,
        counterparty ChannelCounterparty.Data,
        version string,
        counterpartyVersion string
    ) {

    }
```

```go
func onChanOpenAck(
	portId string, 
	channelId string, 
	counterpartyVersion string
) {

}
```

```go
 func onChanOpenConfirm(
 	portId string, 
 	channelId string
 ) {

 }
 ```

 ```go
 func onChanCloseInit(
 	portId string, 
 	channelId string
 ) {

 }
 ```

 ```go
 func onChanCloseConfirm(
 	portId string, 
 	channelId string
 ) {

 }
 ```

