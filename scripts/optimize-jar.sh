#!/bin/bash
set -e

mkdir -p artifacts/icon

cd contracts/javascore
./gradlew clean optimizedJar
cd -

for jar in $(find . -type f -name "*optimized.jar" | grep  /build/libs/); do
  NAME=$(basename "$jar" .jar).jar
  echo "Creating intermediate hash for ${NAME}..."
  sha256sum -- "$jar" | tee -a artifacts/icon/checksums_intermediate.txt
  echo "Copying $NAME"
  cp "$jar" "artifacts/icon/$NAME"
  cp "$jar" "artifacts/icon/${NAME%%-[0-9]*.[0-9]*.[0-9]*-optimized.jar}-latest.jar"
done
