## Introduction
This document outlines the ICS-20 workflow that supports ERC-20 Tokens instead of a consolidated Bank Module. This will help us to represent transferred tokens in proper ERC-20 wrapped assets.

### Logical Components

#### Token Contract: 
This will be standard ERC-20 token contract that will be deployed for each local asset or foreign wrapped asset.

#### ICS20 Contract: 
This contract will be entrypoint for sending and receiving tokens from foreign chains. It will also maintain a registry of tokens that are allowed to be transferred to and fro.Official ICS-20 Spec can be found [here](https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md). Following pseudocode illustrates the intended deviation from original ics-20 spec.

```js

function isNativeAsset(denom:String){
    return denom=="icx"
}

function getTokenContractAddress(denom:String):String {
   assert!(tokenContracts[denom]!=null)
   return tokenContracts[denom]
}
function sendFungibleTokens(
  denomination: string,
  amount: uint256,
  sender: string,
  receiver: string,
  sourcePort: string,
  sourceChannel: string,
  timeoutHeight: Height,
  timeoutTimestamp: uint64, // in unix nanoseconds
): uint64 {
    prefix = "{sourcePort}/{sourceChannel}/"
    // we are the source if the denomination is not prefixed
    source = denomination.slice(0, len(prefix)) !== prefix
    tokenContract=getTokenContracts(denomination)
    if source {
        if !isNativeAsset(denomination){
              // own contract address is to be used as escrow address
      escrowAccount = Context.getAddress()
      
      // escrow source tokens (assumed to fail if balance insufficient)
      tokenContract.Transfer(escrowAccount, amount)

        }
    
    } else {
      // receiver is source chain, burn vouchers
      tokenContract.Burn(sender, amount)
    }

    // create FungibleTokenPacket data
    data = FungibleTokenPacketData{denomination, amount, sender, receiver}

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
  if source {
    // receiver is source chain: unescrow tokens
    // determine escrow account
    denomOnly=data.denom.slice(len(prefix),len(data.denom.prefix));
    if isNativeAsset(denomOnly){
        Context.getAddress().transfer(data.receiver,data.amount)
    }
    tokenContract=getTokenContract(denomOnly)
    escrowAccount = Context.getAddress()
    // unescrow tokens to receiver (assumed to fail if balance insufficient)
    
      try {
            Context.call(tokenContract, "Transfer", data.receiver, data.amount)
        } catch (Exception e) {
           ack = FungibleTokenPacketAcknowledgement{false, "transfer coins failed"}
        }
    
  } else {
    prefix = "{packet.destPort}/{packet.destChannel}/"
    prefixedDenomination = prefix + data.denom
    tokenContract=getTokenContract(prefixedDenomination)
    // sender was source, mint vouchers to receiver (assumed to fail if balance insufficient)
    err = tokenContract.Mint(data.receiver, data.amount)
    if (err !== nil)
      ack = FungibleTokenPacketAcknowledgement{false, "mint coins failed"}
  }
  return ack
}


function refundTokens(packet: Packet) {
  FungibleTokenPacketData data = packet.data
  prefix = "{packet.sourcePort}/{packet.sourceChannel}/"
  // we are the source if the denomination is not prefixed
  tokenContract=getTokenContracts(data.denom)
  source = data.denom.slice(0, len(prefix)) !== prefix
  if source {
    // sender was source chain, unescrow tokens back to sender
    escrowAccount = Context.getAddress()
    tokenContract.Transfer(data.sender, data.amount)
  } else {
    // receiver was source chain, mint vouchers back to sender
    tokenContract.Mint(data.sender, data.denom, data.amount)
  }
}

function tokenFallback(self, _from: Address, _value: int, _data: bytes){
    return
}
```




