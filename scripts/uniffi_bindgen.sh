SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

bindgen() {
    cargo run \
        --manifest-path="$SCRIPT_DIR/../crates/uniffi-bindgen/Cargo.toml" -- \
        generate $SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/src/radix_engine_toolkit_uniffi.udl \
        --language $1 \
        --no-format \
        --out-dir ./output \
        --lib-file $SCRIPT_DIR/../target/debug/libradix_engine_toolkit_uniffi.dylib
}

bindgen_ext_tool() {
    $1 \
    "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/src/radix_engine_toolkit_uniffi.udl" \
    --lib-file "$SCRIPT_DIR/../target/debug/libradix_engine_toolkit_uniffi.dylib" --out-dir output \
    --no-format \
    --config "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/uniffi.toml"
}

cd $SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi;
cargo build

bindgen swift
bindgen kotlin
bindgen python

bindgen_ext_tool uniffi-bindgen-cs
bindgen_ext_tool uniffi-bindgen-go
