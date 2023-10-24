bindgen() {
    cargo run \
        --manifest-path="../uniffi-bindgen/Cargo.toml" -- \
        generate ./src/radix_engine_toolkit_uniffi.udl \
        --language $1 \
        --out-dir ./output \
        --lib-file ./target/debug/libradix_engine_toolkit_uniffi.a
}

cd radix-engine-toolkit-uniffi;
cargo build

bindgen swift
bindgen kotlin
bindgen python

uniffi-bindgen-cs src/radix_engine_toolkit_uniffi.udl --lib-file ./target/debug/libradix_engine_toolkit_uniffi.a --out-dir output