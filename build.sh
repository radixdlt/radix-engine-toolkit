echo "Building the library";

# WebAssembly targets
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release

# iOS Targets
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-ios-sim \
    --release       # iOS Simulator Aarch64 target
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-ios \
    --release        # iOS iPhone Aarch64 target

# PC Targets
cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-darwin \
    --release

(
    cd ./target
    mkdir iOS
    mkdir iOS/simulator
    mkdir iOS/iPhone
)

# Create the C header of the provided functions and adding it to the directory of each of the 
# builds
(
    rustup default nightly

    # Creating the header file
    cbindgen \
        --lang c \
        --config cbindgen.toml \
        --crate radix-engine-toolkit \
        --output radix_engine_toolkit.h

    # Copying the header file to all of the builds
    cp radix_engine_toolkit.h ./target/wasm32-unknown-unknown/release/
    cp radix_engine_toolkit.h ./target/aarch64-apple-ios-sim/release/
    cp radix_engine_toolkit.h ./target/aarch64-apple-ios/release/
    
    cp radix_engine_toolkit.h ./target/iOS/iPhone
    cp radix_engine_toolkit.h ./target/iOS/simulator
    
    # The root-level header is no longer needed now. Safe to delete
    rm radix_engine_toolkit.h

    rustup default stable
)

# Creating an XC Framework of the static libraries. 
xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/release/libradix_engine_toolkit.a \
    -headers ./target/aarch64-apple-ios/release/radix_engine_toolkit.h \
    -library ./target/aarch64-apple-ios-sim/release/libradix_engine_toolkit.a \
    -headers ./target/aarch64-apple-ios-sim/release/radix_engine_toolkit.h \
    -output ./target/iOS/universal.xcframework

wasm-opt \
    -Os -g \
    --strip-debug --strip-dwarf --strip-producers \
    -o ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm \
    ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm

echo "Done!"