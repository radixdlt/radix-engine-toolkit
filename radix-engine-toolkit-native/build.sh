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

CRATE_NAME=`(echo "radix-engine-toolkit" | tr "-" "_")`

LIBRARY_NAME="lib$CRATE_NAME"
LIBRARY_FILE_NAME="$LIBRARY_NAME.a"
HEADER_FILE_NAME="$LIBRARY_NAME.h"

CARGO_TARGETS="aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios"
CROSS_TARGETS="x86_64-pc-windows-gnu x86_64-unknown-linux-gnu"

# Building for the desired targets
echo "Building the library";
for target in $CARGO_TARGETS
do
    echo "Building for target '$target'"

    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target $target \
        --target-dir ./target \
        --release
done

# for target in $CROSS_TARGETS
# do
#     echo "Building for target '$target'"

#     cross +nightly build \
#         -Z build-std=std,panic_abort \
#         -Z build-std-features=panic_immediate_abort \
#         --target $target \
#         --target-dir ./target \
#         --release
# done

# Creating a C-header and copying it to the directory of all of our build targets
echo "Generating the C Header"
rustup default nightly
cbindgen \
    --clean --lang c \
    --config cbindgen.toml \
    --output $HEADER_FILE_NAME
rustup default stable

# Combining the iOS and MacOS builds into a fat binary (required for XC Framework)
echo "Creating fat libraries"
(
    cd target
    mkdir -p macos-arm64_x86_64
    mkdir -p ios-simulator-arm64_x86_64

    echo "Creating a fat MacOS library"    
    lipo -create \
        aarch64-apple-darwin/release/$LIBRARY_FILE_NAME \
        x86_64-apple-darwin/release/$LIBRARY_FILE_NAME \
        -o macos-arm64_x86_64/$LIBRARY_FILE_NAME

    echo "Creating a fat iOS simulator library"    
    lipo -create \
        aarch64-apple-ios-sim/release/$LIBRARY_FILE_NAME \
        x86_64-apple-ios/release/$LIBRARY_FILE_NAME \
        -o ios-simulator-arm64_x86_64/$LIBRARY_FILE_NAME
)

echo "Creating the include directory"
mkdir ./include
mv $HEADER_FILE_NAME include
echo "module RadixEngineToolkit {
    umbrella header \"$HEADER_FILE_NAME\"
    export *
}" > ./include/module.modulemap

echo "Copying the include dir to the targets"
for target in "$CARGO_TARGETS $CROSS_TARGETS"
do
    cp -r ./include ./target/$target/release
done
cp -r include ./target/macos-arm64_x86_64
cp -r include ./target/ios-simulator-arm64_x86_64

mkdir target/iOS
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/$LIBRARY_FILE_NAME \
    -library target/ios-simulator-arm64_x86_64/$LIBRARY_FILE_NAME \
    -library target/macos-arm64_x86_64/$LIBRARY_FILE_NAME \
    -headers ./include \
    -output target/iOS/RadixEngineToolkit.xcframework

rm -rf include