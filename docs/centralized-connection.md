Centralized Connection
===

If you want a quick way to send cross chain messages using xcall, then you can use centralized-connection right away. It has extremely easy setup and can be done quickly.

This doc assumes you know what xcall is. So, if you're not familiar with xCall, learn more about it [here](https://www.xcall.dev/what-is-xcall)

xCall is deployed on [these](https://github.com/icon-project/xcall-multi/wiki#expansion) chains already, and plans to support more chains in the future. If you want xCall to be deployed on any chain, let us know by creating a [github issue](https://github.com/icon-project/xcall-multi/issues/new).

## What is centralized connection?

It is a design for a simple xCall connection between any two chains. This allows dapps to quickly establish Multi Protocol Verification on their xCall messages. This can also help prevent dapps from being exposed to bridge hacks but can also help with protocol hacks by filtering out abnormal transactions such as large token Transfers off chain, by adding some logic on the relay.

## Security Concerns
The main security concerns of this application is losing control of Owner privileges or using compromised RPC endpoints. To make it harder to hack RPC endpoints we can utilize the use of reference endpoint to verify the data received from the main endpoint.

If used with at least a second functioning bridge protocol the implications of this is that the filtering can be assumed to not have any effect anymore and any transaction from the other bridges can go through. This means if a hacker who wants full control over the protocol needs to break the other bridges along with breaking this relayer. 

## Process To Setup Centralized Connection for your dApp
1. Deploy connection contract on the source chain and destination chains. It takes relayer address and xCall address as parameter. You can get the relevant xCall address from the [xcall wiki](https://github.com/icon-project/xcall-multi/wiki/xCall-Deployment-Info). The guide to deploy the centralized connections is [here](./centralized-connection-setup.md)

2. Setup [centralized-relayer](https://github.com/icon-project/centralized-relay)

After this, you should have a centralized connection ready, which is able to send cross chain message to interact with your application on the destination chain. 

