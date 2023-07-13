
## Introduction

This specification document describes a IBC enabled client (verification algorithm) for ICON BTP-blocks



## Design Overview

The ICON lightclient operates by relying on BTP (Blockchain Transfer Protocol) blocks sourced from the ICON chain, resulting in the formation of a BTP lightclient. BTP blocks are comprised of specific sections of blocks present on the ICON chain, which have been authenticated by the ICON validator set, and solely contain pertinent information for a given lightclient. To achieve this functionality, ICON facilitates multiple BTP networks which are each associated with a unique networkId. Consequently, every lightclient will be linked to a distinct BTP network Id on the ICON blockchain.

In contrast to alternative lightclients, the BTP lightclient necessitates the consistent updating of each BTP block in a chronological sequence, beginning from the initial block chosen at the time of lightclient creation.

BTP blocks contain a message root, which represents the merkle root of all the messages incorporated in the block. During each block update, the lightclient retains the message root at the updated height, which can subsequently be utilized by IBC (Inter-Blockchain Communication) to establish proof of membership for particular Commitments at specific heights.

In the context of IBC, Commitments are storage commitments that verify the storage of a certain value at a specific path and height. In the BTP lightclient, we instead prove inclusion of the concatenation of the path and value at a particular height. This message is verified using the stored message root for the corresponding height.

- Core components and their interactions

## Architecture

- Detailed architecture diagram
- Explanation of each component

### Definitions
- `hash` is a generic collision-resistant hash function, and can easily be configured.


### Desired Properties

This specification must satisfy the client interface defined in ICS 2.


## Technical Specification


### Constant Identifiers

```typescript
// Id of blockchain originating the btp blocks
SRC_NETWORK_ID : string 
// The btp network type 
NETWORK_TYPE_ID : unit64
// The btp network id
NETWORK_ID : unit64
```


### Client state

The BTP lightclient state tracks  current validator set, latestNetworkSectionHash, trusting period, latest height, and a possible frozen height.


```typescript
interface ClientState {
  trustingPeriod: uint64
  frozenHeight: uint64
  maxClockDrift: uint64
  latestHeight: uint64
  networkId: u64,
  networkTypeId: u64,
  srcNetworkId: String,
  trust_level: TrustLevel
}
```

### Consensus state

The client tracks the messageRoot for each block update 

```typescript
interface ConsensusState {
  messageRoot: []byte,
  nextProofContextHash: []byte
}
```

### Headers


```typescript
interface BTPBlockHeader {
  mainHeight: uint64
  round: uint64
  nextProofContextHash: []byte
  networkSectionToRoot: []byte
  networkId: uint64
  updateNumber: uint64
  prev: []byte
  messagesRoot: []byte
  messageCount: uint64
  nextValidators: [][]byte
  currentValidators: [][]byte
  trustedHeight: uint64
}
```

```typescript
interface BlockUpdate {
  header: BTPBlockHeader,
  signatures: [][]byte
}
```
### Encodings
Constructing hashes used in the BTP Client is done by hashing the RLP encoded list of RLP encoded objects.

```typescript
function createNetworkSectionHash( header: BTPBlockHeader) internal pure returns (bytes32) {
    byte[][] networkSection = byte[][] {
        encodeUint(header.networkId),
        encodeUint((header.messageSn << 1) | (header.hasNextValidators ? 1 : 0)),
        header.prev != bytes32(0) ? encodeBytes(header.prev) : encodeNull(),
        encodeUint(header.messageCount),
        header.messageRoot != bytes32(0) ? encodeBytes(header.messageRoot): encodeNull(),
    }

    return hash(encodeList(networkSection));
}
```

```typescript
function createNetworkTypeSectionDecisionHash(
        header: BTPBlockHeader,
        srcNetworkId: string,
        networkType: uint64 
    ) : []byte  {
    byte[][] ntsd  = byte[][] {
        encodeString(srcNetworkId),
        encodeUint(networkType),
        encodeUint(header.mainHeight),
        encodeUint(header.round),
        encodeBytes(createNetworkTypeSectionHash(header))),
    }
    return hash(encodeList(ntsd));
}
```

```typescript
function createNetworkTypeSectionHash(header: BTPBlockHeader) : []byte {
    bytes[] memory nts = byte[][] {
        encodeBytes(header.nextProofContextHash);
        encodeBytes(getNetworkSectionRoot(header));
    }
    return hash(encodeList(nts));
}
```



### Proof specs

```typescript
function verifyBlockProof( 
        header: BTPBlockHeader,
        signatures : [][]byte,
        validators: [][]byte
    ) {
        votes = 0
        // create desicion hash from btp block header
        decision = createNetworkTypeSectionDecisionHash(header, SRC_NETWORK_ID, NETWORK_TYPE_ID)
        for i = 0; i < proof.signatures.length; i++) {
            //recover signer address and if it matchse valdiators[i] count the vote
            address signer = recoverSigner(proof.signatures[i])
            if signer == validators[i] {
                votes++
            }
        }
        // assert 2/3 of validators has signed the networkSectionDecision 
        assert(hasQuorumOf(validators.length, votes), Errors.ERR_UNKNOWN);
    }
}
```

```typescript
function verifyMembership(
        messageRoot: []byte,
        proof : MessageProof,
        path: []byte,
        value: []byte
    ) {
        calulatedRoot = MerkleRoot(proof.leftValues, concat(path,value), proof.rightValues)
        assert(calulatedRoot == messageRoot)
}
```


### `Misbehaviour`
 
The `Misbehaviour` type is used for detecting misbehaviour and freezing the client - to prevent further packet flow - if applicable.

TODO:
Potential soultiuon would be to verify a block at the same height but with a different message root.


### Client initialisation

Tendermint client initialisation requires a (subjectively chosen) latest consensus state, including the full validator set.

```typescript
function initialise(
        srcNetworkId : string,
        networkTypeId: uint64,
        trustingPeriod: uint64,
        maxClockDrift: uint64,
        header: BTPBlockHeader
    ) : ClientState {
    consensusState = { header.messageRoot , header.nextProofContextHash}
    SRC_NETWORK_ID = srcNetworkId
    NETWORK_TYPE_ID = networkTypeId
    NETWORK_ID = header.networkId
    provableStore.set("clients/{NETWORK_ID}/consensusStates/{header.mainHeight}", consensusState)
    return ClientState {
      trustingPeriod
      frozenHeight: null
      maxClockDrift
      latestHeight: header.mainHeight
      networkSectionHash: header.getNetworkSectionHash()
      validatorHash: header.nextValidators
    }
}
```

The client `latestClientHeight` function returns the latest stored height, which is updated every time a new (more recent) header is validated.

```typescript
function latestClientHeight(clientState: ClientState): Height {
  return clientState.latestHeight
}
```

### Validity Predicate

BTP client validity checking uses verifyBlockProof algorithm described in [Proof specs](###Proof-specs)
If the provided header is valid, the client state is updated & the newly verified commitment written to the store.

Verify validity of regular update to the client

```typescript
function verifyHeader(blockUpdate: BlockUpdate) {
    header = blockUpdate.header
    clientState = provableStore.get("clients/{CLIENT_ID}/clientState")
    consensusState=  provableStore.get("clients/{CLIENT_ID}/consensusStates/{header.trustedHeight}")

    // assert header has correct networkId
    assert(header.networkId == NETWORK_ID)

    assert(header.trustedHeight > header.mainHeight)
    
    // assert trusting period has not yet passed
    assert(header.trustedHeight - header.mainHeight < clientState.trustingPeriod)

    currentProofContextHash=get_proof_context_hash(header.currentValidators)

    // assert current validators hash matches trusted context hash
    assert(consensusState.nextProofContextHash==currentProofContextHash)

    // call the BTP block proof verification
    assert(verifyBlockProof(header, blockUpdate.signatures, header.currentValidators))
}
```

### Misbehaviour Predicate
TODO

### `UpdateState`

`UpdateState` will perform a regular update for the client. It will add a consensus state to the client store and update the latest height and networkSectionHash to the client state. If nextValidators are includes also replace the validator set.

```typescript
function updateState(header: BTPBlockHeader) {
    // only update the validator set if a new is present in the header
    if nextValidators != null {
      clientState.validators = header.nextValidators
    }

    // set latest height and networkSection and save the client state
    if header.mainHeight>clientState.latestHeight {
       clientState.latestHeight = header.mainHeight
    }
   
    provableStore.set("clients/{CLIENT_ID}/clientState", clientState)
    
    // create recorded consensus state, save it
    consensusState = ConsensusState{header.messageRoot,header.nextProofContextHeight}
    provableStore.set("clients/{CLIENT_ID}/consensusStates/{header.mainHeight}", consensusState)

    //TODO VERIFY NEED  
    {
            // these may be stored as private metadata within the client in order to verify
            // that the delay period has passed in proof verification
            provableStore.set("clients/{NETWORK_ID}/processedTimes/{header.mainHeight}", currentTimestamp())
            provableStore.set("clients/{NETWORK_ID}/processedHeights/{header.mainHeight}", currentHeight())
    }
}
```

### `UpdateStateOnMisbehaviour`
TODO

### Upgrades
TODO

### State verification functions

BTP client state verification functions check a Merkle proof against a previously validated Message root.


```typescript
function verifyMembership(
  clientState: ClientState,
  height: Height,
  delayTimePeriod: uint64,
  delayBlockPeriod: uint64,
  proof: MessageProof,
  path: []byte,
  value: []byte): Error {
    // check that the client is at a sufficient height
    assert(clientState.latestHeight >= height)
    // check that the client is unfrozen or frozen at a higher height
    assert(clientState.frozenHeight === null || clientState.frozenHeight > height)
    //TODO VERIFY NEED   
    {
        // assert that enough time has elapsed
        assert(currentTimestamp() >= processedTime + delayPeriodTime)
        // assert that enough blocks have elapsed
        assert(currentHeight() >= processedHeight + delayPeriodBlocks)
    }
    // fetch the previously verified message root & verify membership
    // Implementations may choose how to pass in the identifier
    // ibc-go provides the identifier-prefixed store to this method
    // so that all state reads are for the client in question
    messageRoot = provableStore.get("clients/{NETWORK_ID}/consensusStates/{height}")
    // verify that concatination of <path, value> exists in the message root. 
    if !verifyMembership(root, proof, path, value) {
      return error
    }
    return nil
}

function verifyNonMembership(
  clientState: ClientState,
  height: Height,
  delayTimePeriod: uint64,
  delayBlockPeriod: uint64,
  proof: MessageProof,
  path: CommitmentPath): Error {
    // check that the client is at a sufficient height
    assert(clientState.latestHeight >= height)
    // check that the client is unfrozen or frozen at a higher height
    assert(clientState.frozenHeight === null || clientState.frozenHeight > height)
    //TODO VERIFY NEED   
    {
        // assert that enough time has elapsed
        assert(currentTimestamp() >= processedTime + delayPeriodTime)
        // assert that enough blocks have elapsed
        assert(currentHeight() >= processedHeight + delayPeriodBlocks)

    }
    
    // fetch the previously verified message root & verify membership
    // Implementations may choose how to pass in the identifier
    // ibc-go provides the identifier-prefixed store to this method
    // so that all state reads are for the client in question
    messageRoot = provableStore.get("clients/{NETWORK_ID}/consensusStates/{height}")
    // verify that concatination of <path, ABSENCE> exists in the message root. 
    if !verifyMembership(root, proof, path) {
      return error
    }
    return nil
}
```

### Properties & Invariants


## Security Considerations

- Security measures implemented
- Potential threats and mitigations

## Testing and Validation

- Testing strategy
- Test cases and expected results
- Validation process

## Other Implementations

Repository for [BMV in solidity](https://github.com/icon-project/btp2-solidity/tree/main/bmv).

Repository for [BMV in java](https://github.com/icon-project/btp2-java/tree/main/bmv).

## History


## Copyright
