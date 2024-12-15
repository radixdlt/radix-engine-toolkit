SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo +nightly-2024-12-14 build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --manifest-path="$SCRIPT_DIR/../Cargo.toml" \
    --package radix-engine-toolkit-json \
    --target wasm32-unknown-unknown \
    --release
npx wasm-opt@1.4 -Oz -g \
    --strip-dwarf \
    --strip-debug \
    --strip-producers \
    -o \
    "$SCRIPT_DIR/../target/wasm32-unknown-unknown/release/radix_engine_toolkit_json.wasm" \
    "$SCRIPT_DIR/../target/wasm32-unknown-unknown/release/radix_engine_toolkit_json.wasm"