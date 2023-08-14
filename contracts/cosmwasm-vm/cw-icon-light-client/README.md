# Deviations from IBC Tendermint LightClient

This LightClient was meant to be used for [ICON-IBC Integration](https://github.com/icon-project/IBC-Integration). So, it is made to accomodate all the changes made on IBC Integration such that IBC can be established between ICON and Cosmos based chains.

The IBC Tendermint LightClient is originally available as module. One of the major deviations in ICON LightClient was to implement it under smart contract environment.

All other deviations can be found [here]().

## Changes
The major changes in Icon LightClient Implementation includes:

- [createClient](#createclient)
- [updateClient](#updateclient)
- [verifyNonMembership](#verifynonmembership)

## createClient
The IBC Tendermint LightClient sets the *type, clientState, consensusState, blockTimestamp* and *blockHeight* while creating client.

Our Implementation sets just the *clientState, consensusState* and *blockHeight*.

## updateClient
In case of IBC Tendermint LightClient, while updating client, the headers are verfied for adjacent or non-adjacent updates.

In our ICON LightClient relay can update non-adjacent blocks provided that it is within the trusting period and can provide a previous trusted height.It can also update older blocks upto certain time limit from latest updated height in clientstate.

Similarly, in our ICON LightClient implementation, we are not using the actual block headers, but the BTP headers where we just verify if the incoming BTP header height is always greater than the previous one. Then we store a mapping of BTP block height and actual block height as well as BTP block height and actual block timestamp.

In IBC Tendermint LightClient, the update is based on block height but in our ICON LightClient, the update is based on BTP block height and epoch.

## verifyNonMembership
Both IBC Tendermint Lightclient and ICON LightClient uses MerkelProof for verifying membership and non-membership.

In IBC Tendermint Lightclient, the verifyNonMembership verifies the absence of key. But in case of ICON LightClient, we need to verify that the value of key is empty.