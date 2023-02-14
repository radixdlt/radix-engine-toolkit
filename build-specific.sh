#!/usr/bin/env bash

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
# 
#   http://www.apache.org/licenses/LICENSE-2.0
# 
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
# 1. CRATE_NAME: This is the name of the crate to build (e.g.: native-json-interface)
# 2. TARGET_TRIPLE: The target to build the radix engine toolkit for (e.g. aarch64-apple-darwin)
# 3. CUSTOM_COMPILER: The custom compiler to use to use for this build. When unsure, set this to the
#    path of your current clang binary and try running this script (e.g. /usr/bin/clang)
# 4. CUSTOM_ARCHIVER: The custom archiver to use to use for this build. When unsure, set this to the
#    path of your current llvm-ar binary and try running this script (e.g. /usr/bin/llvm-ar)
# 5. CUSTOM_LINKER: The custom linker to use to use for this build. When unsure, do not set this 
#    variable to anything and try running this script. This variable should not be needed for all 
#    targets. 
# 6. FEATURES: A string of the features string to use for the build (e.g. `jni`)

set -x
set -e

# The path of the directory that this script is in.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Setting the features to default if none are provided
FEATURES=${FEATURES:="default"}

# A function which builds the create the current working directory for the specified target triple.
cargo_build() {
    local target_triple=$1
    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target $target_triple \
        --features $FEATURES \
        --release
}

# Generates the CBindgen header for that specific target.
generate_cbindgen_header() {
    local target_triple=$1
    local crate_path=$2

    # Creating an include directory in the path of the target. This will store the header and the 
    # module map
    INCLUDE_DIRECTORY_PATH="$crate_path/target/$target_triple/release/include"
    mkdir $INCLUDE_DIRECTORY_PATH

    rustup default nightly
    unset $LINKER_ENVIRONMENT_VARIABLE_NAME
    CC=$(which clang) AR=$(which llvm-ar) cbindgen \
        --lang c \
        --config cbindgen.toml \
        --output "$INCLUDE_DIRECTORY_PATH/libradix_engine_toolkit.h" 
    rustup default stable

    # Create a module map which links to the generated header in the include directory
    echo "module RadixEngineToolkit {" > "$INCLUDE_DIRECTORY_PATH/module.modulemap" 
    echo "  umbrella header \"libradix_engine_toolkit.h\"" >> "$INCLUDE_DIRECTORY_PATH/module.modulemap" 
    echo "  export *" >> "$INCLUDE_DIRECTORY_PATH/module.modulemap" 
    echo "}" >> "$INCLUDE_DIRECTORY_PATH/module.modulemap" 
}

# A function which is used to create a compressed zip file of the build artifacts for a given target
# and crate
package_and_compress_build() {
    local target_triple=$1
    local crate_path=$2

    (
        # The path where all of the build artifacts for this given crate and target triple can be found
        BUILD_PATH="$crate_path/target/$target_triple/release"
        cd $BUILD_PATH

        # Finding all of build artifacts which we want to zip up in a file
        BUILD_ARTIFACTS_PATH=$(find . -type f \( -name "*.a" -o -name "*.dylib" -o -name "*.dll" -o -name "*.so" -o -name "*.d" -o -name "*.wasm" \) -maxdepth 1)
        gtar -czf "./$target_triple.tar.gz" $BUILD_ARTIFACTS_PATH ./include
    )
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
CRATE_PATH="$SCRIPT_DIR/$CRATE_NAME"
cd $CRATE_PATH

cargo_build $TARGET_TRIPLE
generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
package_and_compress_build $TARGET_TRIPLE $CRATE_PATH