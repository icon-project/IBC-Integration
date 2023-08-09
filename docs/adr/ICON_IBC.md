
## Summary
This document describes the changes introduced to the [IBC Spec](https://github.com/cosmos/ibc) in order to implement IBC for ICON in Java smart contracts.
Currently only the Core componenets are implemented.

## Host changes
A host in IBC must provide a provable store defined as follows:

> Host state machines MUST provide two instances of this interface - a provableStore for storage read by (i.e. proven to) other chains, provableStore.set('some/path', 'value').<br>
The provableStore:<br>
MUST write to a key/value store whose data can be externally proved with a vector commitment as defined in ICS 23.

On ICON we will utilize BTP blocks to post commitments as messages which can be proven by simple merkle proofs. For details on how BTP block proofs are made see : [ICON IBC Lightclient](https://github.com/icon-project/IBC-Integration/blob/main/docs/adr/ICON-lightclient.md).
To create the separation of path value, each message provable store action is in Java replaced by a BTP message consisting of the concatenation of Path and Value bytes.

```
// IBC
provableStore.set('some/path', 'value')
```
```
// Java
msg = join('some/path', 'value')
sendBTPMessage(msg)
```

## ICS-02
For the client each latest Consensus and the Client state should be stored in provable stores. However, in order to not emit new BTP block for every update to a lightclient on ICON we only emit these when relevant to a counterparty. Which is during connection establishment.

## ICS-03
For each change to the connection we also commit the latest client and consensus states. This puts all relevant information needed to establish a connection with a counterparty in the same BTP block.

```
func updateConnectionCommitment(connection):
    sendBTPMessage(join(clientKey, clientState));
    sendBTPMessage(join(consensusKey, latestConsensusState));
    sendBTPMessage(join(connectionKey, connection))
}
```

During connection establishment all self client validation is skipped, and will have to be done manually when opening a new channel.

## ICS-04
For BTP blocks and the ICON lightclient we have two restrictions:
- Non-membership-proofs are not possible.
- BTP block includes no timestamp.

To solve the non-membership-proofs we take the recommended approach given in the IBC spec:
>Since the verification method is designed to give complete control to client implementations, clients can support chains that do not provide absence proofs by verifying the existence of a non-empty sentinel ABSENCE value. Thus in these special cases, the proof provided will be an ICS-23 Existence proof, and the client will verify that the ABSENCE value is stored under the given path for the given height.

To achieve this we send only the key part as a message. Which can then be proven to have a empty value in the lightclient.

Since BTP blocks includes no timestamp there will be no way for the counterparty to getTimestampByHeight and it will therefore be impossible to timeout packets by timestamp. To prevent this Java IBC requires all packets to specify a timeout height.

To allow relay to efficiently query what packets have been sent we have added an additional queryable store containing each packet sequences mapped to the block height on which they are sent. This store is also pruned along with commitments after acknowledgements or timeouts.