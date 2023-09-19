#!/bin/bash
set -e
mkdir -p artifacts/icon
mkdir -p artifacts/archway

LOCAL_X_CALL_REPO="xcall-multi"

clone_xCall_multi() {
  echo "Cloning xCall-multi repo..."
  X_CALL_BRANCH="${1:-main}"

  LOCAL_X_CALL_REPO_VC_DIR="$LOCAL_X_CALL_REPO/.git"

  if [ ! -d "$LOCAL_X_CALL_REPO_VC_DIR" ]; then
    git clone -b "$X_CALL_BRANCH" --single-branch "https://github.com/icon-project/xcall-multi.git" "$LOCAL_X_CALL_REPO"
    cd "$LOCAL_X_CALL_REPO"
  else
    cd "$LOCAL_X_CALL_REPO"
    git checkout "$X_CALL_BRANCH"
    git pull origin "$X_CALL_BRANCH"
  fi
}

build_xCall_contracts() {
  echo "Generating optimized cosmwasm/jar of xCall contracts..."
  clone_xCall_multi "${1:-main}"
  ./scripts/optimize-cosmwasm.sh
  ./scripts/optimize-jar.sh
  echo "$PWD"
  cp artifacts/archway/*.wasm ../artifacts/archway/
  cp artifacts/icon/*.jar ../artifacts/icon/
  cd -
}

if [ "$1" = "build" ]; then
  shift
  build_xCall_contracts "$@"
fi

