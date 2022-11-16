CRATE_NAME=`(echo "radix-engine-toolkit" | tr "-" "_")`

LIBRARY_NAME="lib$CRATE_NAME"
LIBRARY_FILE_NAME="$LIBRARY_NAME.a"
HEADER_FILE_NAME="$LIBRARY_NAME.h"

TARGETS="aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios x86_64-unknown-linux-gnu"

# Building for the desired targets
echo "Building the library";
for target in $TARGETS
do
    echo "Building for target '$target'"

    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target $target \
        --target-dir ./target \
        --release
done

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
mkdir include
mv $HEADER_FILE_NAME include
echo "module RadixEngineToolkit {
    umbrella header \"$HEADER_FILE_NAME\"
    export *
}" > ./include/module.modulemap

echo "Copying the include dir to the targets"
for target in $TARGETS
do
    cp -r include ./target/$target/release
done
cp -r include ./target/macos-arm64_x86_64
cp -r include ./target/ios-simulator-arm64_x86_64

rm -rf include

mkdir target/iOS
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/$LIBRARY_FILE_NAME \
    -headers target/aarch64-apple-ios/release/include \
    -library target/ios-simulator-arm64_x86_64/$LIBRARY_FILE_NAME \
    -headers target/ios-simulator-arm64_x86_64/include \
    -library target/macos-arm64_x86_64/$LIBRARY_FILE_NAME \
    -headers target/macos-arm64_x86_64/include \
    -output target/iOS/RadixEngineToolkit.xcframework