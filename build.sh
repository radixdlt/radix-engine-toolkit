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
        x86_64-apple-ios/release/libtransaction_library.a \
        aarch64-apple-ios-sim/release/libtransaction_library.a \
        -o iOS/simulator/libtransaction_library.a
    
    # Combine the two iPhone builds into one fat file
    lipo -create \
        x86_64-apple-ios/release/libtransaction_library.a \
        aarch64-apple-ios/release/libtransaction_library.a \
        -o iOS/iPhone/libtransaction_library.a
)

# Create the C header of the provided functions and adding it to the directory of each of the 
# builds
(
    rustup default nightly

    # Creating the header file
    cbindgen \
        --lang c \
        --config cbindgen.toml \
        --crate transaction-library \
        --output transaction_library.h

    # Copying the header file to all of the builds
    cp transaction_library.h ./target/wasm32-unknown-unknown/release/
    cp transaction_library.h ./target/x86_64-apple-ios/release/
    cp transaction_library.h ./target/aarch64-apple-ios-sim/release/
    cp transaction_library.h ./target/aarch64-apple-ios/release/
    
    cp transaction_library.h ./target/iOS/iPhone
    cp transaction_library.h ./target/iOS/simulator
    
    # The root-level header is no longer needed now. Safe to delete
    rm transaction_library.h

    rustup default stable
)

# Creating an XC Framework of the static libraries. Note: at the current moment of time, I have
# removed the `x86_64-apple-ios` target from the XC Framework as it clashed with 
# `ios-arm64-simulator`. If this causes issues, then we can look into it further. 
xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/release/libtransaction_library.a \
    -headers ./target/aarch64-apple-ios/release/transaction_library.h \
    -library ./target/aarch64-apple-ios-sim/release/libtransaction_library.a \
    -headers ./target/aarch64-apple-ios-sim/release/transaction_library.h \
    -output ./target/iOS/universal.xcframework

echo "Done!"