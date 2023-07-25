# XCall


## Introduction
xCall is a standard interface to make calls between different blockchain networks.

## Protocol Overview
![](https://hackmd.io/_uploads/SkBUxFhBh.png)


## Prerequisites
### Network Addresses
XCall uses network address to refer to different addresses across many networks. A "network address" consists of a network and an account section. A network address is represented as a string with "networkId" and "account" separated by /.
A networkId is a unique id of a network and there can't be two networks with the same id connection to the same xCall network.

### Connections
XCall is designed to utilize a wide range of bridging protocols that facilitate data transfer, known as connections. These connections can be selected by users and dApps, ensuring a permissionless protocol. However, this places a responsibility on the dApps to verify that they exclusively accept messages from trusted protocols. Users can also opt to use the default connections set up by the xCall admin and in this case does not need to manage connections at all.

## Protocol Specification
### Sending Messages
Sending messages via xCall is simply done by calling sendCallMessage on the xCall contract. `_to` address is a networkAddress used by xCall to figure out the destination chain.
The user can also  specify which connections to use, if not specified the default connections will be used. This also allows dapps to have their messages secured by multiple protocols.

The default connections are specified by and admin and can be changed at any time.
```javascript
/**
 * Sends a call message to the contract on the destination chain.
 *
 * @param _to The network address of the callee on the destination chain
 * @param _data The calldata specific to the target contract
 * @param _rollback (Optional) Data used to specify error handling of a two-way messages
 * @param sources  (Optional) The contracts that will be used to send the message
 * @param destinations (Optional) The addresses of the contracts that xcall will expect the message from.
 *
 * @return The serial number of the request
 */
payable external sendCallMessage(String _to,
                                byte[] _data,
                                @Optional bytes _rollback,
                                @Optional String[] sources
                                @Optional String[] destinations) return Integer;
```
### Events
#### CallMessage
CallMessage event is emitted when a new message is received by xCall and is ready to be executed.

 `_from` The network address of the caller on the source chain
 `_to` A string representation of the callee address
 `_sn` The serial number of the request from the source
 `_reqId` The request id of the destination chain used in execute call
 `_data` The calldata
```javascript
CallMessage {
    String _from,
    String _to,
    Integer _sn,
    Integer _reqId,
    byte[] data
}
```
#### CallExecuted
CallExecuted event is emitted when a message is executed. For one way message error is always empty
 `_reqId` The message id
 `_code` The execution result code (0: Success, -1: Unknown generic failure, >=1: User defined error code)
 `_msg` Error message
```javascript
CallExecuted{
    Integer _reqId,
    Integer _code,
    String _msg
}
```
#### ResponseMessage
ResponseMessage is emitted for all two-way messages (i.e., _rollback is non-null), the xcall on the source chain receives a response message from the xcall on the destination chain and emits the following event regardless of its success or not.
 `_sn` The message id
 `_code` The execution result code (1: Success, 0: failure)
```javascript
ResponseMessage{
    Integer _sn,
    Integer _code
}
```
#### RollbackMessage
RollbackMessage is emitted when an error occurred on the destination chain and the _rollback is non-null, xcall on the source chain emits the following event for notifying the user that an additional rollback operation is required.
 `_sn` The message id
```javascript
RollbackMessage {
    Integer _sn
}
```


#### RollbackExecuted
RollbackExecuted event is emitted when a rollback message is executed.
 `_sn` The message id
```javascript
RollbackExecuted{
    Integer _sn,
}
```


#### CallMessageSent
CallMessageSent is emitted when for each message sent.
 ` _from` The network address of the caller
 `_to` The network address of the calle
 `_sn` The serial number of the request

```javascript
CallMessageSent{
    Address _from,
    String _to,
    Integer _sn
}
```

### Receiving Messages

#### Execution

The user on the destination chain recognizes the call request and invokes the following method on xcall with the given _reqId and _data.
To minimize the gas cost, the calldata payload delivered from the source chain are exported to event, `_data` field, instead of storing it in the state db. In case of a two-way message rollback will be triggered in case of failure.
The `_data` payload should be repopulated by the user (or client) when calling the following `executeCall` method.
Then `xcall` compares it with the saved hash value to validate its integrity.

```javascript
/**
 * Executes the requested call message.
 *
 * @param _reqId The request Id
 * @param _data The calldata
 */
external executeCall(BigInteger _reqId, byte[] _data);
```
The user on the source chain recognizes the rollback situation and invokes the following method on xcall with the given _sn.
Note that the executeRollback can be called only when the original call request has responded with a failure.
It should be reverted when there is no failure response with the call request.
```javascript
/**
 * Rollbacks the caller state of the request '_sn'.
 *
 * @param _sn The serial number of the previous request
 */
external executeRollback(BigInteger _sn);
```

#### Handling

When the user calls executeCall or executeRollback method, the xcall invokes the following predefined method in the target DApp with the calldata associated in _reqId. If only using default protocols implementing only the two parameter version is preferred
```javascript
/**
 * Handles the call message received from the source chain.
 * Only called from the Call Message Service.
 *
 * @param _from The network address of the caller on the source chain
 * @param _data The calldata delivered from the caller
 * @param _protocols The contract addresses that delivered the data, if omitted the default protocol was used
 */
external handleCallMessage(String _from, byte[] _data);
external handleCallMessage(String _from, byte[] _data, @Optional String[] _protocols);
```

#### Success verification
If rollback was specified and the call was successful, the success can be verified,


```javascript
/**
 * checks if message '_sn' did succeed on target chain.
 *
 * @param _sn The serial number of the request
 *
 * @return If the '_sn' has received a success response
 */
external readonly verifySuccess(BigInteger _sn) returns boolean;
```

### Fee Managment
Sending a message through xCall has 2 types of fees. One for using the protocol and one for each connections used.

```javascript
/**
 * Gets the fee for delivering a message to the _net.
 * If the sender is going to provide rollback data, the _rollback param should set as true.
 * The returned fee is the total fee required to send the message.
 *
 * @param _protocol The protocol/connection used
 * @param _net The network id
 * @param _rollback Indicates whether it provides rollback data
 * @param sources The protocols used to send the message is omitted default protocol is used.
 * @return The total fee of sending the message
 */
external readonly getFee(String _net,
                          boolean _rollback
                          @Optional String[] sources) returns Integer;
```

```javascript
/**
 * Gets the protocol fee for sending a xCall message
 *
 * @return the xCall protocol fee
 */
external readonly getProtocolFee() Returns Integer;
```

### Error Handling
**TODO: This section is not finished**
Since one way messages are standalone not much happens if they are dropped or lost but when using multi protocol with rollbacks there are a few cases where extra handling is needed

##### Partial delivery: Message is dropped by one or more protocol but delivered by others.
**Mitigation**: If message is a one way message the recommended approach would be to disallow messages from the non working protocol and then safely be able to resend message without worrying about double spending.
If message requires rollback start a timed rollback which will rollback the message after x days via the protocols that delivered the message. Once rollbacked use forced rollback to try a rollback without waiting for all protocols.

##### Dropped package: Message is dropped by all connections.
**Mitigation**: All protocols involved should be removed as trusted in your dapp. Then with new connections it would be safe to resend the message. If the original message required rollback it has to be solved through other means, like governance or similar.

##### Partial failure: Message fails to be delivered and trigger rollbacks via one or more protocols but delivered by others.
**Mitigation**: Sames a partial delivery but a forced rollback should not be needed since all protocols will rollback

##### Partial failure delivery: Message rollback is dropped by one or more protocols but delivered by others.
**Mitigation**: Once removing the non working protocol you can try force rollback with the protocols that has delivered the rollback.

##### Dropped failure delivery: Rollback is dropped by all connections:
**Mitigation**: All protocols involved should be removed as trusted in your dapp. After that it has to be solved through other means, like governance or similar.


### Security Considerations

The security of xCall comes from the security of the underlying connections. It is up to the dapp to verify that protocols are checked to be valid during `handleCallMessage`.

## Implementation Guidelines

### Connections
The provided code snippet demonstrates a specific behavior of a connection. It consists of two external functions: sendMessage(targetNetwork, svc, sn, msg) and getFee(network, response).

The sendMessage function is responsible for sending a message to a specified targetNetwork. It accepts four parameters: targetNetwork, svc, sn, and msg.

The behavior of sendMessage depends on the value of sn (sequence number). If sn is greater than 0, it indicates a new message that requires a response. In this case, both the sending fee and the response fee should be included. If sn is 0, it signifies a one-way message where no response is expected. If sn is less than 0, it implies that the message is a response to a previously received message. In this scenario, no fee is included in the sending message since it should have already been paid when the positive sn was sent.

After handling the sn value, the sendMessage function triggers the handleMessage function on the targetNetwork. It passes targetNetwork and msg as arguments to handleMessage. The purpose of this function is to handle the incoming message on the specified targetNetwork's xCall contract.

In case the message fails to be delivered for any reason, the code triggers the handleError function. It passes the failed sn to the function. The responsibility of this function is to handle errors that occur during the message delivery process.

The second external function, getFee(network, response), calculates and returns the fee required to send a message to the specified network and back. It takes into account the optional response parameter when determining the fee.

In summary, this code snippet illustrates a specific behavior expected from a connection regarding message sending and error handling.
```javascript
external function sendMessage(targetNetwork, svc, sn, msg)
    On targetNetwork, trigger handleMessage(targetNetwork, msg)
    if message fails to deliver:
        trigger handleError(sn)
```
``` javascript
external function getFee(network, response)
    Returns the fee required to send a message to "network" and back, considering the optional response parameter.
```


### Data Structures
#### Messages
All messages when passed to a connection are RLP encoded.
RLP encoding order is the same as the order they are defined in below.

##### CSMessageRequest
```javascript
CSMessageRequest {
    String from;
    String to;
    BigInteger sn;
    boolean rollback;
    bytes data;
    String[] protocols;
}
```
##### CSMessageResponse
```javascript
int SUCCESS = 1;
int FAILURE = 0;
CSMessageResponse {
    BigInteger sn;
    int code;
}
```
##### CSMessage
```javascript
int REQUEST = 1;
int RESPONSE = 2;
CSMessage {
  // The message type, either REQUEST or RESPONSE
  int type;
  // RLP encoded bytes of the Message
  bytes data;
}
```

#### Internal structs
```javascript
CallRequest {
    Address from;
    String netTo;
    String[] protocols;
    bytes rollback;
    boolean enabled = false; // defaults to false
}
```
### Storage
```
MAX_DATA_SIZE: <size>
MAX_ROLLBACK_SIZE: <size>
NID: <chains networkId>

sn: <current send message sequence>
reqId: <current incoming message sequence>
requests: sn -> CallRequest
proxyReqs: reqId -> CSMessageRequest

# default values should be false in case of boolean storage
pendingReqs: msgHash -> connection address -> boolean
pendingResponses: sn -> connection address -> boolean
successfulResponses: sn -> boolean

admin: <admin>
defaultConnection: networkId -> Address
protocolFee: <protocolFee>
feeHandler: <Address>
```

### Contract initialization
```javascript
function init(String networkId) {
    NID = networkId
    if admin == null
        admin = getCaller();
        feeHandler = getCaller();

}
```

### Communication
#### Sending messages
`sendCallMessage` sends a some arbitrary data to `_to` via a path specified by the caller.

`_to`: The network address of the target contract.
`_data`: The data to be sent to the `_to` contract.
`_rollback`: The data to be returned to the caller in case of failure.
`sources`: A set of addresses representing the connections to be used when sending the message. These connections are also used to verify potential rollbacks
`destination`: The addresses that the target contract should wait for messages from before considering the it complete.

```javascript
payable external function sendCallMessage(String _to,
                                          byte[] _data,
                                          @Optional bytes _rollback,
                                          @Optional String[] sources
                                          @Optional String[] destinations) returns Integer {
    caller = getCaller()
    require(caller.isContract() || _rollback == null, "RollbackNotPossible");
    require(_rollback == null || _rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded")

    sn++
    dst = NetworkAddress(_to)
    from = NetworkAddress(NID, caller).toString()

    needResponse = _rollback != null && _rollback.length > 0
    if needResponse:
        req = CallRequest(caller, dst.net(), sources, _rollback)
        requests[sn] = req


    msgReq = CSMessageRequest(from, dst.account(), sn, needResponse, _data, destinations)
    msg = CSMessage(CSMessage.REQUEST, msgReq.toBytes()).toBytes();
    require(msg.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded")

    sendSn = needResponse ? sn : 0
    if sources == []:
        src = defaultConnection[dst.net()]
        fee = src->getFee(dst.net(), needResponse)
        src->sendMessage(fee, dst.net(), "xcall", sendSn, msg)
    else:
        for src in sources:
            fee = src->getFee(dst.net(), needResponse)
            src->sendMessage(fee, dst.net(), "xcall", sendSn, msg)


    remaningBalance = getBalance();
    require(remaningBalance >= getProtocolFee())
    transfer(feeHandler, balance)
    emit CallMessageSent(caller, dst.toString(), sn)

    return sn
}

```
#### Receiving messages
`handleMessage` is the external function used by connections to deliver messages.
```javascript
external function handleMessage(String _from, bytes _msg) {
    msg = CSMessage.decode(_msg);
    switch (msg.type):
        case CSMessage.REQUEST:
            handleRequest(_from, msg.data);
            break;
        case CSMessage.RESPONSE:
            handleResponse(msg.data);
            break;
        default:
            Context.revert("UnknownMsgType(" + msg.type + ")");
}
```
`handleError` is the external function used by connections to report error messages..
```javascript
external function handleError(BigInteger _sn) {
        CSMessageResponse res = CSMessageResponse(_sn, CSMessageResponse.FAILURE);
        handleResponse(res.toBytes());
}
```

`handleBTPMessage` Can be added to natively support the BTP protocol without a standalone connection.

```javascript
external function handleBTPMessage(String _from, _svc String, Integer _sn, bytes _msg) {
    // verify svc is as registered
    handleMessage(_from, _msg)
}
 ```
`handleBTPError` Can be added to natively support the BTP protocol without a standalone connection.
```javascript
external function handleBTPError(String _src, String _svc, BigInteger _sn, long _code, String _msg) {
    // verify svc is as registered
    handleError(_sn)
}
```

```javascript
internal function handleRequest(String srcNet, bytes data) {
    msgReq = CSMessageRequest.decode(data);
    from = NetworkAddress(msgReq.from);
    require(from.net() == srcNet);
    source = getCaller();

    if (msgReq.protocols.length > 1):
        _hash = hash(data);
        pendingReqs[_hash][source] = true;
        for (protocol : msgReq.protocols):
            if (!pendingReqs[_hash][protocol]):
                return;

        for (protocol : msgReq.protocols):
            pendingReqs[_hash][protocol] = null;
    else if (msgReq.protocols.length == 1):
        require(source == msgReq.protocols[0]);
    else:
        require(source == defaultConnection[srcNet]);
    reqId = getNextReqId();

    emit CallMessage(msgReq.from, msgReq.to, msgReq.sn, reqId, msgReq.data);
    msgReq.data = hash(msgReq.data)
    proxyReqs[reqId] = msgReq;
}
```

```javascript
internal function handleResponse(data bytes) {
        response = CSMessageResponse.decode(data);
        resSn = response.sn;
        req = requests[resSn];
        source = getCaller

        if req == null:
            return; // just ignore

        if req.protocols.length > 1:
            pendingResponses.at(resSn).set(source, true);
            for protocol : req.protocols:
                if !pendingResponses[resSn][protocol]:
                    return;

            for (String protocol : protocols):
                pendingResponses[resSn][protocol] = null
        else if (msgReq.protocols.length == 1):
            require(source == msgReq.protocols[0]);
        else:
            require(source == defaultConnection[req.netTo]);

        emit ResponseMessage(resSn, response.getCode());
        switch response.getCode():
            case CSMessageResponse.SUCCESS:
                requests[resSn] = null;
                successfulResponses[resSn] = 1;
                break;
            case CSMessageResponse.FAILURE:
            default:
                // emit rollback event
                require(req.rollback != null, "NoRollbackData");
                req.enabled = true;
                requests[resSn] = req;
                emit RollbackMessage(resSn);
}
```

#### Message Execution
If a two-message the function should allow the call to fail and send a new message to rollback the message. While if a one way message fails re-execution should be allowed.
```javascript
external function executeCall(Integer _reqId, byte[] data) {
        req = proxyReqs[_reqId];
        require(req != null, "InvalidRequestId");
        proxyReqs[_reqId] == null;

        assert hash(data) == req.data

        from = NetworkAddress(req.from);
        if !req.needRollback():
             if req.protocols == []:
                req.to->handleCallMessage(req.from, data);
            else:
                req.to->handleCallMessage(req.from, data, req.protocols);
            emit CallExecuted(_reqId, CSMessageResponse.SUCCESS, "");
        else:

            ErrorMessage = ""
            try:
                 if req.protocols == []:
                    req.to->handleCallMessage(req.from, data);
                else:
                    req.to->handleCallMessage(req.from, data, req.protocols);
                response = new CSMessageResponse(req.sn, CSMessageResponse.SUCCESS);
            catch err:
                response = new CSMessageResponse(req.sn, CSMessageResponse.FAILURE);
                ErrorMessage = err.message

            emit CallExecuted(_reqId, response.code, ErrorMessage);
            sn = req.sn.negate();
            msg = CSMessage(CSMessage.RESPONSE, response.toBytes());
            if req.protocols == []:
                protocol = defaultConnection[from.net()]
                protocol->sendMessage(from.net(), "xcall", sn, msg.toBytes())
            else:
                for (String protocol : req.protocols)):
                    protocol->sendMessage(from.net(), "xcall", sn, msg.toBytes())
    }

```
```javascript
external function executeRollback(Integer _sn) {
    req = requests.get(_sn);
    require(req != null, "InvalidSerialNum");
    require(req.enabled, "RollbackNotEnabled");
    requests[_sn] = null;

    if req.protocols == []:
        req.from->handleCallMessage(getNetworkAddress(), req.rollback);
    else:
        req.from->handleCallMessage(getNetworkAddress(), req.rollback req.protocols);

     RollbackExecuted(_sn);
}

```

### Admin methods

```javascript
adminOnly function setAdmin(Address admin){
    admin = admin
}

adminOnly function setProtocolFeeHandler(Address address){
    protocolFeeHandler = address
}

adminOnly function setProtocolFee(Integer fee){
    protocolFee = fee
}

adminOnly function setDefaultConnection(String nid, Address connection){
    defaultConnection.set(nid, connection)
}
```

### Readonly methods

```javascript
external readonly function  getNetworkAddress() returns String {
    return NetworkAddress(NID, this.address);
}
```

```javascript
external readonly function  getNetworkId() returns String {
    return NID;
}
```

```javascript
external readonly function  getProtocolFee() returns Integer {
    return protocolFee;
}
```

```javascript
external readonly function  getFee(String _net,
                                   boolean _rollback
                                   @Optional String[] sources)
                                        returns Integer {
    fee = protocolFee;
    if sources == [] {
        return defaultConnection[_net]->getFee(_net, _rollback) + fee
    }


    for protocol in sources:
        fee += protocol->getFee(_net, _rollback)
    return fee
}
```

```javascript
external readonly function  verifySuccess(Integer sn) returns boolean {
    return successfulResponses[sn];
}
```

### Error Handling
...


## FAQs
...
