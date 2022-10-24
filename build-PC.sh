echo "Building the library";

cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target aarch64-apple-darwin \
    --features jni \
    --release