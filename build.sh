echo "Building the library";
cargo build --target wasm32-unknown-unknown --release

cargo build --target x86_64-apple-ios --release         # iOS Simulator & iPhone x86 target
cargo build --target aarch64-apple-ios-sim --release    # iOS Simulator Aarch64 target
cargo build --target aarch64-apple-ios --release        # iOS iPhone Aarch64 target

(
    cd ./target
    mkdir iOS
    mkdir iOS/simulator
    mkdir iOS/iPhone

    # Combine the two simulator builds into one fat file
    lipo -create \
        x86_64-apple-ios/release/libradix_engine_toolkit.a \
        aarch64-apple-ios-sim/release/libradix_engine_toolkit.a \
        -o iOS/simulator/libradix_engine_toolkit.a
    
    # Combine the two iPhone builds into one fat file
    lipo -create \
        x86_64-apple-ios/release/libradix_engine_toolkit.a \
        aarch64-apple-ios/release/libradix_engine_toolkit.a \
        -o iOS/iPhone/libradix_engine_toolkit.a
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
    cp radix_engine_toolkit.h ./target/x86_64-apple-ios/release/
    cp radix_engine_toolkit.h ./target/aarch64-apple-ios-sim/release/
    cp radix_engine_toolkit.h ./target/aarch64-apple-ios/release/
    
    cp radix_engine_toolkit.h ./target/iOS/iPhone
    cp radix_engine_toolkit.h ./target/iOS/simulator
    
    # The root-level header is no longer needed now. Safe to delete
    rm radix_engine_toolkit.h

    rustup default stable
)

# Creating an XC Framework of the static libraries. Note: at the current moment of time, I have
# removed the `x86_64-apple-ios` target from the XC Framework as it clashed with 
# `ios-arm64-simulator`. If this causes issues, then we can look into it further. 
xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/release/libradix_engine_toolkit.a \
    -headers ./target/aarch64-apple-ios/release/radix_engine_toolkit.h \
    -library ./target/aarch64-apple-ios-sim/release/libradix_engine_toolkit.a \
    -headers ./target/aarch64-apple-ios-sim/release/radix_engine_toolkit.h \
    -output ./target/iOS/universal.xcframework

echo "Done!"