CRATE_NAME=`(echo "radix-engine-toolkit" | tr "-" "_")`

LIBRARY_NAME="lib$CRATE_NAME"
LIBRARY_FILE_NAME="$LIBRARY_NAME.a"
HEADER_FILE_NAME="$LIBRARY_NAME.h"

TARGETS="x86_64-unknown-linux-gnu aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios aarch64-apple-darwin x86_64-apple-darwin"

# Building for the desired targets
echo "Building the library";
for target in $TARGETS
do
    echo "Building for target '$target'"

    cargo build \
        --target $target \
        --target-dir ./target \
        --release
done

# Creating a C-header and copying it to the directory of all of our build targets
echo "Generating the C Header"
rustup default nightly
cbindgen \
    --lang c \
    --config cbindgen.toml \
    --crate radix-engine-toolkit-native \
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
        
    mv aarch64-apple-ios/release/$LIBRARY_FILE_NAME aarch64-apple-ios/release/$LIBRARY_FILE_NAME

	echo "🔮 🙏 Finished merging some of the targets using 'lipo'"
)

echo "Creating the include directory"
mkdir include
mv $HEADER_FILE_NAME include
echo echo $MODULE_MAP_CONTENTS > ./include/module.modulemap

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