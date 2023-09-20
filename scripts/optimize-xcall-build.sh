#!/bin/bash
set -e
mkdir -p artifacts/icon
mkdir -p artifacts/archway

LOCAL_X_CALL_REPO=".xcall-multi"

clone_xCall_multi() {
  echo "Cloning xcall-multi repo..."
  X_CALL_BRANCH="${1:-main}"
  rm -rf "$LOCAL_X_CALL_REPO"
  git clone -b "$X_CALL_BRANCH" --single-branch "https://github.com/icon-project/xcall-multi.git" "$LOCAL_X_CALL_REPO"
}

build_xCall_contracts() {
  echo "Generating optimized cosmwasm/jar of xcall-multi contracts..."
  clone_xCall_multi "${1:-main}"
  cd "$LOCAL_X_CALL_REPO"
  ./scripts/optimize-cosmwasm.sh
  ./scripts/optimize-jar.sh
  cp artifacts/archway/*.wasm ../artifacts/archway/
  cp artifacts/icon/*.jar ../artifacts/icon/
  cd -
}

if [ "$1" = "build" ]; then
  shift
  build_xCall_contracts "$@"
fi
