#!/bin/bash
set -e

mkdir -p artifacts/icon

cd contracts/javascore
./gradlew clean optimizedJar
cd -

cd xCall/contracts/javascore
./gradlew clean optimizedJar
cd -
for jar in $(find . -type f -name "*optimized.jar" | grep  /build/libs/); do
  NAME=$(basename "$jar" .jar)${SUFFIX}.jar
  echo "Creating intermediate hash for ${NAME}..."
  sha256sum -- "$jar" | tee -a artifacts/icon/checksums_intermediate.txt
  echo "Copying $NAME ..."
  cp "$jar" "artifacts/icon/$NAME"
done
