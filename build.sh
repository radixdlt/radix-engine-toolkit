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

# This script is used to build the Radix Engine Toolkits and the various crates (interfaces) which
# exist for it. For this script to run suceessfully, there are a few environment variables which
# need to be set
# 1. LLVM_BIN_PATH: The path to the currently installed LLVM toolchain for your machine. If you are 
#    on MacOS, then you can install this through homebrew: `brew install llvm`.
# 2. NDK_BIN_PATH: The path to Android's Native Development Kit which is needed to be able to build
#    the Radix Engine Toolkit for Android targets. Currently, the toolkit requires a minimum NDK 
#    version of 25 to be built.
# 3. MINGW_BIN_PATH: The path to the installation of the Mingw w64 toolchain. This is required to be
#    able to cross-compile the Radix Engine Toolkit to an x86_64 Windows target. 
# 4. LINUX_CROSS_BIN_PATH: The path to the installation of a linux cross compiler. The following is
#    an example of where you can install that: 
#    https://stackoverflow.com/questions/40424255/cross-compilation-to-x86-64-unknown-linux-gnu-fails-on-mac-osx
# 
# With these environment variables set, this script will utilize them to build the different crates 
# of the Radix Engine Toolkit

set -x
set -e

# The path of the directory that the script is in.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
# The name of the library that we are building
PACKAGE_NAME="radix-engine-toolkit"
# The package name after the - has been replaced with _
CLEANED_PACKAGE_NAME=$(echo "$PACKAGE_NAME" | tr "-" "_")
# The library name. By convention, this is `lib` concatenated with the package name
LIBRARY_NAME="lib$CLEANED_PACKAGE_NAME"

# Generates the CBindgen header for that specific target.
generate_cbindgen_header() {
    local target_triple=$1
    local crate_path=$2

    # Creating an include directory in the path of the target. This will store the header and the 
    # module map
    INCLUDE_DIRECTORY_PATH="$crate_path/target/$target_triple/release/include"
    mkdir $INCLUDE_DIRECTORY_PATH

    rustup default nightly
    CC="$LLVM_BIN_PATH/clang" AR="$LLVM_BIN_PATH/llvm-ar" cbindgen \
        --lang c \
        --config cbindgen.toml \
        --output "$INCLUDE_DIRECTORY_PATH/$LIBRARY_NAME.h" 
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

cargo_build() {
    local target_triple=$1
    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target $target_triple \
        --release
}

# =================================================
# Building the "radix-engine-toolkit-native" crate
# =================================================
(
    # The name of the crate that we are building
    CRATE_NAME="radix-engine-toolkit-native"

    # The path of the crate
    CRATE_PATH="$SCRIPT_DIR/$CRATE_NAME"
    
    cd $CRATE_PATH

    # Building the Radix Engine Toolkit for a `aarch64-apple-darwin` target
    TARGET_TRIPLE="aarch64-apple-darwin"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-apple-darwin` target
    TARGET_TRIPLE="x86_64-apple-darwin"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `aarch64-apple-ios-sim` target
    TARGET_TRIPLE="aarch64-apple-ios-sim"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `aarch64-apple-ios` target
    TARGET_TRIPLE="aarch64-apple-ios"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-apple-ios` target
    TARGET_TRIPLE="x86_64-apple-ios"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-pc-windows-gnu` target
    TARGET_TRIPLE="x86_64-pc-windows-gnu"
    
    export CC="$MINGW_BIN_PATH/x86_64-w64-mingw32-gcc"
    export AR="$MINGW_BIN_PATH/x86_64-w64-mingw32-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH

    # Building the Radix Engine Toolkit for a `x86_64-unknown-linux-gnu` target
    TARGET_TRIPLE="x86_64-unknown-linux-gnu"
    
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$LINUX_CROSS_BIN_PATH/x86_64-unknown-linux-gnu-gcc"
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
)

# =================================================
# Building the "radix-engine-toolkit-wasm" crate
# =================================================
(
    # The name of the crate that we are building
    CRATE_NAME="radix-engine-toolkit-wasm"

    # The path of the crate
    CRATE_PATH="$SCRIPT_DIR/$CRATE_NAME"
    
    cd $CRATE_PATH

    # Building the Radix Engine Toolkit for a `wasm32-unknown-unknown` target
    TARGET_TRIPLE="wasm32-unknown-unknown"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
)

# ==============================================
# Building the "radix-engine-toolkit-jni" crate
# ==============================================
(
    # The name of the crate that we are building
    CRATE_NAME="radix-engine-toolkit-jni"

    # The path of the crate
    CRATE_PATH="$SCRIPT_DIR/$CRATE_NAME"
    
    cd $CRATE_PATH

    # Building the Radix Engine Toolkit for a `aarch64-apple-darwin` target
    TARGET_TRIPLE="aarch64-apple-darwin"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-apple-darwin` target
    TARGET_TRIPLE="x86_64-apple-darwin"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `aarch64-apple-ios-sim` target
    TARGET_TRIPLE="aarch64-apple-ios-sim"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `aarch64-apple-ios` target
    TARGET_TRIPLE="aarch64-apple-ios"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-apple-ios` target
    TARGET_TRIPLE="x86_64-apple-ios"
    
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `x86_64-pc-windows-gnu` target
    TARGET_TRIPLE="x86_64-pc-windows-gnu"
    
    export CC="$MINGW_BIN_PATH/x86_64-w64-mingw32-gcc"
    export AR="$MINGW_BIN_PATH/x86_64-w64-mingw32-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH

    # Building the Radix Engine Toolkit for a `x86_64-unknown-linux-gnu` target
    TARGET_TRIPLE="x86_64-unknown-linux-gnu"
    
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$LINUX_CROSS_BIN_PATH/x86_64-unknown-linux-gnu-gcc"
    export CC="$LLVM_BIN_PATH/clang"
    export AR="$LLVM_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `aarch64-linux-android` target
    TARGET_TRIPLE="aarch64-linux-android"
    
    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK_BIN_PATH/aarch64-linux-android21-clang"
    export CC="$NDK_BIN_PATH/aarch64-linux-android21-clang"
    export AR="$NDK_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `armv7-linux-androideabi` target
    TARGET_TRIPLE="armv7-linux-androideabi"
    
    export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$NDK_BIN_PATH/armv7a-linux-androideabi19-clang"
    export CC="$NDK_BIN_PATH/armv7a-linux-androideabi19-clang"
    export AR="$NDK_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
    
    # Building the Radix Engine Toolkit for a `i686-linux-android` target
    TARGET_TRIPLE="i686-linux-android"
    
    export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="$NDK_BIN_PATH/i686-linux-android19-clang"
    export CC="$NDK_BIN_PATH/i686-linux-android19-clang"
    export AR="$NDK_BIN_PATH/llvm-ar"
    cargo_build $TARGET_TRIPLE

    generate_cbindgen_header $TARGET_TRIPLE $CRATE_PATH
    package_and_compress_build $TARGET_TRIPLE $CRATE_PATH
)

# =================
# Composite Builds
# =================

# Creating an XCFramework from the Apple builds
(
    # The name of the crate that we are building
    CRATE_NAME="radix-engine-toolkit-native"

    # The path of the crate
    CRATE_PATH="$SCRIPT_DIR/$CRATE_NAME"
    
    cd $CRATE_PATH

    # Creating the two directories where the temporary FAT libraries will be stored
    mkdir $CRATE_PATH/target/macos-arm64_x86_64/
    mkdir $CRATE_PATH/target/ios-simulator-arm64_x86_64

    # Creating the fat libraries
    lipo -create \
        "$CRATE_PATH/target/aarch64-apple-darwin/release/$LIBRARY_NAME.a" \
        "$CRATE_PATH/target/x86_64-apple-darwin/release/$LIBRARY_NAME.a" \
        -o "$CRATE_PATH/target/macos-arm64_x86_64/$LIBRARY_NAME.a"
    lipo -create \
        "$CRATE_PATH/target/aarch64-apple-ios-sim/release/$LIBRARY_NAME.a" \
        "$CRATE_PATH/target/x86_64-apple-ios/release/$LIBRARY_NAME.a" \
        -o "$CRATE_PATH/target/ios-simulator-arm64_x86_64/$LIBRARY_NAME.a"

    # Copying the "include" directory from its origin into the fat library directory
    cp -r $CRATE_PATH/target/aarch64-apple-darwin/release/include $CRATE_PATH/target/macos-arm64_x86_64/
    cp -r $CRATE_PATH/target/aarch64-apple-ios-sim/release/include $CRATE_PATH/target/ios-simulator-arm64_x86_64/

    # Creating the XC Framework
    xcodebuild -create-xcframework \
        -library "$CRATE_PATH/target/aarch64-apple-ios/release/$LIBRARY_NAME.a" \
        -headers "$CRATE_PATH/target/aarch64-apple-ios/release/include" \
        -library "$CRATE_PATH/target/macos-arm64_x86_64/$LIBRARY_NAME.a" \
        -headers "$CRATE_PATH/target/macos-arm64_x86_64/include" \
        -library "$CRATE_PATH/target/ios-simulator-arm64_x86_64/$LIBRARY_NAME.a" \
        -headers "$CRATE_PATH/target/ios-simulator-arm64_x86_64/include" \
        -output "$CRATE_PATH/target/RadixEngineToolkit.xcframework"

    # Deleting the temporary Fat libraries directories
    rm -rf $CRATE_PATH/target/macos-arm64_x86_64/
    rm -rf $CRATE_PATH/target/ios-simulator-arm64_x86_64
)

# ======================
# Aggregate and Collect
# ======================

BUILDS_DIRECTORY="$SCRIPT_DIR/build"
[[ -d $BUILDS_DIRECTORY ]] && rm -r $BUILDS_DIRECTORY
mkdir $BUILDS_DIRECTORY

for crate_name in "radix-engine-toolkit-jni" "radix-engine-toolkit-native" "radix-engine-toolkit-wasm"; 
do
    CRATE_PATH="$SCRIPT_DIR/$crate_name"
    TARGET_PATH="$CRATE_PATH/target"

    ARGUMENTS=""
    for path in $(find $TARGET_PATH \( -name "*.xcframework" -o -name "*.tar.gz" \) -maxdepth 3);
    do
        ARTIFACT_DIRECTORY_PATH=$(cd $(dirname $path); pwd)
        ARTIFACT_FILE_NAME=$(basename $path)

        ARGUMENTS+=" -C $ARTIFACT_DIRECTORY_PATH $ARTIFACT_FILE_NAME "
    done
    gtar -czf "$BUILDS_DIRECTORY/$crate_name.tar.gz" $ARGUMENTS
done