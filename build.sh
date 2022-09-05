echo "Generating the OpenAPI Spec";
python3 ./spec/generate_spec.py

echo "Building the library";
cargo build --target wasm32-unknown-unknown --release

echo "Done!"