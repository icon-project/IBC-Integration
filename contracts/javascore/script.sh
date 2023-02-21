#!/usr/bin/env bash

txHash=$(goloop rpc sendtx call \
    --uri http://localhost:9082/api/v3 \
    --nid 3 \
    --step_limit 1000000000\
    --to cx908bc5bf68c7f2d20de84e26d271abe0e0093bff \
    --method sendEvent \
    --key_store  ~/keystore/godWallet.json \
    --key_password gochain | jq -r .)
echo $txHash
sleep 2
goloop debug trace --uri http://localhost:9082/api/v3d $txHash | jq -r .
