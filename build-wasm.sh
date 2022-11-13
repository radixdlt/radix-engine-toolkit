echo "Building the library";

cargo +nightly \
    build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release

echo "Using wasm-opt to compress the WASM"

wasm-opt \
    -Oz -g \
    --strip-debug --strip-dwarf --strip-producers \
    -o ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm \
    ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm