#!/bin/bash
source .env

if [ $# -eq 0 ]; then
    echo "No arguments supplied: Pass --help for details"
elif [ "$1" == "--rpc-url" ]; then
    if [ $# -lt 2 ]; then
        echo "Missing RPC URL argument. Usage: $0 --rpc-url <RPC_URL>"
    else
        rpc_url="$2"
        if [ $# -eq 4 ] && [ "$3" == "--api-key" ]; then
            api_key="$4"
            echo "Deploying CallService.sol to $rpc_url with etherscan API key $api_key"
            forge script script/CallService.s.sol:CallServiceScript --fork-url $2 --broadcast --verify --etherscan-api-key $4
            echo "done"
        else
            echo "Deploying CallService.sol to $rpc_url"
            forge script script/CallService.s.sol:CallServiceScript --fork-url $2 --broadcast 
            echo "done"
        fi
    fi
else
    echo "Invalid arguments. Usage: $0 --rpc-url <RPC_URL> [--api-key <API_KEY>]"
fi
