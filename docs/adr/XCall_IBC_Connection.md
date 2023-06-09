# XCall IBC Connection


## Introduction

This specification document describes a IBC enabled xCall connection


## General Connection Design Overview
The provided code snippet demonstrates a specific behavior of a connection. It consists of two external functions: sendMessage(targetNetwork, svc, sn, msg) and getFee(network, response).

The sendMessage function is responsible for sending a message to a specified targetNetwork. It accepts four parameters: targetNetwork, svc, sn, and msg.

The behavior of sendMessage depends on the value of sn (sequence number). If sn is greater than 0, it indicates a new message that requires a response. In this case, both the sending fee and the response fee should be included. If sn is 0, it signifies a one-way message where no response is expected. If sn is less than 0, it implies that the message is a response to a previously received message. In this scenario, no fee is included in the sending message since it should have already been paid when the positive sn was sent.

After handling the sn value, the sendMessage function triggers the handleBTPMessage function on the targetNetwork. It passes targetNetwork, svc, sn, and msg as arguments to handleBTPMessage. The purpose of this function is to handle the incoming message on the specified targetNetwork's xCall contract.

In case the message fails to be delivered for any reason, the code triggers the handleBTPError function. It passes an empty string as the first argument, svc, sn, errorCode, and errorMessage to the handleBTPError function. The responsibility of this function is to handle errors that occur during the message delivery process.

The second external function, getFee(network, response), calculates and returns the fee required to send a message to the specified network and back. It takes into account the optional response parameter when determining the fee.

In summary, this code snippet illustrates a specific behavior expected from a connection regarding message sending and error handling.
``` javascript
external function sendMessage(targetNetwork, svc, sn, msg)
    if sn < 0:
        sn = sn.negate()
    On targetNetwork, trigger handleBTPMessage(targetNetwork, svc, sn, msg)
    if message fails to deliver:
        trigger handleBTPError("", svc, sn, errorCode, errorMessage)
```

``` javascript
external function getFee(network, response)
    Returns the fee required to send a message to "network" and back, considering the optional response parameter.
```

## General IBC Connection Design Overview
The IBC (Inter-Blockchain Communication) connection facilitates communication between different chains using IBC channels and ports. This connection relies on the administrator to associate network IDs with specific connections/ports. Once established, these properties become immutable and cannot be changed. Consequently, users can depend on the stability and reliability of a configured connection to another chain.

For message transmission, the sendPacket function is utilized. All responses are handled through acknowledgments, ensuring reliable delivery. If a packet timeout occurs, it will be treated as an error, indicating a failure in the message transmission process.

Two types of fees are associated with this connection. The first fee pertains to delivering the packet itself, while the second fee is related to delivering an acknowledgment. These fee amounts are determined and controlled by the administrator. To ensure user safety, there are caps on the fee amounts, preventing excessive charges.

Additionally, the packet fee rewards are included with the message and stored on the counterparty connections. These rewards can be claimed separately through a distinct message, providing a mechanism for managing and distributing the associated fees.

## Technical Specification

### Storage
```
TIMEOUT_HEIGHT = <Some high fixed value>
configuredNetworkIds : connection->port->NetworkId

PORT: portId
channels : NetworkId -> channelId
networkIds : channelId -> NetworkId
destinationChannel: channelId -> counterPartyChannelId
destinationPort: channelId -> counterPartyPortId

incomingPackets: channel -> sn -> packetSequence
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
    String sn;
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
    if (_sn.compareTo(BigInteger.ZERO) < 0) {
        writeAcknowledgement(_to, _sn.negate(), _msg);
        return
    }
    packetfee = sendPacketFee.get(_to)
    _ackFee = 0;
    if (sn.compareTo(BigInteger.ZERO) > 0) {
        _ackFee = ackFee[_to]
    }

    assert Context.value() == packetfee + _ackFee

    String channel = channels.get(_to);
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
        TIMEOUT_HEIGHT
    );

    unclaimedAckFees[_to][packet.sequence] = _ackFee;

    ibc.sendPacket(packet);
}
```

```java
 private void writeAcknowledgement(String _to, BigInteger _sn, byte[] _msg) {
    String channel = channels.get(_to);
    BigInteger sequenceNumber = incomingPackets[channel][_sn]
    incomingPackets[channel][_sn] = null;

    Packet packet = new Packet();
    packet.setSequence(sequenceNumber);
    packet.setDestinationPort(PORT);
    packet.setDestinationChannel(channel);

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
    unclaimedPacketFees[nid][relayer] += msg.getFee();

    if (msg.getSn() > 0) {
        incomingPackets[packet.getDestinationChannel()][msg.getSn()] = packet.getSequence();
    } else if (msg.getSn() == -1) {
         Context.transfer(msg.getFee(), Address.fromBytes(msg.getData()))
         return  new byte[0]
    }

    xCall.handleBTPMessage(nid, "xcall", msg.getSn(), msg.getData());
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

    xCall.handleBTPMessage(nid, "xcall", sn, acknowledgement);
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

    fee = msg.getFee();
    if (sn != null) {
        fee += unclaimedAckFees[nid][packet.sequence];
        unclaimedAckFees[nid][packet.sequence] = null
        xCall.handleBTPError("", "xcall", sn, -1, "Timeout");
    }

    Context.transfer(fee, relayer)
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
 * @param _to       String ( BTP Network Address of the destination BMC )
 * @param _response Boolean ( Whether the responding fee is included )
 * @return Integer (The fee of sending a message to a given destination network )
 */
public BigInteger getFee(String _to, boolean _response) {
    fee = 0
    if response {
        fee = ackFee[_to]
    }

    return sendPacketFee[_to] + fee
}

public void claimFees(String nid, byte[] address) {
    relayer = getCaller()
    BigInteger amount = unclaimedPacketFees[nid][relayer]
    assert amount > 0;
    String channel = channels.get(nid);
    Message msg = new Message(-1, amount, address);
    Packet packet = Packet(
        port
        channel
        destinationPort[channel]
        destinationChannel[channel]
        msg
        TIMEOUT_HEIGHT
    );
    ibc.sendPacket(packet);

}

```
### Channel setup

``` java
public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId, byte[] counterpartyPb, String version) {
    onlyIBCHandler();

    assert order == Undorderd

    // TODO verify version
    Counterparty counterparty = Counterparty.decode(counterpartyPb);
    String counterpartyPortId = counterparty.getPortId()
    String counterPartyNid = configuredNetworkIds[connectionHops[0]][counterpartyPortId]
    assert portId == PORT;
    assert counterPartNid != null
    assert channels[counterPartyNid] == null
    destinationPort[channelId] = counterpartyPortId
    channels[counterPartyNid] = channelId
    networkIds[channelId] = counterPartyNid
}
```

```java
public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
        byte[] counterpartyPb, String version, String counterpartyVersion) {
    onlyIBCHandler();

    assert order == Undorderd
    // TODO verify version

    Counterparty counterparty = Counterparty.decode(counterpartyPb);
    String counterpartyPortId = counterparty.getPortId()
    String counterPartyNid = configuredNetworkIds[connectionHops[0]][counterpartyPortId]
    assert portId == PORT;
    assert counterPartNid != null
    assert channels[counterPartyNid] == null

    destinationPort[channelId] = counterpartyPortId
    channels[counterPartyNid] = channelId
    destinationChannel[channelId] = counterparty.getChannelId();
    networkIds[channelId] = counterPartyNid
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

Once a channel is closed, no further action is required. By assigning an empty string to the channel ID, the IBC system ensures that no messages will be processed through that specific channel. This effectively halts any communication through the closed channel. However, timeouts are still implemented to allow for the recovery of lost messages.

Setting the channel ID to an empty string also serves another important purpose: it prevents the administrator from replacing the connection. With an empty channel ID, any attempt to modify or replace the existing connection is prohibited. Instead, to establish a new connection with the desired chain, a new contract must be deployed. This ensures the stability and integrity of the established connection,

*Todo improve recovery without redeploying*
```java
public void onChanCloseInit(String portId, String channelId) {
    nid = networkIds[channelId]
    channels[nid] = ""
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
public void configureConnection(String connectionId, String portId, String counterpartyNid) {
    onlyAdmin();
    assert configuredNetworkIds[connectionId][portId] == null
    assert channels[counterpartyNid] == null;
    configuredNetworkIds[connectionId][portId] = counterpartyNid
}
```

```java
void setFee(String _to, BigIntger packetFee, BigIntger ackFee) {
    onlyAdmin();
    //check chainn specfic limits
    sendPacketFee[_to] = packetFee
    ackFee[_to] = ackFee
}
```

## Security Considerations

After a connections is established for a specific networkId it can't change but if a connections is safe to use is up to each dapp to decide.


## Testing and Validation

## Implementations

## History

## Copyright


