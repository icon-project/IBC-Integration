#!/bin/bash

ARCHWAY_KEY=$(archwayd keys export xcall_wallet --unsafe --unarmored-hex --keyring-backend test)
SUI_KEY=$(sui keytool convert $ARCHWAY_KEY --json | grep suiprivkey | awk -F\" '{print $4}')
rm -rf /root/.sui/sui_config/*
sui keytool import --alias sui_wallet_testnet $SUI_KEY ed25519 >/dev/null && echo "SUCCESS" || echo "FAILED"
