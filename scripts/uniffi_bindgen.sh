SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Detect OS and set library extension
case "$(uname -s)" in
    Darwin*)
        LIB_EXT="dylib"
        ;;
    Linux*)
        LIB_EXT="so"
        ;;
    CYGWIN*|MINGW*|MSYS*)
        LIB_EXT="dll"
        ;;
    *)
        LIB_EXT="so"
        ;;
esac

LIB_FILE="$SCRIPT_DIR/../target/debug/libradix_engine_toolkit_uniffi.$LIB_EXT"

bindgen() {
    cargo run \
        --manifest-path="$SCRIPT_DIR/../crates/uniffi-bindgen/Cargo.toml" -- \
        generate $SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/src/radix_engine_toolkit_uniffi.udl \
        --language $1 \
        --no-format \
        --out-dir ./output \
        --lib-file "$LIB_FILE"
}

bindgen_ext_tool() {
    $1 \
    "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/src/radix_engine_toolkit_uniffi.udl" \
    --lib-file "$LIB_FILE" --out-dir output \
    --no-format \
    --config "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/uniffi.toml"
}

cd $SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi;
cargo build

# Generate bindings - continue even if some fail
bindgen swift || echo "Swift binding generation failed"
bindgen kotlin || echo "Kotlin binding generation failed"
bindgen python || echo "Python binding generation failed"

# Generate Go bindings
echo ""
echo "Generating Go bindings..."
if command -v uniffi-bindgen-go &> /dev/null; then
    bindgen_ext_tool uniffi-bindgen-go || echo "Go binding generation failed"
elif [ -d "$SCRIPT_DIR/../uniffi-bindgen-go" ]; then
    echo "Using local uniffi-bindgen-go clone..."
    cargo run \
        --manifest-path="$SCRIPT_DIR/../uniffi-bindgen-go/bindgen/Cargo.toml" -- \
        "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/src/radix_engine_toolkit_uniffi.udl" \
        --lib-file "$LIB_FILE" \
        --out-dir output \
        --config "$SCRIPT_DIR/../crates/radix-engine-toolkit-uniffi/uniffi.toml"
else
    echo "ERROR: uniffi-bindgen-go not found!"
    echo "Please either:"
    echo "  1. Install uniffi-bindgen-go: cargo install uniffi-bindgen-go"
    echo "  2. Or clone it to the project root:"
    echo "     git clone https://github.com/NordSecurity/uniffi-bindgen-go.git"
    exit 1
fi

echo ""
echo "Binding generation complete!"
