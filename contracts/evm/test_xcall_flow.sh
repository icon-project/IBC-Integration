#!/bin/bash
source .env

listen_events_and_execute() {
    local network=$1
    local topic="0x2cbc78425621c181f9f8a25fc06e44a0ac2b67cd6a31f8ed7918934187f8cc59" 

    if [ "$network" = "bsctest" ]; then
        api_endpoint="https://api-testnet.bscscan.com/api"
        api_key=${BSCSCAN_API_KEY}
        contract_address=${BSCTEST_XCALL}
    elif [ "$network" = "fuji" ]; then
        api_endpoint="https://testnet.snowtrace.io/api/evm/43113/etherscan/api"
        api_key=${SNOWSCAN_API_KEY}
        contract_address=${FUJI_XCALL}
    elif [ "$network" = "base_goerli" ]; then
        api_endpoint="https://api-goerli.basescan.org/api"
        api_key=${BASESCAN_API_KEY}
        contract_address=${BASE_GOERLI_XCALL}
    elif [ "$network" = "optimism_goerli" ]; then
        api_endpoint="https://goerli-optimism.etherscan.io/api"
        api_key=${OPTIMISMSCAN_API_KEY}
        contract_address=${OPTIMISM_GOERLI_XCALL}
    elif [ "$network" = "arbitrum_goerli" ]; then
        api_endpoint="https://api-goerli.arbiscan.io/api"
        api_key=${ARBITRUMSCAN_API_KEY}
        contract_address=${ARBITRUM_GOERLI_XCALL}
    elif [ "$network" = "optimism_sepolia" ]; then
        api_endpoint="https://sepolia-optimism.etherscan.io/api"
        api_key=${OPTIMISMSCAN_API_KEY}
        contract_address=${OPTIMISM_SEPOLIA_XCALL}
    elif [ "$network" = "arbitrum_sepolia" ]; then
        api_endpoint="https://api-sepolia.arbiscan.io/api"
        api_key=${ARBITRUMSCAN_API_KEY}
        contract_address=${ARBITRUM_SEPOLIA_XCALL}
    else
        echo "Unsupported network"
        return 1
    fi

    API_URL="$api_endpoint?module=proxy&action=eth_blockNumber&apikey=$api_key"

    CURRENT_BLOCK_HEX=$(curl -s $API_URL | jq -r '.result')

    # Converting the block number from hex to decimal
    CURRENT_BLOCK_DEC=$(printf "%d\n" $CURRENT_BLOCK_HEX)

    echo "Current Ethereum Block Number: $CURRENT_BLOCK_DEC"

    echo "Listening for events on $network $contract_address $api_endpoint $api_key"

while true; do
    logData=$(curl -s -X GET "$api_endpoint?module=logs&action=getLogs&fromBlock=$CURRENT_BLOCK_DEC&toBlock=latest&address=$contract_address&topic0=$topic&apikey=$api_key")
    # Check if the result is not empty
    if [ $(echo $logData | jq '.result | length') -gt 0 ]; then

        # Process the response
        topics=($(echo $logData | jq -r '.result[0].topics[]'))
        data=$(echo $logData | jq -r '.result[0].data')

        from=${topics[1]}
        to=${topics[2]}
        sn=${topics[3]}
        #This is just for the testing purpose, on real, we need to check the signature and indexed params as well

        _reqId_hex="${data:2:64}"
        req_id=$((16#${_reqId_hex}))

        #for this as well, since offset of bytes is 64
        data_hex="${data:66+64+64}"
        trimmed_data_hex=$(echo $data_hex | sed 's/0*$//')

        # Output the extracted values
        echo "From (hashed): $from"
        echo "To (hashed): $to"
        echo "Serial Number: $sn"
        echo "Request ID: $req_id"
        echo "Additional Data: $trimmed_data_hex"

        # Skip the loop after processing
        forge script WormholeTest -s "executeCall(uint256 req_id, bytes memory data, string memory chain)" $req_id $trimmed_data_hex $network --fork-url $network --broadcast
        break
    else
        echo "No new logs found."
    fi

    # Sleep for a while before checking again
    sleep 1
done

}

end_to_end_test() {
        local source=$1
        local destination=$2
        local fee=$3
        forge script WormholeTest -s "sendMessage(string memory chain1, string memory chain2, uint256 fee)" $source $destination $fee --fork-url $source --broadcast
        listen_events_and_execute $destination
}

# if [ $# -lt 2 ]; then
#     echo "Configure action requires exactly two parameters for chain\n"
#     echo "Usage: $0 <source_chain> <destination_chain> --transaction fee"
#     echo "Valid chains: fuji bsctest base_goerli optimism_goerli arbitrum_goerli"
# else
#     end_to_end_test $1 $2
# fi
source_chain="fuji"
dest_chain="bsctest"
transaction_fee=30000000000000000

while [ "$1" != "" ]; do
    case $1 in
        --fee ) shift
                if [[ $1 =~ ^[0-9]+$ ]]; then
                    transaction_fee=$1
                else
                    echo "Invalid fee value. Please provide a number."
                    exit 1
                fi
                ;;
        --src ) shift
                case $1 in
                    fuji|bsctest|base_goerli|optimism_sepolia|arbitrum_sepolia )
                        source_chain=$1
                        ;;
                    *)
                        echo "Invalid source chain. Valid chains: fuji bsctest base_goerli optimism_sepolia arbitrum_sepolia"
                        exit 1
                        ;;
                esac
                ;;
        --dest ) shift
                case $1 in
                    fuji|bsctest|base_goerli|optimism_sepolia|arbitrum_sepolia )
                        dest_chain=$1
                        ;;
                    *)
                        echo "Invalid destination chain. Valid chains: fuji bsctest base_goerli optimism_sepolia arbitrum_sepolia"
                        exit 1
                        ;;
                esac
                ;;
    esac
    shift
done

if [ "$source_chain" = "$dest_chain" ]; then
    echo "Source and destination chains cannot be the same."
    exit 1
fi

echo "Source Chain: $source_chain"
echo "Destination Chain: $dest_chain"
echo "Transaction Fee: $transaction_fee"
end_to_end_test $source_chain $dest_chain $transaction_fee

