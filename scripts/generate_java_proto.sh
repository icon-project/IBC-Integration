#!/bin/bash

cd proto
buf export buf.build/protocolbuffers/wellknowntypes -o ./
buf export buf.build/cosmos/ibc -o ./
buf export buf.build/cosmos/ics23:c7c728879896fb260fe76b208ea6a17c2b0132a3 -o ./
buf export buf.build/tendermint/tendermint -o ./
sed -i -e 's/  .tendermint/  tendermint/g' ibc/lightclients/tendermint/v1/tendermint.proto

cd ../contracts/javascore
./gradlew proto-util:generate

rm -r proto-lib/src/main/java/*
cp -r proto-util/build/generated/sources/cosmos proto-lib/src/main/java/
cp -r proto-util/build/generated/sources/google proto-lib/src/main/java/
cp -r proto-util/build/generated/sources/ibc proto-lib/src/main/java/
cp -r proto-util/build/generated/sources/icon proto-lib/src/main/java/
cp -r proto-util/build/generated/sources/tendermint proto-lib/src/main/java/
rm proto-lib/src/main/java/google/protobuf/FileDescriptorSet.java

cd ../../proto
rm -r amino
rm -r capability
rm -r cosmos_proto
rm -r google
rm -r cosmos_proto
rm -r gogoproto
rm -r ibc
rm -r tendermint
