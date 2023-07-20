#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

CRATE_MANIFEST_FILE_PATH="$SCRIPT_DIR/../radix-engine-toolkit-uniffi/Cargo.toml"
RET_VERSION=$(grep -m 1 version $CRATE_MANIFEST_FILE_PATH | awk -F= '{print $2}' | tr -d '" ')
COMMIT_HASH=$(git rev-parse --short HEAD)
VERSION="$RET_VERSION-commit-$COMMIT_HASH"

echo $VERSION