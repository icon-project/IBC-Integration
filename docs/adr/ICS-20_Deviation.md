## Introduction
This document outlines the ICS-20 workflow that supports IRC2 Tokens instead of a consolidated Bank Module. This will help us to represent transferred tokens in proper IRC2 wrapped assets.
This is an extension of [ICS20](https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md) specs to support IBC on ICON.

### Logical Components

#### Token Contract: 
This will be mintabe burnable IRC2 token contract that will be deployed for each local asset or foreign wrapped asset. We will be using the audited [IRC2 Tradeable Contract](https://github.com/icon-project/icon-bridge/tree/main/javascore/irc2Tradeable) from the ICON Bridge repo

#### ICS20 Contract: 
This contract will be entrypoint for sending and receiving tokens from foreign chains. It will also maintain a registry of tokens that are allowed to be transferred to and fro. Official ICS-20 specs can be found [here](https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md). 

Any arbitrary tokens from the COSMOS chains will not be allowed to transfer to ICON. Only registered tokens will be allowed to do so. The ICS20 Contract will have an admin, which can register tokens of COSMOS chains on ICON. Once registered, those tokens can be minted on ICON by the ICS20 contract. The name of the token MUST be the corresponding denom from centauri chain. 

**Cross chain tokens to be supported**

If `ARCH` token is to be bridged from Archway to ICON, it has to go through Archway -> Centauri -> Icon.

Denom of ARCH on centauri: `transfer/channel-X/ARCH`

Denom of ARCH on icon: `transfer/channel-ICON/transfer/channel-X/ARCH`.

The name of the token to register MUST be `transfer/channel-ICON/transfer/channel-X/ARCH`. 

If ARCH token was transfered to another cosmos chain, neutron, and if that needs to be sent to ICON, it MUST go back to Archway chain, then be sent to ICON via centauri. Only the denoms from their native chain will be supported.

#### Token register
```js
function registerCosmosToken(name: String, symbol: String, decimals: int) {
  onlyAdmin()
  tokenAddress = deployIRC2Tradeable(name, symbol, decimals)
  tokenContracts[name] = tokenAddress
}
```

The following function will be used to register tokens on ICON to the ICS 20 App.

```js
function registerIconToken(tokenAddress: Address) {
  onlyAdmin()
  name = Context.call(tokenAddress, "name")
  tokenContracts[name] = tokenAddress
}
```

#### Helper methods
```js
function isNativeAsset(denom:String){
    return denom=="icx"
}

function getTokenContractAddress(denom:String):String {
   assert(tokenContracts[denom]!=null)
   return tokenContracts[denom]
}
```

#### Sending Tokens
- To send ICX, send using the `sendFungibleTokens` function. 
- To send tokens other then ICX, it should go through `tokenFallback` function. The `data` bytes, should be parsed into the following structure.
  ```json
  {
    "method": "sendFungibleTokens",
    "params": {
      "denomination": "string",
      "amount": "uint64",
      "sender": "string",
      "receiver": "string",
      "sourcePort": "string",
      "sourceChannel": "string",
      "timeoutHeight": {
        "latestHeight": "uint64",
        "revisionNumber": "uint64",
      },
      "timeoutTimestamp": "uint64",
      "memo":"string"
    }
  }
  ```
- Implementation of `tokenFallback` and `sendFungibleTokens`
```js
// to send tokens other than icx
function tokenFallback(from: Address, value: uint64, data: bytes) {
  data = parseStructure(data)
  if data.method == "sendFungibleTokens" {
    sendFungibleToken  = parseFungibleToken(data.params)
    assert(sendFungibleToken.amount == value)
    assert(sendFungibleToken.sender == from)
    sendFungibleTokens(...)
  } else {
    revert("wrong data")
  }
}

@payable
function sendFungibleTokens(
  denomination: string,
  amount: uint256,
  sender: string,
  receiver: string,
  sourcePort: string,
  sourceChannel: string,
  timeoutHeight: Height,
  timeoutTimestamp: uint64, // in unix nanoseconds
  @Optional memo: string,
): uint64 {
    prefix = "{sourcePort}/{sourceChannel}/"
    // we are the source if the denomination is not prefixed
    source = denomination.slice(0, len(prefix)) !== prefix
    tokenContract=getTokenContracts(denomination)
    if source {
      if isNativeAsset(denomination) {
        assert amount == Context.getValue()
      }
    }
    if !source {
      tokenContract.burn(amount);
    }

    // create FungibleTokenPacket data
    data = FungibleTokenPacketData{denomination, amount, sender, receiver, memo}

    // send packet using the interface defined in ICS4
    sequence = handler.sendPacket(
      getCapability("port"),
      sourcePort,
      sourceChannel,
      timeoutHeight,
      timeoutTimestamp,
      json.marshal(data) // json-marshalled bytes of packet data
    )

    return sequence
}

```

#### Receiving tokens

```js

function onRecvPacket(packet: Packet) {
  FungibleTokenPacketData data = packet.data
  assert(data.denom !== "")
  assert(data.amount > 0)
  assert(data.sender !== "")
  assert(data.receiver !== "")

  // construct default acknowledgement of success
  FungibleTokenPacketAcknowledgement ack = FungibleTokenPacketAcknowledgement{true, null}
  prefix = "{packet.sourcePort}/{packet.sourceChannel}/"
  // we are the source if the packets were prefixed by the sending chain
  source = data.denom.slice(0, len(prefix)) === prefix
  assert data.receiver is Address
  if source {
    // receiver is source chain: unescrow tokens
    // determine escrow account
    denomOnly=data.denom.slice(len(prefix),len(data.denom.prefix));
    if isNativeAsset(denomOnly){
      Context.transfer(data.receiver, data.amount)
    }
    tokenContract=getTokenContract(denomOnly)
    // unescrow tokens to receiver (assumed to fail if balance insufficient)
    try {
      tokenContract.transfer(data.receiver,data.amount)
    } catch (Exception e) {
      ack = FungibleTokenPacketAcknowledgement{false, "transfer coins failed"}
    }
  } else {
    prefix = "{packet.destPort}/{packet.destChannel}/"
    prefixedDenomination = prefix + data.denom
    tokenContract=getTokenContract(prefixedDenomination)
    try {
      // sender was source, mint vouchers to receiver (assumed to fail if balance insufficient)
      tokenContract.mint(data.receiver, data.amount)
    } catch (Exception e) {
      ack = FungibleTokenPacketAcknowledgement{false, "mint coins failed"}
    }
  }
  return ack
}
```

#### Acknowledge Packet
```js
function onAcknowledgePacket(
  packet: Packet,
  acknowledgement: bytes) {
  // if the transfer failed on dst chain, refund the tokens
  if (!acknowledgement.success)
    refundTokens(packet)
}
```
#### Timeout Packet
```js
function onTimeoutPacket(packet: Packet) {
  // the packet timed-out, so refund the tokens
  refundTokens(packet)
}
```
#### Refund logic
```js

function refundTokens(packet: Packet) {
  FungibleTokenPacketData data = packet.data
  prefix = "{packet.sourcePort}/{packet.sourceChannel}/"
  // we are the source if the denomination is not prefixed
  tokenContract=getTokenContracts(data.denom)
  source = data.denom.slice(0, len(prefix)) !== prefix
  if source {
    // sender was source chain, unescrow tokens back to sender
    if isNativeAsset {
      Context.transfer(data.sender, data.amount)
      return
    } 
    tokenContract.transfer(data.sender, data.amount)
  } else {
    // receiver was source chain, mint vouchers back to sender
    tokenContract.mint(data.sender,data.amount)
  }
}

function tokenFallback(self, _from: Address, _value: int, _data: bytes){
    return
}
```

### Hopchain
Hop is done based on the memo field during sendFungibleTokens. The structure of memo should follow the following [spec](https://github.com/cosmos/ibc-apps/tree/main/middleware/packet-forward-middleware)

