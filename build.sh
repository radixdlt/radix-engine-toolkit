echo "Building the library";
cargo build --target wasm32-unknown-unknown --release

echo "Copying the WASM modules to the example directories"
cp ./target/wasm32-unknown-unknown/release/transaction_library.wasm ./examples/dotnet/wasm/
cp ./target/wasm32-unknown-unknown/release/transaction_library.wasm ./examples/typescript/wasm/

echo "Generating the OpenAPI models for the examples";
python3 ./scripts/generate-openapi-models.py

echo "Done!"