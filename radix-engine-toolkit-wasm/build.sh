echo "Building the library";

cargo \
    build \
    --target wasm32-unknown-unknown \
    --release

echo "Using wasm-opt to compress the WASM"

wasm-opt \
    -Oz -g \
    --strip-debug --strip-dwarf --strip-producers \
    -o ./target/wasm32-unknown-unknown/release/radix_engine_toolkit_wasm.wasm \
    ./target/wasm32-unknown-unknown/release/radix_engine_toolkit_wasm.wasm

echo "WASM build completed"