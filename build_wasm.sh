cd radix-engine-toolkit
cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target wasm32-unknown-unknown \
    --release
npx wasm-opt@1.4 -Oz -g \
    --strip-dwarf \
    --strip-debug \
    --strip-producers \
    -o \
    ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm \
    ./target/wasm32-unknown-unknown/release/radix_engine_toolkit.wasm