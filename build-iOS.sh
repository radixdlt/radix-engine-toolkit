#!/bin/sh -e

XCFRAMEWORKNAME=RadixEngineToolkit
XCFRAMEWORKPATH=./target/iOS

echo "🔮 ✨ Building $XCFRAMEWORKNAME...";

XCFRAMEWORK="$XCFRAMEWORKNAME.xcframework"

cd $(dirname $0)
script_parent=$PWD

echo "🔮 🗑 Removing any existing $XCFRAMEWORK..."
rm -rf $XCFRAMEWORKPATH
mkdir -p $XCFRAMEWORKPATH

echo "🔮 🛠 🎯 Building targets for all platforms...☑️"

echo "🔮 🦀  Switch rust to stable ⚖️"
rustup default stable

# iOS Simulator Aarch64 target
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-ios-sim \
    --release

# iOS Simulator x86 target
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-apple-ios \
    --release

# iOS iPhone Aarch64 target
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-ios \
    --release

# Apple Silicon Mac
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-darwin \
    --release

# Apple Intel Mac
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-apple-darwin \
    --release

RET_RUST_CRATE_NAME=radix-engine-toolkit

# replace "-" with "_"
LIB_NAME=`(echo $RET_RUST_CRATE_NAME | tr "-" "_")`

# MUST start with "lib", otherwise we get an error when running `swift test` in CLI (actually only via CLI)
# Xcode seems function without that prefix. But we want to be able to run tests from CLI. Cause is this strange
# https://github.com/apple/swift-package-manager/blob/main/Sources/Build/BuildPlan.swift#L2075 design decision
# However, it is ONLY the `.a` file which MUST start with "lib", it is fine to let the `.xcframework` be named
# "RadixEngineToolkit", it is also fine to let the module in `module.modulemap` to be `RadixEngineToolkit` and 
# thus the Swift `binaryTarget`. And also, header file does not need to be named "libradix_engine_toolkit.h", 
# the header can probably be named whatever actually, as long as it matches what is specified in the 
# `module.modulemap`. So the (Swift)EngineToolkit target will never ever see that the `.a` file is
# prefix with the name "lib". So all is well!
LIB_BINARY_REQUIRED_PREFIX=lib
TARGET_BINARY_NAME="$LIB_BINARY_REQUIRED_PREFIX$LIB_NAME.a"

# actually the same as `TARGET_BINARY_NAME`, but not semantically. 
# I.e. `cargo build` outputs binaries prefix with "lib", but will that always be the case?
LIB_BINARY_NAME="$LIB_BINARY_REQUIRED_PREFIX$LIB_NAME.a" 

HEADER_NAME="$LIB_NAME.h"
MODULEMAP_FILE_NAME=module.modulemap

echo "🔮 🛠 🎯 Finished building all targets ✅"

# Create a fat binary for mac (combining Intel Mac + Apple Silicon Mac)
(
    cd target
    mkdir -p macos-arm64_x86_64
    mkdir -p ios-simulator-arm64_x86_64

     # Combine two builds for the macOS archictures together into one fat file.
    lipo -create \
        aarch64-apple-darwin/release/$TARGET_BINARY_NAME \
        x86_64-apple-darwin/release/$TARGET_BINARY_NAME \
        -o macos-arm64_x86_64/$LIB_BINARY_NAME

    lipo -create \
        aarch64-apple-ios-sim/release/$TARGET_BINARY_NAME \
        x86_64-apple-ios/release/$TARGET_BINARY_NAME \
        -o ios-simulator-arm64_x86_64/$LIB_BINARY_NAME
        
    # Lipo is not needed for iOS, since we only support one architecture, and that is `aarch64` (ARM64 iOS).
    # And lipo is only used to combine different architectures for same platform together.
    
    mv aarch64-apple-ios/release/$TARGET_BINARY_NAME aarch64-apple-ios/release/$LIB_BINARY_NAME

	echo "🔮 🙏 Finished merging some of the targets using 'lipo'"
)

# Create the C header of the provided functions and adding it to the directory of each of the
# builds
(
	echo "🔮 🦀 Switch rust to nightly 🌓"
    rustup default nightly

    # Creating the header file
	echo "🔮 👤 Creating header file: '$HEADER_NAME'..."
    cbindgen \
        --lang c \
        --config cbindgen.toml \
        --crate $RET_RUST_CRATE_NAME \
        --output $HEADER_NAME

    echo "🔮 Making sure folder 'include' exists"
    rm -rf include
    mkdir include

    echo "🔮 👤 Copying header 'radix_engine_toolkit.h' to 'include' folder"
    cp $HEADER_NAME include

    echo "🔮 🗺 Creating and populating file $MODULEMAP_FILE_NAME"
    rm -rf $MODULEMAP_FILE_NAME
    touch $MODULEMAP_FILE_NAME
    
    # Populate the `module.modulemap` using variables declared above.
    tee -a $MODULEMAP_FILE_NAME << END
module $XCFRAMEWORKNAME {
    umbrella header "$HEADER_NAME"
    export *
}
END

    echo "🔮 🗺 Copying header 'module.modulemap' to 'include' folder"
    cp $MODULEMAP_FILE_NAME include

    echo "🔮 🗂 Copying 'include' folder for iOS target"
    cp -r include target/aarch64-apple-ios/release/

    echo "🔮 🗂 Copying 'include' folder for Mac target"
    cp -r include target/macos-arm64_x86_64/

    echo "🔮 🗂 Copying 'include' folder for iOS Sim target"
    cp -r include target/ios-simulator-arm64_x86_64/

    echo "🔮 🗑 Removing folder 'include'"
    rm -r include

    echo "🔮 🦀 Restore rust to stable ⚖️"
    rustup default stable
)

echo "🔮 📦 Creating '.xcframework' for platforms: [iOS, iOS Simulator, macOS] ☑️"

# iOS, iOS Sim, macOS
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/$LIB_BINARY_NAME \
    -headers target/aarch64-apple-ios/release/include \
    -library target/ios-simulator-arm64_x86_64/$LIB_BINARY_NAME \
    -headers target/ios-simulator-arm64_x86_64/include \
    -library target/macos-arm64_x86_64/$LIB_BINARY_NAME \
    -headers target/macos-arm64_x86_64/include \
    -output $XCFRAMEWORKPATH/$XCFRAMEWORK

echo "🔮 📦 Created '.xcframework' for platforms: [iOS, iOS Simulator, macOS] ✅"

rm $MODULEMAP_FILE_NAME
rm $HEADER_NAME