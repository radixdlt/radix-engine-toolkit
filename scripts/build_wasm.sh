SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --manifest-path="$SCRIPT_DIR/../crates/radix-engine-toolkit/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release
npx wasm-opt@1.4 -Oz -g \
    --strip-dwarf \
    --strip-debug \
    --strip-producers \
    -o \
    "$SCRIPT_DIR/../target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm" \
    "$SCRIPT_DIR/../target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm"