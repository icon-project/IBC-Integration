# XCall IBC Connection


## Introduction

This specification document describes a IBC enabled xCall connection


## General Connection Design Overview
The provided code snippet demonstrates a specific behavior of a connection. It consists of two external functions: sendMessage(targetNetwork, svc, sn, msg) and getFee(network, response).

The sendMessage function is responsible for sending a message to a specified targetNetwork. It accepts four parameters: targetNetwork, svc, sn, and msg.

The behavior of sendMessage depends on the value of sn (sequence number). If sn is greater than 0, it indicates a new message that requires a response. In this case, both the sending fee and the response fee should be included. If sn is 0, it signifies a one-way message where no response is expected. If sn is less than 0, it implies that the message is a response to a previously received message. In this scenario, no fee is included in the sending message since it should have already been paid when the positive sn was sent.

After handling the sn value, the sendMessage function triggers the handleMessage function on the targetNetwork. It passes targetNetwork, sn, and msg as arguments to handleMessage. The purpose of this function is to handle the incoming message on the specified targetNetwork's xCall contract.

In case the message fails to be delivered for any reason, the code triggers the handleError function. It passes sn, errorCode, and errorMessage to the function. The responsibility of this function is to handle errors that occur during the message delivery process.

The second external function, getFee(network, response), calculates and returns the fee required to send a message to the specified network and back. It takes into account the optional response parameter when determining the fee.

In summary, this code snippet illustrates a specific behavior expected from a connection regarding message sending and error handling.
``` javascript
external function sendMessage(targetNetwork, svc, sn, msg)
    if sn < 0:
        sn = sn.negate()
    On targetNetwork, trigger handleMessage(targetNetwork, sn, msg)
    if message fails to deliver:
        trigger handleBTPError(sn, errorCode, errorMessage)
```

``` javascript
external function getFee(network, response)
    Returns the fee required to send a message to "network" and back, considering the optional response parameter.
...
```

## General IBC Connection Design Overview
The IBC (Inter-Blockchain Communication) connection facilitates communication between different chains using IBC channels and ports. This connection relies on the administrator to associate network IDs with specific connections/ports. Once established, these properties become immutable and cannot be changed. Consequently, users can depend on the stability and reliability of a configured connection to another chain.

All responses are handled through acknowledgments If a packet timeout occurs, it will be treated as an error.

Two types of fees are associated with this connection. The first fee pertains to delivering the packet itself, while the second fee is related to delivering an acknowledgment. These fee amounts are determined and controlled by the administrator. To ensure user safety, there should be caps on the fee amounts, preventing excessive charges.

Additionally, the packet fee rewards are included with the message and stored on the counterparty connections. These rewards can be claimed separately through a distinct message, providing a mechanism for managing and distributing the associated fees.

## Technical Specification

### Storage
```
configuredNetworkIds : connection->port->NetworkId
configuredClients : connection->clientId
configuredTimeoutHeight : connection->timeoutHeight

lightClients : channelId->clientId
timeoutHeights : channelId->timeoutHeight

PORT: portId
channels : NetworkId -> channelId
networkIds : channelId -> NetworkId
destinationChannel: channelId -> counterPartyChannelId
destinationPort: channelId -> counterPartyPortId

incomingPackets: channel -> sn -> packet
outgoingPackets: channel -> paketSequence -> sn

sendPacketFee: NetworkId -> int
ackFee: NetworkId -> int

unclaimedAckFees: NetworkId -> seqNum -> amount
unclaimedPacketFees NetworkId -> address -> amount
```

### Message struct
The Message struct is used for all packets sent. While acknowledgments only consist of raw data from xCall, the Message struct is used delivering xCall messages but also facilitating fee claiming for relays.

The Message struct is defined as follows:
```java
class Message() {
    BigIntger sn; // Nullable
    BigIntger fee;
    byte[] data;

    public toBytes() {
        RLP.enocode([sn, fee, data])
    }
}
```

sn: This attribute represents the sequence number associated with the message from xCall. This will always be 0 or positive it from xCall. If the message is used to claim fees sn will be -1.

fee: The fee attribute denotes the amount of fee associated with the message or if sn = -1 the amount to be claimed.

data: The data attribute stores the xCall messsage or int case of a fee claim it stored the address to recive the rewards

The toBytes() method is implemented to convert the Message struct into a byte array representation using the RLP (Recursive Length Prefix) encoding.

### Contract initialization
```javascript
function init(Address _xCall, Address _ibc, String port) {
    xCall.set(_xCall);
    ibc.set(_ibc);
    admin.set(Context.getCaller());
    PORT = port
}
```

### Sending Messages
```java
/**
 * Sends the message to a specific network.
 * _sn : positive for two-way message, zero for one-way message, negative for response
 *
 * @param _to  String ( Network Address of destination network )
 * @param _svc String ( name of the service )
 * @param _sn  Integer ( serial number of the message )
 * @param _msg Bytes ( serialized bytes of Service Message )
 */
void sendMessage(String _to, String _svc, BigInteger _sn, byte[] _msg) {
    onlyXCall();
    String channel = channels.get(caller).get(_to);

    if (_sn.compareTo(BigInteger.ZERO) < 0) {
        writeAcknowledgement(_to, _sn.negate(), _msg);
        return
    }

    seqNum = ibc.getNextSequenceSend(PORT, channel)
    packetfee = sendPacketFee.get(_to)
    _ackFee = 0;
    if (sn.compareTo(BigInteger.ZERO) > 0) {
        _ackFee = ackFee[_to]
        unclaimedAckFees[_to][seqNum] = _ackFee;
    }

    assert Context.value() == packetfee + _ackFee
    if (!_sn.equals(BigInteger.ZERO)) {
        outgoingPackets[channel][seqNum] = _sn;
    }

    Message msg = new Message(_sn, packetfee, _msg);
    Packet packet = Packet(
        port
        channel
        destinationPort[channel]
        destinationChannel[channel]
        msg.toBytes()
        getTimeoutHeight(channel)
    );



    ibc.sendPacket(packet);
}
```

```java
 private void writeAcknowledgement(String _to, BigInteger _sn, byte[] _msg) {
    String channel = channels.get(_to);
    byte[] packet = incomingPackets[channel][_sn]
    incomingPackets[channel][_sn] = null;

    ibc.writeAcknowledgement(packet, _msg);
}
```

### Receiving Messages
```java
public byte[] onRecvPacket(byte[] calldata, Address relayer) {
    onlyIBCHandler();

    Packet packet = Packet.decode(calldata);
    Message msg = Message.fromBytes(packet.getData());
    String nid = networkIds.get(packet.getDestinationChannel());
    assert nid != null;

    if (msg.getSn() == null) {
        Context.transfer(msg.getFee(), Address.fromBytes(msg.getData()))
        return  new byte[0]
    }

    if (msg.getSn() > 0) {
        incomingPackets[packet.getDestinationChannel()][msg.getSn()] = packet.getSequence());
    }

    unclaimedPacketFees[nid][relayer] += msg.getFee();
    xCall.handleMessage(nid, msg.getSn(), msg.getData());
    return new byte[0];
}
```

```java
public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
    onlyIBCHandler();
    Packet packet = Packet.decode(calldata);
    BigInteger sn = outgoingPackets[packet.getSourceChannel()][packet.getSequence()];
    outgoingPackets[packet.getSourceChannel()][packet.getSequence()] = null;
    String nid = networkIds[packet.getSourceChannel()];

    assert nid != null;
    assert sn != null;

    Context.transfer(unclaimedAckFees[_to][packet.sequence], relayer)
    unclaimedAckFees[nid][packet.sequence] = null

    xCall.handleMessage(nid, sn, acknowledgement);
}
```

```java
public void onTimeoutPacket(byte[] calldata, Address relayer) {
    onlyIBCHandler();
    Packet packet = Packet.decode(calldata);
    Message msg = Message.fromBytes(packet.getData());
    BigInteger sn = outgoingPackets.at(packet.getSourceChannel()).get(packet.getSequence());
    outgoingPackets.at(packet.getSourceChannel()).set(packet.getSequence(), null);
    String nid = networkIds[packet.getSourceChannel()];

    assert sn != null;

    fee = msg.getFee();
    if (sn != null) {
        fee += unclaimedAckFees[nid][packet.sequence];
        unclaimedAckFees[nid][packet.sequence] = null
        xCall.handleError(sn, -1, "Timeout");
    }

    Context.transfer(fee, relayer)
}
```

```java
private Height getTimeoutHeight(String channelId) {
    height = ibc.getLatestHeight(lightClients[channelId])
    return height + timeoutHeights[channelId]
}
```

### Fee logic
```java
/**
 * Gets the fee to the target network
 * _response should be true if it uses positive value for _sn of {@link #sendMessage}.
 * If _to is not reachable, then it reverts.
 * If _to does not exist in the fee table, then it returns zero.
 *
 * @param _to       String Network Id of target chain
 * @param _response Boolean ( Whether the responding fee is included )
 * @return Integer (The fee of sending a message to a given destination network )
 */
public BigInteger getFee(String _to, boolean _response) {
    fee = sendPacketFee[_to]
    if response {
        fee += ackFee[_to]
    }

    return fee
}
```
```java
/**
 * Claims fees gathered on 'nid' chain to specific address native to that chain.
 *
 * @param nid       String Network Id of target chain
 * @param address Byte representation of address on target chain
 */
public void claimFees(String nid, byte[] address) {
    relayer = Context.getCaller()
    BigInteger amount = unclaimedPacketFees[nid][relayer]
    unclaimedPacketFees[nid][relayer] = null
    assert amount > 0;
    String channel = channels.get(nid);
    Message msg = new Message(null, amount, address);
    Packet packet = Packet(
        port
        channel
        destinationPort[channel]
        destinationChannel[channel]
        msg
        getTimeoutHeight(channel)
    );
    ibc.sendPacket(packet);

}

public BigInteger getUnclaimedFees(String nid, Address relayer) {
    return  unclaimedPacketFees[nid][relayer]
}

```
### Channel setup

``` java
public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId, byte[] counterpartyPb, String version) {
    onlyIBCHandler();

    assert order == Undorderd

    // TODO verify version
    String connectionId = connectionHops[0]
    Counterparty counterparty = Counterparty.decode(counterpartyPb)
    String counterpartyPortId = counterparty.getPortId()
    String counterPartyNid = configuredNetworkIds[connectionId][counterpartyPortId]
    assert portId == PORT;
    assert counterPartyNid != null
    assert channels[counterPartyNid] == null
    lightClients[channelId] = configuredClients[connectionId]
    destinationPort[channelId] = counterpartyPortId
    channels[counterPartyNid] = channelId
    networkIds[channelId] = counterPartyNid
    timeoutHeights[channelId] = configuredTimeoutHeight[connectionId]
}
```

```java
public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
        byte[] counterpartyPb, String version, String counterpartyVersion) {
    onlyIBCHandler();

    assert order == Undorderd
    // TODO verify version

    String connectionId = connectionHops[0]
    Counterparty counterparty = Counterparty.decode(counterpartyPb)
    String counterpartyPortId = counterparty.getPortId()
    String counterPartyNid = configuredNetworkIds[connectionId][counterpartyPortId]
    assert portId == PORT;
    assert counterPartyNid != null
    assert channels[counterPartyNid] == null
    lightClients[channelId] = configuredClients[connectionId]
    destinationPort[channelId] = counterpartyPortId
    destinationChannel[channelId] = counterparty.getChannelId();
    channels[counterPartyNid] = channelId
    networkIds[channelId] = counterPartyNid
    timeoutHeights[channelId] = configuredTimeoutHeight[connectionId]
}
```
```java
public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId,
        String counterpartyVersion) {
    onlyIBCHandler();
    assert portId == PORT;

    destinationChannel[channelId] = counterpartyChannelId;
}
```

```java
public void onChanOpenConfirm(String portId, String channelId) {
    onlyIBCHandler();
    assert portId == PORT;
}
```

*Todo improve recovery without redeploying*
```java
public void onChanCloseInit(String portId, String channelId) {
    revert()
}
```
```java
public void onChanCloseConfirm(String portId, String channelId) {
    nid = networkIds[channelId]
    channels[nid] = ""
}
```

### Admin methods
```java
public void transferAdmin(Address admin) {
    onlyAdmin();
    this.admin = admin;
}
```

```java
/**
 * Configures a ibc connection so that a channel can be establised for a specigic nid


 * @param connectionId The connection id of the connection/chain to open the connection to
 * @param counterpartyPortId  The allocated port name of the connection on the counterparty chain
 * @param counterpartyNid The network Id to be associated with this connection
 * @param clientId The lightClient associated with this connection
 * @param timeoutHeight The timeoutheight to be used on packets, it is recommended to use a high value similar to the trusting period of the lightclient.
 */
public void configureConnection(String connectionId, String counterpartyPortId, String counterpartyNid, String clientId, BigInteger timeoutHeight) {
    onlyAdmin();
    assert configuredNetworkIds[connectionId][counterpartyPortId] == null
    assert channels[counterpartyNid] == null;
    configuredNetworkIds[connectionId][counterpartyPortId] = counterpartyNid
    configuredClients[connectionId] = clientId
    configuredTimeoutHeight[connectionId] = timeoutHeight
}
```

```java
void setFee(String nid, BigInteger packetFee, BigInteger ackFee) {
    onlyAdmin();
    //check chainn specfic limits
    sendPacketFee[nid] = packetFee
    ackFee[nid] = ackFee
}
```

## Security Considerations

After a connections is established for a specific networkId it can't change but if a connections is safe to use is up to each dapp to decide.


## Testing and Validation

## Implementations


## History


## Copyright


