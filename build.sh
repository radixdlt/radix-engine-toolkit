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

    # Combine the two simulator builds into one universal build
    lipo -create x86_64-apple-ios/release/libtransaction_library.a aarch64-apple-ios-sim/release/libtransaction_library.a -o iOS/simulator/libtransaction_library.a
    
    # Combine the two iPhone builds into one universal build
    lipo -create x86_64-apple-ios/release/libtransaction_library.a aarch64-apple-ios/release/libtransaction_library.a -o iOS/iPhone/libtransaction_library.a
)

echo "Done!"