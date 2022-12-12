#!/usr/bin/env bash

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at

#   http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

# This bash script is used to build a specific crate for a specific target triple given the custom
# linker, compiler, and archiver to use. The parameters are defined through environment variables
# which this script makes use of. The following is a list of the environment variables that this
# script requires:
# 
# 1. CRATE_NAME: This is the name of the crate to build (e.g.: radix-engine-toolkit-native)
# 2. TARGET_TRIPLE: The target to build the radix engine toolkit for (e.g. aarch64-apple-darwin)
# 3. CUSTOM_COMPILER: The custom compiler to use to use for this build. When unsure, set this to the
#    path of your current clang binary and try running this script (e.g. /usr/bin/clang)
# 4. CUSTOM_ARCHIVER: The custom archiver to use to use for this build. When unsure, set this to the
#    path of your current llvm-ar binary and try running this script (e.g. /usr/bin/llvm-ar)
# 5. CUSTOM_LINKER: The custom linker to use to use for this build. When unsure, do not set this 
#    variable to anything and try running this script. This variable should not be needed for all 
#    targets. 

set -x
set -e

# The path of the directory that this script is in.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# A function which builds the create the current working directory for the specified target triple.
cargo_build() {
    local target_triple=$1
    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target $target_triple \
        --release
}

# The environment variable that cargo uses to specify the linter is dependent on the target triple.
# So, we need to perform some actions to get to that environment variable.
export LINKER_ENVIRONMENT_VARIABLE_NAME="CARGO_TARGET_"$(echo $TARGET_TRIPLE | tr '[:lower:]' '[:upper:]' | sed 's/-/_/g')"_LINKER"

# Setting the `LINKER_ENVIRONMENT_VARIABLE_NAME` environment variable only if a custom linker was
# specified. Otherwise, there is no need to set this environment variable.
if [ ! -z "$CUSTOM_LINKER" ]
then
    export $LINKER_ENVIRONMENT_VARIABLE_NAME=$CUSTOM_LINKER
fi

# Setting the CC and AR environment variables to the specified custom compiler and archiver
export CC=$CUSTOM_COMPILER
export AR=$CUSTOM_ARCHIVER

# Go into the crate directory and run the build command
CRATE_DIRECTORY="$SCRIPT_DIR/$CRATE_NAME"
cd $CRATE_DIRECTORY
cargo_build $TARGET_TRIPLE